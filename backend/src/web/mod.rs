use rocket::fs::{FileServer, relative};
use rocket::Route;

pub fn routes() -> Vec<Route> {
    FileServer::from(relative!("static")).into()
}
