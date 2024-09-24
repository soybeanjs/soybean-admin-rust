//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_login_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub user_id: String,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub domain: String,
    pub login_time: DateTime,
    #[sea_orm(column_type = "Text")]
    pub ip: String,
    pub port: Option<i32>,
    #[sea_orm(column_type = "Text")]
    pub address: String,
    #[sea_orm(column_type = "Text")]
    pub user_agent: String,
    #[sea_orm(column_type = "Text")]
    pub request_id: String,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    pub created_at: DateTime,
    #[sea_orm(column_type = "Text")]
    pub created_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
