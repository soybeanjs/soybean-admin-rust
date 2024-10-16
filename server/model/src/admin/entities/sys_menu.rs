//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::Serialize;

use super::sea_orm_active_enums::{MenuType, Status};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "sys_menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub menu_type: MenuType,
    pub menu_name: String,
    pub icon_type: Option<i32>,
    pub icon: Option<String>,
    #[sea_orm(unique)]
    pub route_name: String,
    pub route_path: String,
    pub component: String,
    pub path_param: Option<String>,
    pub status: Status,
    pub active_menu: Option<String>,
    pub hide_in_menu: Option<bool>,
    #[sea_orm(column_type = "Text")]
    pub pid: String,
    pub sequence: i32,
    pub i18n_key: Option<String>,
    pub keep_alive: Option<bool>,
    pub constant: bool,
    pub href: Option<String>,
    pub multi_tab: Option<bool>,
    pub created_at: DateTime,
    #[sea_orm(column_type = "Text")]
    pub created_by: String,
    pub updated_at: Option<DateTime>,
    #[sea_orm(column_type = "Text", nullable)]
    pub updated_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
