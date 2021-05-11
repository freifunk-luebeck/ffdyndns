#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod web;

use chrono::DateTime;
use chrono::Utc;
use crate::db::Database;
use crate::db::Domain;
use rocket;
use rocket::get;
use rocket::post;
use rocket::routes;
use rocket::State;
use rocket::request::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use std::fmt::{self, Display};
use std::net::IpAddr;
use rand;



#[derive(Debug, Clone)]
pub struct DomainUpdate {
    domain: String,
    ip: IpAddr
}


fn main() {
    let db = db::Database::new("./ffddns.sqlite".into());
    web::start_web(db);
}
