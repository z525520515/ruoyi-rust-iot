use std::cell::RefCell;

use crate::modules::system::domain::vo::UserCache;

//线程安全的变量，方便会话调用
thread_local!(static USERNAME: RefCell<String> = RefCell::new("".to_string()));
//当前登录用户缓存（包含部门、角色、数据权限等），用于数据权限过滤
thread_local!(static USER_CACHE: RefCell<Option<UserCache>> = RefCell::new(None));

pub fn set_user_name(user_name: String) {
    USERNAME.with(|f| {
        *f.borrow_mut() = user_name;
    });
}

pub fn get_user_name() -> String {
    // we retain our original value of 2 despite the child thread
    let mut user_name = "".to_string();
    USERNAME.with(|f| {
        user_name = f.borrow().to_string()
    });
    user_name
}

/// 保存当前请求的登录用户信息（部门 / 角色 / 权限），供数据权限过滤使用
pub fn set_user_cache(cache: UserCache) {
    USERNAME.with(|f| {
        *f.borrow_mut() = cache.user_name.clone();
    });
    USER_CACHE.with(|f| {
        *f.borrow_mut() = Some(cache);
    });
}

/// 获取当前请求的登录用户信息
pub fn get_user_cache() -> Option<UserCache> {
    USER_CACHE.with(|f| f.borrow().clone())
}
