#![feature(proc_macro_hygiene, decl_macro, exit_status_error)]

mod config;
mod db;
mod domain;
mod web;
mod ffdyndns;
mod nsupdate;

#[allow(unused_imports)]
use log::{debug, error, info, trace};
use clap::{self, App, Arg, ArgMatches};
use config::Config;
use crate::db::Database;
use lazy_static::lazy_static;
use pretty_env_logger;
use rocket::launch;
use std::fs;
use std::io::{Read, Write};
use std::path;
use std::process;
use std::process::exit;
use std::thread;
use std::time;
use tokio;
use toml;

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
pub const CLEAN_INTERVAL: u64 = 30;


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


fn main() {
	pretty_env_logger::init();
	// println!("{:?}", CONFIG.domain);

	let app = clap::App::new(env!("CARGO_PKG_NAME"))
		.subcommand(App::new("server"))
		.subcommand(App::new("genzones")
			.arg(Arg::with_name("out")
				.required(true)
				.long("out")
				.takes_value(true)
				.default_value("./"))
			.arg(Arg::with_name("rname")
				.takes_value(true)
				.long("rname")
				.required(true))
			.arg(Arg::with_name("ns")
				.multiple(true)
				.min_values(1)
				.takes_value(true)
				.long("ns")
				.required(true)));


	match app.get_matches().subcommand() {
		("server", Some(args)) => cmd_server(args),
		("genzones", Some(args)) => cmd_genzones(args),
		_ => {
			error!("try --help");
			process::exit(1)
		}
	}

	process::exit(0);
}

pub fn cmd_genzones(args: &ArgMatches<'_>) {
	let outdir = args.value_of("out").unwrap();
	let rname = args.value_of("rname").unwrap();
	let serial = time::SystemTime::now().duration_since(time::UNIX_EPOCH)
		.unwrap()
		.as_secs();

	for domain in &CONFIG.domain {
		let mut outstr = vec![];
		outstr.push(format!("$ORIGIN ."));
		outstr.push(format!("$TTL 30 ; 30 seconds"));
		outstr.push(format!("{} IN SOA  {} {} (", domain.name, args.values_of("ns").unwrap().next().unwrap() , rname));
		outstr.push(format!("	{} ; serial", serial));
		outstr.push(format!("	3600 ; refresh (1 hour)"));
		outstr.push(format!("	600 ; retry (10 minutes)"));
		outstr.push(format!("	2600 ; expire (43 minutes 20 seconds)"));
		outstr.push(format!("	30 ; minimum (30 seconds)"));
		outstr.push(format!(")"));
		for ns in args.values_of("ns").unwrap() {
			outstr.push(format!("NS {}", ns));
		}
		outstr.push(format!(""));

		let out = outstr.join("\n");

		let mut outfile = fs::File::create(path::Path::new(&format!("{}/{}.zone", outdir, &domain.name))).unwrap();
		outfile.write_all(out.as_bytes()).unwrap();
	}
}


pub fn cmd_server(_: &ArgMatches<'_>) {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let db = db::Database::new(CONFIG.database.clone().into());

	let app = ffdyndns::Service::new(db);

	let app_cleaner = app.clone();

	// start cleaning thread
	std::thread::spawn(move || {
		loop {
			thread::sleep(time::Duration::from_secs(CLEAN_INTERVAL));
			app_cleaner.clean_domains();
		}
	});

	rt.block_on(web::start_web(app));
}
