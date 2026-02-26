use anyhow::Result;
use sea_orm::{Statement, entity::prelude::*};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "equipment_info")] // 一次性把驼峰映射回蛇底
pub struct Model {
    /// 主键，随机 UUID（v4）
    #[sea_orm(primary_key, auto_generate = false)]
    pub id: Uuid,

    /// 设备编号（NOT NULL + UNIQUE）
    pub equipment_number: String,

    /// 项目编号（可空）
    pub project_num: Option<String>,

    /// 设备名称
    pub equipment_name: Option<String>,

    /// 设备型号
    pub equipment_model: Option<String>,

    /// 设备型号中文标签
    pub equipment_model_label: Option<String>,

    /// 设备类型（4 位字典值）
    pub equipment_type: Option<String>,

    /// 设备状态（10 位长度）
    pub equipment_status: Option<String>,

    /// 厂家名称
    pub manufacturer_name: Option<String>,

    /// 厂家字典值
    pub dict_value: Option<String>,

    /// 通讯方式
    pub communication_means: Option<String>,

    /// 通讯地址
    pub addr: Option<String>,

    /// 建筑/位置 ID
    pub location_id: Option<String>,

    /// 安装图（文件路径或 URL）
    pub installation_drawing: Option<String>,

    /// 描述
    pub remark: Option<String>,
    // IP
    pub ip: Option<String>,
    //SIM
    pub sim: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn build(db: &DatabaseConnection) -> Result<()> {
    let sql = r#"
        CREATE TABLE IF NOT EXISTS equipment_info (
    id UUID PRIMARY KEY,
    equipment_number    VARCHAR(50)  NOT NULL,
    project_num         VARCHAR(20),
    equipment_name      VARCHAR(50),
    equipment_model     VARCHAR(50),
    equipment_model_label VARCHAR(255),
    equipment_type      VARCHAR(4),
    equipment_status    VARCHAR(10),
    manufacturer_name   VARCHAR(50),
    dict_value          VARCHAR(255),
    communication_means VARCHAR(255),
    addr                VARCHAR(255),
    location_id         VARCHAR(50),
    installation_drawing VARCHAR(255),
    remark              VARCHAR(255)
     ip                 VARCHAR(20),
    sim              VARCHAR(20)
);
    "#;

    // 根据当前后端拼一条通用 Statement
    let stmt = Statement::from_string(db.get_database_backend(), sql.to_owned());
    db.execute(stmt).await?;
    Ok(())
}
