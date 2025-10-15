#[macro_use] extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct HelloResponse {
    message: String,
    name: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> Json<HelloResponse> {
    Json(HelloResponse {
        message: format!("{}, now with extra Iron Oxide!", name),
        name: name.to_string(),
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}

