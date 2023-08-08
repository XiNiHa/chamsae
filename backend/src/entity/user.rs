//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub created_at: TimeDateTimeWithTimeZone,
    pub updated_at: Option<TimeDateTimeWithTimeZone>,
    pub last_fetched_at: Option<TimeDateTimeWithTimeZone>,
    pub handle: String,
    pub name: String,
    pub follower_count: i32,
    pub following_count: i32,
    pub post_count: i32,
    pub avatar_id: Option<String>,
    pub banner_id: Option<String>,
    pub is_bot: bool,
    pub host: String,
    pub inbox: String,
    pub shared_inbox: String,
    pub uri: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::AvatarId",
        to = "super::file::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    File2,
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::BannerId",
        to = "super::file::Column::Id",
        on_update = "NoAction",
        on_delete = "SetNull"
    )]
    File1,
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
