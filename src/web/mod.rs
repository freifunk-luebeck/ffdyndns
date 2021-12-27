mod api;
mod web;

use crate::db::{self, Database, Domain};
use crate::domain::Dname;
use crate::ffdyndns;
use chrono::DateTime;
use chrono::Utc;
#[allow(unused_imports)]
use log::{debug, error, info};
use rand;
use rocket;
use rocket::get;
use rocket::post;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::http::Status;
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
use std::net::SocketAddr;
use crate::CONFIG;
use rocket::fs::FileServer;
use rocket_dyn_templates::{Template, Engines};


pub struct AppState {
	// templates: Tera,
	db: Database,
	service: ffdyndns::Service,
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


#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIp {
	type Error = String;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let ip = request.client_ip().unwrap();
		Outcome::Success(ClientIp(ip))
	}
}



pub struct AuthorizationToken(String);

impl AuthorizationToken {
	pub fn inner(&self) -> &String {
		&self.0
	}

	pub fn into_inner(self) -> String {
		self.0
	}
}


impl Display for AuthorizationToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.inner().to_string())
	}
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizationToken {
	type Error = String;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		let header = match request.headers().get_one("Authorization") {
			None => return Outcome::Failure((Status::Unauthorized, "Authorization header is missing".to_string())),
			Some(x) => x.to_string()
		};

		Outcome::Success(AuthorizationToken(header))
	}
}


pub async fn start_web(db: Database) {
	let appstate = AppState {
		db: db.clone(),
		service: ffdyndns::Service::new(db),
	};

	let config = rocket_config();

	rocket::custom(config)
		.mount("/", routes![
			web::index,
			web::newdomain
		])
		.mount("/api", routes![
			api::update,
			// api::update_rest
		])
		.mount("/static", FileServer::from("./static"))
		.manage(appstate)
		.attach(Template::fairing())
		.launch().await.unwrap();
}


#[cfg(debug_assertions)]
fn rocket_config() -> rocket::Config {
	let mut conf = rocket::Config::debug_default();
	conf.port = CONFIG.bind_port as u16;
	conf.address = CONFIG.bind_address;
	conf
}

#[cfg(not(debug_assertions))]
fn rocket_config() -> rocket::Config {
	let mut conf = rocket::Config::release_default();
	conf.port = CONFIG.bind_port as u16;
	conf.address = CONFIG.bind_address;
	conf
}
