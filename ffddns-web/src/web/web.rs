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

const TEMPLATES: &[(&str, &str)] = &[
	("index", include_str!("../../templates/index.html")),
	("nodelist", include_str!("../../templates/nodelist.html")),
	("head", include_str!("../../templates/head.html")),
	("node", include_str!("../../templates/node.html")),
	("navbar", include_str!("../../templates/navbar.html")),
	("newdomain", include_str!("../../templates/newdomain.html")),
];



#[get("/")]
pub fn index(state: State<AppState>) -> Html<String> {
	let html = state
		.templates
		.render("index", &tera::Context::from_serialize(&json!({})).unwrap())
		.unwrap();

	Html(html)
}

#[get("/newdomain?<domainname>")]
pub fn newdomain(state: State<'_, AppState>, domainname: Option<String>) -> Html<String> {
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
			&tera::Context::from_serialize(&template_data).unwrap(),
		)
		.unwrap();

	info!("{:#?}", domainname);

	Html(html)
}


pub fn load_templates() -> tera::Tera {
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
