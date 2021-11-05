mod routes;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "This is a master (chief) server for ElDewrito."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            index,
            routes::announce::announce
        ])

}
