#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

// #[get("/")]
// fn index() -> Template {
//     Template::render("index")
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("templates")))
}