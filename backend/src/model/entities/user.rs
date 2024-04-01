//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(column_name = "User_Id", primary_key, auto_increment = false)]
    pub user_id: i32,
    #[sea_orm(column_name = "User_name", column_type = "Text", nullable)]
    pub user_name: Option<String>,
    #[sea_orm(column_name = "Current_msg", column_type = "Text", nullable)]
    pub current_msg: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
