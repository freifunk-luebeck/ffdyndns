use log::{error, info, debug};
use tera::{self};
use crate::db::{self, Database, Domain};
use chrono::DateTime;
use chrono::Utc;
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
use crate::domain::Dname;
use super::AppState;
use super::ClientIp;
use crate::ffdyndns::{UpdateRequest, Error};


#[get("/update?<token>&<domain>&<ip>")]
pub fn update(
	state: State<AppState>,
	clientip: ClientIp,
	token: String,
	domain: Dname,
	ip: Option<String>,
) -> Result<String, Error> {
	// prefer the ip address from parameters
	let new_ip: IpAddr = {
		if let Some(iip) = ip {
			iip.parse::<IpAddr>().unwrap()
		} else {
			clientip.into_inner()
		}
	};

	state.service.update_domain(UpdateRequest {
		addr: new_ip,
		token: token,
		domain: domain.to_string()
	}).map(|_| "Update successful\n".to_string())
}


#[get("/status?<domain>")]
fn status(db: State<AppState>, domain: String) -> String {
	let domaininfo = match db.db.get_domain(&domain) {
		None => return "domain not found".to_string(),
		Some(r) => r,
	};

	format!("{:#?}", domaininfo)
}
