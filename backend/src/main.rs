#[macro_use] extern crate rocket;

mod web;
mod models;
mod api;

use api::v1;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api/v1", v1::routes())
        .mount("/", web::routes())
}
