//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "sys_operation_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub user_id: String,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub domain: String,
    #[sea_orm(column_type = "Text")]
    pub module_name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
    #[sea_orm(column_type = "Text")]
    pub request_id: String,
    #[sea_orm(column_type = "Text")]
    pub method: String,
    #[sea_orm(column_type = "Text")]
    pub url: String,
    #[sea_orm(column_type = "Text")]
    pub ip: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub user_agent: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub params: Option<JsonValue>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub body: Option<JsonValue>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub response: Option<JsonValue>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub duration: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
