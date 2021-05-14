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


#[get("/update?<token>&<domain>&<ip>")]
pub fn update(
	state: State<AppState>,
	clientip: ClientIp,
	token: String,
	domain: Dname,
	ip: Option<String>,
) -> String {
	let new_ip: IpAddr = {
		if let Some(iip) = ip {
			iip.parse::<IpAddr>().unwrap()
		} else {
			clientip.into_inner()
		}
	};
	let db = &state.db;
	let d = db.get_domain(&domain.to_string()).unwrap();
	info!("{:#?}", d);

	if d.token != token {
		return "not a valid token".to_string();
	}

	info!("write new ip to database: {:?}", new_ip);
	match new_ip {
		IpAddr::V4(addr) => db.update_ipv4(&"foobar.ffhl.de.".to_string(), addr),
		IpAddr::V6(addr) => db.update_ipv6(&"foobar.ffhl.de.".to_string(), addr),
	}

	info!("update timestamp");
	db.update_lastupdate(&domain.to_string(), Utc::now());

	format!("{} updated to {:?}", domain, new_ip)
}


#[get("/status?<domain>")]
fn status(db: State<AppState>, domain: String) -> String {
	let domaininfo = match db.db.get_domain(&domain) {
		None => return "domain not found".to_string(),
		Some(r) => r,
	};

	format!("{:#?}", domaininfo)
}
