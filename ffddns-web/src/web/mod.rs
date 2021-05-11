mod dns;

use log::{error, info};
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

const TEMPLATES: &[(&str, &str)] = &[
	("index", include_str!("../../templates/index.html")),
	("nodelist", include_str!("../../templates/nodelist.html")),
	("head", include_str!("../../templates/head.html")),
	("node", include_str!("../../templates/node.html")),
	("navbar", include_str!("../../templates/navbar.html")),
	("newdomain", include_str!("../../templates/newdomain.html")),
];

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

#[get("/update?<token>&<domain>&<ip>")]
fn update(
	db: State<Database>,
	clientip: ClientIp,
	token: String,
	domain: String,
	ip: Option<String>,
) -> String {
	let new_ip: IpAddr = {
		if let Some(iip) = ip {
			iip.parse::<IpAddr>().unwrap()
		} else {
			clientip.into_inner()
		}
	};

	let d = db.get_domain(&domain).unwrap();

	if d.token != token {
		return "not a valid token".to_string();
	}

	match new_ip {
		IpAddr::V4(addr) => db.update_ipv4(&domain, addr),
		IpAddr::V6(addr) => db.update_ipv6(&domain, addr),
	}

	db.update_lastupdate(&domain, Utc::now());

	format!("{} updated to {:?}", domain, new_ip)
}

#[get("/create?<domain>")]
fn create(db: State<Database>, domain: String) -> String {
	let token = db::generate_token();
	let d = Domain {
		domainname: domain.clone(),
		token: token.clone(),
		lastupdate: None,
		ipv4: None,
		ipv6: None,
	};

	db.insert_new_domain(&d);

	format!("your token for {}: {}", domain, token)
}

#[get("/status?<domain>")]
fn status(db: State<Database>, domain: String) -> String {
	let domaininfo = match db.get_domain(&domain) {
		None => return "domain not found".to_string(),
		Some(r) => r,
	};

	format!("{:#?}", domaininfo)
}

#[get("/")]
fn index(state: State<'_, AppState>) -> Html<String> {
	let html = state
		.templates
		.render("index", &tera::Context::from_serialize(&json!({})).unwrap())
		.unwrap();

	Html(html)
}

#[get("/newdomain?<domainname>")]
fn newdomain(state: State<'_, AppState>, domainname: Option<String>) -> Html<String> {
	let db = &state.db;
	let mut template_data: json::Value = json!({});

	match &domainname {
		Some(name) if db.get_domain(&name).is_some() => {
			template_data = json!({
				"form_request": true,
				"created": false,
				"error": true,
				"error_msg": "Domain already exists"
			})
		}
		Some(name) if db.get_domain(&name).is_none() => {
			let domain = db::Domain::new(name.clone());
			db.insert_new_domain(&domain);

			template_data = json!({
				"form_request": true,
				"created": true,
				"error": false,
				"token": domain.token
			});
		}
		None | _ => {
			template_data = json!({
				"form_request": false
			})
		}
	}



	let html = state
		.templates
		.render(
			"newdomain",
			&tera::Context::from_serialize(&template_data).unwrap()
		).unwrap();

	info!("{:#?}", domainname);

	Html(html)
}

pub fn start_web(db: Database) {
	let appstate = AppState {
		db: db,
		templates: load_templates(),
	};

	rocket::custom(rocket::config::ConfigBuilder::new(rocket::config::Environment::Development)
		.port(8053)
		.finalize()
		.unwrap()
	)
		.mount("/", routes![index, update, create, status, newdomain])
		.mount("/dns", routes![dns::lookup])
		.manage(appstate)
		.launch();
}

fn load_templates() -> tera::Tera {
	let mut t = tera::Tera::default();

	for (name, template) in TEMPLATES {
		if let Err(e) = t.add_raw_template(name, template) {
			error!("failed to load template: {}", name);
			match &e.kind {
				tera::ErrorKind::Msg(m) => error!("{}", m),
				_ => error!("unknown error"),
			}
			panic!("loading templates failed: {:#?}", e);
		}
	}

	t
}
