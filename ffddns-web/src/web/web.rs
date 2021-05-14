use chrono::DateTime;
use chrono::Utc;
use crate::db::{self, Database, Domain};
use crate::domain::Dname;
use crate::CONFIG;
use lazy_static::lazy_static;
use log::{error, info, debug};
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
use super::AppState;
use super::ClientIp;
use tera::{self};
use tera::Tera;


const TEMPLATES_INCLUDES: &[(&str, &str)] = &[
	("index", include_str!("../../templates/index.html")),
	("nodelist", include_str!("../../templates/nodelist.html")),
	("head", include_str!("../../templates/head.html")),
	("node", include_str!("../../templates/node.html")),
	("navbar", include_str!("../../templates/navbar.html")),
	("newdomain", include_str!("../../templates/newdomain.html")),
];


// load al templates with lazy_static magic
lazy_static! {
	static ref TEMPLATES: Tera = load_templates();
}


#[get("/")]
pub fn index(state: State<AppState>) -> Html<String> {
	let html = TEMPLATES
		.render("index", &tera::Context::from_serialize(&json!({})).unwrap())
		.unwrap();

	Html(html)
}

#[get("/newdomain?<domainname>&<suffix>&<tos>")]
pub fn newdomain(state: State<'_, AppState>, domainname: Option<String>, suffix: Option<String>, tos: Option<bool>) -> Html<String> {
	let db = &state.db;
	let mut template_data: json::Value = json!({});

	match (&domainname, &suffix, tos) {
		(Some(name), Some(suffix), Some(tos)) if tos => {
			let newdomain = format!("{}.{}", name, suffix);
			let r = state.service.new_domain(&newdomain);


			template_data = json!({
				"form_request": true,
				"error": r.is_err(),
				"errormsg": r,
				"token": r
			});
		}
		_ => {
			template_data = json!({
				"form_request": false,
				"available_domains": CONFIG.domain.iter().map(|x| &x.name).collect::<Vec<&String>>()
			})
		}
	}

	let html = TEMPLATES
		.render(
			"newdomain",
			&tera::Context::from_serialize(&template_data).unwrap(),
		)
		.unwrap();

	info!("{:#?}", domainname);

	Html(html)
}


fn load_templates() -> tera::Tera {
	let mut t = tera::Tera::default();

	for (name, template) in TEMPLATES_INCLUDES {
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
