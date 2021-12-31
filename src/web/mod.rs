mod api;
mod web;

use crate::ffdyndns;
#[allow(unused_imports)]
use log::{debug, error, info};
use rocket;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::http::Status;
use rocket::request::Request;
use rocket::routes;
use std::fmt::{self, Display};
use std::net::IpAddr;
use crate::CONFIG;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;


pub struct AppState {
	// templates: Tera,
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
		let ip = request.headers().get("X-Forwarded-For").next()
			.map(|x| x.parse().unwrap())
			.unwrap_or(request.client_ip().unwrap());

		Outcome::Success(ClientIp(ip))
	}
}



pub struct AuthorizationToken(String);

impl AuthorizationToken {
	pub fn inner(&self) -> &String {
		&self.0
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


pub async fn start_web(app: ffdyndns::Service) {
	let appstate = AppState {
		service: app,
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
