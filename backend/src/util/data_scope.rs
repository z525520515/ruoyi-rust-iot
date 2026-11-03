use std::collections::HashMap;

use crate::config::global_variables::ADMIN_NAME;
use crate::error::{Error, Result};
use crate::modules::system::domain::table::SysDept;
use crate::pool;
use rbs::value;

/// 当前登录用户的数据权限范围
#[derive(Debug, Clone)]
pub enum DataScope {
    /// 全部数据（admin 或角色配置为“全部数据权限”）
    All,
    /// 指定部门（含下级部门）
    DeptIds(Vec<i64>),
    /// 仅本人创建的数据（设备台账使用）
    SelfOnly(String),
}

/// sys_dept.dept_id 为 varchar，只有数字型部门 ID 才能用于设备表的 dept_id(int8)
pub fn parse_dept_id(s: &str) -> Option<i64> {
    let t = s.trim();
    if t.is_empty() {
        return None;
    }
    t.parse::<i64>().ok()
}

/// 查询某部门及其全部下级部门的数字 ID
pub async fn dept_and_children(dept_id: i64) -> Result<Vec<i64>> {
    let target = dept_id.to_string();
    let depts: Vec<SysDept> = SysDept::select_all(pool!()).await?;
    let mut ids: Vec<i64> = Vec::new();
    for d in depts {
        let id = d.dept_id.unwrap_or_default();
        let in_subtree = id == target
            || d.ancestors
                .as_deref()
                .map(|anc| anc.split(',').any(|p| p.trim() == target))
                .unwrap_or(false);
        if in_subtree {
            if let Some(v) = parse_dept_id(&id) {
                if !ids.contains(&v) {
                    ids.push(v);
                }
            }
        }
    }
    Ok(ids)
}

/// 部门 ID -> 部门名称
pub async fn dept_name_map() -> Result<HashMap<String, String>> {
    let depts: Vec<SysDept> = SysDept::select_all(pool!()).await?;
    let mut map = HashMap::new();
    for d in depts {
        if let (Some(id), Some(name)) = (d.dept_id, d.dept_name) {
            map.insert(id, name);
        }
    }
    Ok(map)
}

/// 当前登录用户的数据权限
pub async fn current_scope() -> Result<DataScope> {
    let cache = match crate::web_data::get_user_cache() {
        Some(c) => c,
        None => return Ok(DataScope::All),
    };
    if cache.user_name == ADMIN_NAME {
        return Ok(DataScope::All);
    }
    let scopes: Vec<char> = cache.roles.iter().filter_map(|r| r.data_scope).collect();
    if scopes.is_empty() || scopes.contains(&'1') {
        return Ok(DataScope::All);
    }
    let dept_id = cache.user.as_ref().and_then(|u| u.dept_id.clone());
    let dept_id = match dept_id {
        Some(d) => d,
        None => return Ok(DataScope::DeptIds(Vec::new())),
    };
    if scopes.contains(&'5') && !scopes.iter().any(|s| *s == '2' || *s == '3' || *s == '4') {
        return Ok(DataScope::SelfOnly(cache.user_name.clone()));
    }
    match parse_dept_id(&dept_id) {
        // 2(自定义) / 3(本部门) / 4(本部门及以下) 统一按“本部门及以下”过滤；
        // 如需精确的自定义部门范围，可在此基础上结合 sys_role_dept 扩展
        Some(id) => Ok(DataScope::DeptIds(dept_and_children(id).await?)),
        None => Ok(DataScope::DeptIds(Vec::new())),
    }
}

/// 追加 `and <column> in (?, ?, ...)`
pub fn push_in_clause(sql: &mut String, args: &mut Vec<rbs::Value>, column: &str, ids: &[i64]) {
    if ids.is_empty() {
        sql.push_str(" and 1=0");
        return;
    }
    let marks = vec!["?"; ids.len()].join(",");
    sql.push_str(&format!(" and {} in ({})", column, marks));
    for id in ids {
        args.push(value!(*id));
    }
}

/// 追加 `and <column> like ?`
pub fn push_like(
    sql: &mut String,
    args: &mut Vec<rbs::Value>,
    column: &str,
    keyword: &Option<String>,
) {
    if let Some(raw) = keyword.as_deref() {
        let v = raw.trim();
        if !v.is_empty() {
            sql.push_str(&format!(" and {} like ?", column));
            args.push(value!(format!("%{}%", v)));
        }
    }
}

/// 追加 `and <column> = ?`（字符串）
pub fn push_eq_str(
    sql: &mut String,
    args: &mut Vec<rbs::Value>,
    column: &str,
    v: &Option<String>,
) {
    if let Some(raw) = v.as_deref() {
        let v = raw.trim();
        if !v.is_empty() {
            sql.push_str(&format!(" and {} = ?", column));
            args.push(value!(v.to_string()));
        }
    }
}

/// 追加 `and <column> = ?`（整数）
pub fn push_eq_i32(
    sql: &mut String,
    args: &mut Vec<rbs::Value>,
    column: &str,
    v: &Option<i32>,
) {
    if let Some(v) = v {
        sql.push_str(&format!(" and {} = ?", column));
        args.push(value!(*v));
    }
}

/// 设备台账数据权限：支持“仅本人”（按 create_by 过滤）
pub async fn apply_scope(
    sql: &mut String,
    args: &mut Vec<rbs::Value>,
    dept_column: &str,
    create_by_column: &str,
) -> Result<()> {
    match current_scope().await? {
        DataScope::All => {}
        DataScope::DeptIds(ids) => push_in_clause(sql, args, dept_column, &ids),
        DataScope::SelfOnly(user_name) => {
            sql.push_str(&format!(" and {} = ?", create_by_column));
            args.push(value!(user_name));
        }
    }
    Ok(())
}

/// 设备数据数据权限：表里没有 create_by，“仅本人”退化为本部门
pub async fn apply_scope_dept_only(
    sql: &mut String,
    args: &mut Vec<rbs::Value>,
    dept_column: &str,
) -> Result<()> {
    match current_scope().await? {
        DataScope::All => {}
        DataScope::DeptIds(ids) => push_in_clause(sql, args, dept_column, &ids),
        DataScope::SelfOnly(_) => {
            let user_dept = crate::web_data::get_user_cache()
                .and_then(|c| c.user.and_then(|u| u.dept_id));
            match user_dept.and_then(|d| parse_dept_id(&d)) {
                Some(dept_id) => {
                    let ids = dept_and_children(dept_id).await?;
                    push_in_clause(sql, args, dept_column, &ids);
                }
                None => sql.push_str(" and 1=0"),
            }
        }
    }
    Ok(())
}

/// 单条记录的数据权限校验
pub async fn check_row_scope(dept_id: Option<i64>, create_by: Option<&str>) -> Result<()> {
    match current_scope().await? {
        DataScope::All => Ok(()),
        DataScope::DeptIds(ids) => {
            if dept_id.map(|d| ids.contains(&d)).unwrap_or(false) {
                Ok(())
            } else {
                Err(Error::from("无权访问该数据"))
            }
        }
        DataScope::SelfOnly(user_name) => {
            if create_by.unwrap_or_default() == user_name {
                Ok(())
            } else {
                Err(Error::from("无权访问该数据"))
            }
        }
    }
}

/// 单条记录的数据权限校验（仅按部门判断，用于没有 create_by 的设备数据）
pub async fn check_dept_scope(row_dept_id: Option<i64>) -> Result<()> {
    match current_scope().await? {
        DataScope::All => Ok(()),
        DataScope::DeptIds(ids) => {
            if row_dept_id.map(|d| ids.contains(&d)).unwrap_or(false) {
                Ok(())
            } else {
                Err(Error::from("无权访问该数据"))
            }
        }
        DataScope::SelfOnly(_) => {
            let user_dept = crate::web_data::get_user_cache()
                .and_then(|c| c.user.and_then(|u| u.dept_id))
                .and_then(|d| parse_dept_id(&d));
            match user_dept {
                Some(root_dept_id) => {
                    let ids = dept_and_children(root_dept_id).await?;
                    if row_dept_id.map(|d| ids.contains(&d)).unwrap_or(false) {
                        Ok(())
                    } else {
                        Err(Error::from("无权访问该数据"))
                    }
                }
                None => Err(Error::from("无权访问该数据")),
            }
        }
    }
}
