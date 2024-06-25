//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub uuid: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub username: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub password: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub nick_name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub side_mode: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub header_img: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub base_color: Option<String>,
    pub authority_id: Option<i64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub phone: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub email: Option<String>,
    pub enable: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
