#[macro_use] extern crate rocket;
use serde::{Serialize,Deserialize};
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
mod models;
use crate::models::*;
use diesel::prelude::*;

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

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

#[openapi]
#[get("/")]
fn index() -> Json<EchoResponse> {
    Json(EchoResponse {
        message: "Hello World".to_string(),
    })
}

#[openapi]
#[post("/user", format = "json", data = "<input>")]
fn create_user(input:Json<CreateUserInput>) -> Result<Json<()>>{
    let connection = &mut establish_connection();
    let user = User {
        name: input.name.clone(),
        email: input.email.clone(),
    };
    diesel::insert_into(users)
        .values(&user)
        .execute(connection);
    
    Ok(rocket::serde::json::Json(()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![index,create_user])
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/openapi.json".to_owned(),
                ..Default::default()
            })
        )
}
