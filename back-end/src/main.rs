#[macro_use] extern crate rocket;
use serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
mod models;
mod schema;
mod services;
use crate::models::*;
use diesel::prelude::*;
use std::fmt::Debug;
use crate::schema::users::dsl::users;
use crate::schema::items::dsl::items;
use crate::schema::currency::dsl::currency;
use crate::schema::users::id;
use crate::schema::inventory;       
use crate::schema::wallet;        
use crate::services::*;


#[derive(Serialize,Deserialize,  JsonSchema)]
struct EchoResponse {
    message: String,
}

#[derive(Serialize,Deserialize, JsonSchema)]
struct CreateUserResponse{
    id : i32,
    name: String,
    email: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct CreateUserInput {    
    name: String,
    email: String,
}


#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct CreateItemInput {    
    name: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct UpdateItemInventory {
    item_id : i32,   
    quantity: i32,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct CreateCurrencyInput {    
    name: String,
}

#[derive(Deserialize, JsonSchema)]
#[serde(crate = "rocket::serde")]
struct UpdateCurrencyWallet {
    currency_id : i32,   
    quantity: i32,
}

// ---------------------

#[openapi]
#[get("/")]
fn index() -> Json<EchoResponse> {
    Json(EchoResponse {
        message: "Hello World".to_string(),
    })
}


/*
================================================================================================================
========================================================================================================================
========================================================================================================================================
===================================USER=============================================================================================================
================================================================================================================================V
========================================================================================================================
*/

#[openapi]
#[post("/user", format = "json", data = "<input>")]
fn create_user(input:Json<CreateUserInput>) -> Option<Json<()>>{
    let connection = &mut establish_connection();
    let user = UserInsert {
        name: input.name.clone(),
        email: input.email.clone(),
    };
    diesel::insert_into(users)
        .values(&user)
        .execute(connection);
    
    Some(rocket::serde::json::Json(()))
}

#[openapi]
#[get("/user")]
fn get_users() -> Option<Json<Vec<User>>>{
    let connection = &mut establish_connection();
    Some(rocket::serde::json::Json(users.load::<User>(connection).ok()?))
}

#[openapi]
#[get("/user/<user_id>")]
fn get_user(user_id : i32) -> Option<Json<Vec<User>>>{
    let connection = &mut establish_connection();   
    Some(rocket::serde::json::Json(users.filter(id.eq(user_id)).load::<User>(connection).ok()?))
}

/*
================================================================================================================
========================================================================================================================
========================================================================================================================================
================================================================================================================================================
=================================ITEM===============================================================================================V
========================================================================================================================
*/
#[openapi]
#[post("/item", format = "json", data = "<input>")]
fn create_item(input:Json<CreateItemInput>) -> Option<Json<()>>{
    let connection = &mut establish_connection();
    let item = ItemInsert {
        name: input.name.clone(),
    };    
    diesel::insert_into(items)
        .values(&item)
        .execute(connection);
    
    Some(rocket::serde::json::Json(()))
}

#[openapi]
#[get("/item")]
fn get_items() -> Option<Json<Vec<Item>>>{
    let connection = &mut establish_connection();
    Some(rocket::serde::json::Json(items.load::<Item>(connection).ok()?))
}

/*
================================================================================================================
========================================================================================================================
========================================================================================================================================
================================================================================================================================================
=================================INVENTORY===============================================================================================V
========================================================================================================================
*/

// PATCH 
#[openapi]
#[patch("/inventory/<id_user>", format = "json", data = "<input>")]
fn modify_amount_inventory(id_user : i32,input:Json<UpdateItemInventory>) -> Option<Json<()>>{
    let connection = &mut establish_connection();

    let exist = inventory::dsl::inventory
                 .filter(inventory::user_id.eq(id_user))
                 .filter(inventory::item_id.eq(input.item_id))
                 .count()
                 .execute(connection).ok()?;

    if  exist <= 0{

        let create_inventory_slot = Inventory {
            user_id: id_user,
            item_id: input.item_id,
            quantity: 0,
        };

        diesel::insert_into(inventory::dsl::inventory)
                .values(&create_inventory_slot)
                .execute(connection);
    }

    diesel::update(inventory::dsl::inventory)
    .filter(inventory::user_id.eq(id_user))
    .filter(inventory::item_id.eq(input.item_id))
    .set(inventory::quantity.eq(inventory::quantity + input.quantity))
    .execute(connection).ok()?;

    Some(rocket::serde::json::Json(()))
}

// GET inventory pour 1 user_id
#[openapi]
#[get("/inventory/<id_user>")]
fn get_inventory(id_user : i32) -> Option<Json<Vec<Inventory>>>{
    let connection = &mut establish_connection();   
    Some(rocket::serde::json::Json(inventory::dsl::inventory.filter(inventory::user_id.eq(id_user)).load::<Inventory>(connection).ok()?))
}
/*
================================================================================================================
========================================================================================================================
========================================================================================================================================
================================================================================================================================================
=================================Currency===============================================================================================V
========================================================================================================================
*/

#[openapi]
#[post("/currency", format = "json", data = "<input>")]
fn create_currency(input:Json<CreateCurrencyInput>) -> Option<Json<()>>{
    let connection = &mut establish_connection();
    let curr = CurrencyInsert {
        name: input.name.clone(),
    };    
    diesel::insert_into(currency)
        .values(&curr)
        .execute(connection);
    
    Some(rocket::serde::json::Json(()))
}

#[openapi]
#[get("/currency")]
fn get_currencies() -> Option<Json<Vec<Currency>>>{
    let connection = &mut establish_connection();
    Some(rocket::serde::json::Json(currency.load::<Currency>(connection).ok()?))
}


/*
================================================================================================================
========================================================================================================================
========================================================================================================================================
================================================================================================================================================
=================================Wallet===============================================================================================V
========================================================================================================================
*/

#[openapi]
#[patch("/wallet/<id_user>", format = "json", data = "<input>")]
fn modify_amount_wallet(id_user : i32,input:Json<UpdateCurrencyWallet>) -> Option<Json<()>>{
    let connection = &mut establish_connection();

    let exist = wallet::dsl::wallet
                 .filter(wallet::user_id.eq(id_user))
                 .filter(wallet::currency_id.eq(input.currency_id))
                 .count()
                 .execute(connection).ok()?;

    if  exist <= 0{

        let create_wallet_slot = Wallet {
            user_id: id_user,
            currency_id: input.currency_id,
            quantity: 0,
        };

        diesel::insert_into(wallet::dsl::wallet)
                .values(&create_wallet_slot)
                .execute(connection);
    }

    diesel::update(wallet::dsl::wallet)
    .filter(wallet::user_id.eq(id_user))
    .filter(wallet::currency_id.eq(input.currency_id))
    .set(wallet::quantity.eq(wallet::quantity + input.quantity))
    .execute(connection).ok()?;

    Some(rocket::serde::json::Json(()))
}

#[openapi]
#[get("/wallet/<id_user>")]
fn get_wallet(id_user : i32) -> Option<Json<Vec<Wallet>>>{
    let connection = &mut establish_connection();   
    Some(rocket::serde::json::Json(wallet::dsl::wallet.filter(wallet::user_id.eq(id_user)).load::<Wallet>(connection).ok()?))
}

/*
================================================================================================================
========================================================================================================================
========================================================================================================================================
================================================================================================================================================
=================================LUNCH===============================================================================================V
========================================================================================================================
*/
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![index,
                                        create_user,get_users,get_user,
                                        create_item,get_items,
                                        modify_amount_inventory,get_inventory,
                                        create_currency,get_currencies,
                                        modify_amount_wallet,get_wallet])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/openapi.json".to_owned(),
                ..Default::default()
            })
        )
}
