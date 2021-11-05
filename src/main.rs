mod routes;
mod models;

#[macro_use] extern crate rocket;

use crate::models::database::Database;

#[get("/")]
fn index() -> &'static str {
    "This is a master (chief) server for ElDewrito."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Database::new())
        .mount("/", routes![
            index,
            routes::announce::announce,
            routes::list::list
        ])

}
