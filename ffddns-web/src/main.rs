#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod web;
mod config;

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
use toml;
use lazy_static::lazy_static;
use config::Config;
use std::fs;
use std::path;
use std::io::Read;
use log::{debug, info, error};
use std::process::exit;


const CONFIG_DIRS: &[&str] = &[
	"./ffdyndns.toml",
	"/etc/ffdyndns.toml",
	"/var/lib/ffdyndns/ffdyndns.toml",
];

lazy_static! {
	pub static ref CONFIG: Config = {
		let file = CONFIG_DIRS.iter()
			.map(|x| path::Path::new(x))
			.find(|p| p.exists() && p.is_file());

		match file {
			Some(f) => {
				debug!("loading config: {}", f.to_str().unwrap());
				let mut f = fs::File::open(f).unwrap();
				let mut toml_str = String::new();
				f.read_to_string(&mut toml_str).expect("can't read config file");

				match toml::from_str::<Config>(&toml_str) {
					Err(e) => {
						eprintln!("configuration error: {}", e);
						exit(1);
					}
					Ok(r) => r
				}
			}
			None => {
				eprintln!("could not find config file");
				exit(1);
			},
		}
	};
}


// for p in CONFIG_DIRS.iter().map(|x| path::Path::new(x)) {
// 	if !p.exists() || !p.is_file() {
// 		continue;
// 	}
// 	debug!("loading config: {}", p.to_str().unwrap());
// 	let mut f = fs::File::open(p).unwrap();
// 	let mut toml_str = String::new();
// 	f.read_to_string(&mut toml_str).expect("can't read config file");

// 	config = toml::from_str::<Config>(&toml_str).unwrap()
// };

// return config;



#[derive(Debug, Clone)]
pub struct DomainUpdate {
	domain: String,
	ip: IpAddr
}


fn main() {
	println!("{:?}", CONFIG.domain);

	let db = db::Database::new("./ffddns.sqlite".into());
	web::start_web(db);
}
