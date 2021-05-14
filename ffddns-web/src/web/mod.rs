mod api;
mod dns;
mod web;

use crate::db::{self, Database, Domain};
use crate::domain::Dname;
use chrono::DateTime;
use chrono::Utc;
use log::{debug, error, info};
use rand;
use rocket;
use rocket::get;
use rocket::post;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::request::Request;
use rocket::response::content;
use rocket::response::content::Html;
use rocket::routes;
use rocket::State;
use serde_json as json;
use serde_json::json;
use std::fmt::{self, Display};
use std::net::IpAddr;
use tera::Tera;
use tera::{self};

pub struct AppState {
	templates: Tera,
	db: Database,
}

pub struct ClientIp(IpAddr);

impl ClientIp {
	pub fn inner(&self) -> &IpAddr {
		let ClientIp(ip) = self;
		ip
	}

	pub fn into_inner(self) -> IpAddr {
		let ClientIp(ip) = self;
		ip
	}
}

impl Display for ClientIp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.inner().to_string())
	}
}

impl<'a, 'r> FromRequest<'a, 'r> for ClientIp {
	type Error = String;

	fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
		let ip = request.client_ip().unwrap();
		Outcome::Success(ClientIp(ip))
	}
}


pub fn start_web(db: Database) {
	let appstate = AppState {
		db: db,
		templates: web::load_templates(),
	};

	rocket::custom(
		rocket::config::ConfigBuilder::new(rocket::config::Environment::Development)
			.port(8053)
			// .log_level(rocket::logger::LoggingLevel::Debug)
			.finalize()
			.unwrap(),
	)
	.mount(
		"/",
		routes![
			web::index,
			web::newdomain
		],
	)
	.mount(
		"/dns",
		routes![
			dns::lookup_a,
			dns::lookup_aaaa,
			dns::lookup_soa,
			dns::lookup_getalldomainmetadata,
			dns::lookup_any,
		],
	)
	.mount("/api", routes![
		api::update
	])
	.manage(appstate)
	.launch();
}
