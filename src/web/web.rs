use super::AppState;
use super::ClientIp;
use crate::db::{self, Database, Domain};
use crate::domain::Dname;
use crate::CONFIG;
use chrono::DateTime;
use chrono::Utc;
use lazy_static::lazy_static;
use log::{debug, error, info};
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
use serde::{Deserialize, Serialize};
use serde_json as json;
use serde_json::json;
use std::fmt::{self, Display};
use std::net::IpAddr;
use tera::Tera;
use tera::{self};

const TEMPLATES_INCLUDES: &[(&str, &str)] = &[
	("index", include_str!("../../templates/index.html")),
	("head", include_str!("../../templates/head.html")),
	("navbar", include_str!("../../templates/navbar.html")),
	("newdomain", include_str!("../../templates/newdomain.html")),
];

// load al templates with lazy_static magic
lazy_static! {
	static ref TEMPLATES: Tera = load_templates();
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TemplateContext<T> {
	server_url: String,
	domains: Vec<json::Value>,
	name: String,
	description: String,
	data: T,
}

impl TemplateContext<()> {
	fn empty() -> Self {
		Self::new(())
	}
}

impl<T> TemplateContext<T> {
	fn new(data: T) -> Self {
		Self {
			name: CONFIG.name.clone(),
			description: CONFIG.description.clone(),
			server_url: CONFIG.server_url.clone(),
			domains: CONFIG
				.domain
				.iter()
				.map(|d| json!({"name": d.name.clone(), "description": d.description.clone()}))
				.collect(),
			data: data,
		}
	}
}

#[get("/")]
pub fn index(state: State<AppState>) -> Html<String> {
	let html = TEMPLATES
		.render(
			"index",
			&tera::Context::from_serialize(TemplateContext::empty()).unwrap(),
		)
		.unwrap();

	Html(html)
}

#[get("/newdomain?<domainname>&<suffix>&<tos>")]
pub fn newdomain(
	state: State<'_, AppState>,
	domainname: Option<String>,
	suffix: Option<String>,
	tos: Option<bool>,
) -> Html<String> {
	let db = &state.db;
	let mut template_data: json::Value = json!({});

	match (&domainname, &suffix, tos) {
		(Some(name), Some(suffix), Some(tos)) if tos => {
			let newdomain: Dname = format!("{}.{}", name, suffix).parse().unwrap();
			let r = state.service.new_domain(newdomain.clone());

			template_data = json!({
				"form_request": true,
				"error": r.is_err(),
				"errormsg": r,
				"token": r,
				"domainname": newdomain.to_string(),
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
			&tera::Context::from_serialize(&TemplateContext::new(template_data)).unwrap(),
		)
		.unwrap();

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
