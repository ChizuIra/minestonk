use super::schema::{users,items,inventory};
use serde::{Serialize,Deserialize};
use diesel::prelude::*;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Queryable, Selectable,Serialize,Deserialize,JsonSchema,diesel::Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable,Serialize,Deserialize,JsonSchema)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserInsert {
    pub name: String,
    pub email: String,
}

// --------------

#[derive(Queryable, Selectable,Serialize,Deserialize,JsonSchema,diesel::Identifiable)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Item {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable,Serialize,Deserialize,JsonSchema)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ItemInsert {
    pub name: String,
}

// ----------------- 

#[derive(Queryable,Insertable,Selectable,Serialize,Deserialize,JsonSchema)]
#[diesel(table_name = inventory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inventory {
    pub user_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}