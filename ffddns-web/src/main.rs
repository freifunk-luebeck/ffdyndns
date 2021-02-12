#![feature(proc_macro_hygiene, decl_macro)]

use rocket;
use rocket::get;
use rocket::routes;

#[get("/update?<key>&<address>")]
fn index(key: String, address: Option<String>) -> String {
    format!("{}: updating ip address to {:?}", key, address)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}
