use super::schema::{users,items,inventory,currency,wallet};
use serde::{Serialize,Deserialize};
use diesel::prelude::*;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

// -------------- User

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

// -------------- Item

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

// -----------------  Inventory

#[derive(Queryable,Insertable,Selectable,Serialize,Deserialize,JsonSchema)]
#[diesel(table_name = inventory)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Inventory {
    pub user_id: i32,
    pub item_id: i32,
    pub quantity: i32,
}

// ----------------- Currency

#[derive(Queryable, Selectable,Serialize,Deserialize,JsonSchema,diesel::Identifiable)]
#[diesel(table_name = currency)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Currency {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable,Serialize,Deserialize,JsonSchema)]
#[diesel(table_name = currency)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CurrencyInsert {
    pub name: String,
}

// ----------------- Wallet

#[derive(Queryable,Insertable,Selectable,Serialize,Deserialize,JsonSchema)]
#[diesel(table_name = wallet)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Wallet {
    pub user_id: i32,
    pub currency_id: i32,
    pub quantity: i32,
}