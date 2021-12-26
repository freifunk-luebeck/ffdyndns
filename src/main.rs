#![feature(proc_macro_hygiene, decl_macro, exit_status_error)]

mod config;
mod db;
mod domain;
mod web;
mod ffdyndns;
mod nsupdate;
mod db2;

use config::Config;
use crate::db::Database;
use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, error, info, trace};
use rocket;
use std::fs;
use std::io::Read;
use std::path;
use std::process::exit;
use toml;
use pretty_env_logger;

const CONFIG_DIRS: &[&str] = &[
	"./ffdyndns.toml",
	"/etc/ffdyndns.toml",
	"/var/lib/ffdyndns/ffdyndns.toml",
];

pub const WEB_STATIC_DIR: &str = "/usr/lib/ffdyndns/static";
pub const WEB_TEMPLATES_DIR: &str = "/usr/lib/ffdyndns/templates";

pub const DNSTTL: usize = 60;
pub const NSUPDATE_BIN: &str = "/usr/bin/nsupdate";
pub const NSUPDATE_TIMEOUT: u32 = 3;


lazy_static! {
	pub static ref CONFIG: Config = {
		let file = CONFIG_DIRS
			.iter()
			.map(|x| path::Path::new(x))
			.find(|p| p.exists() && p.is_file());

		match file {
			Some(f) => {
				eprintln!("loading config: {}", f.to_str().unwrap());
				let mut f = fs::File::open(f).unwrap();
				let mut toml_str = String::new();
				f.read_to_string(&mut toml_str)
					.expect("can't read config file");

				match toml::from_str::<Config>(&toml_str) {
					Err(e) => {
						eprintln!("configuration error: {}", e);
						exit(1);
					}
					Ok(r) => r,
				}
			}
			None => {
				eprintln!("could not find config file");
				exit(1);
			}
		}
	};
}

#[macro_export]
macro_rules! sha256 {
	($x:expr) => {{
		use crypto::digest::Digest;
		use crypto::sha2::Sha256;

		let mut sum = Sha256::new();
		sum.input_str($x);
		sum.result_str()
	}};
}


#[rocket::main]
async fn main() {
	pretty_env_logger::init();
	// println!("{:?}", CONFIG.domain);

	let db = db::redis::Redisdb::new(&CONFIG.database.into()).unwrap();
	web::start_web(&db).await;
}
