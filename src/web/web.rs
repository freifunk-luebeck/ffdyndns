use crate::CONFIG;
use crate::domain::Dname;
#[allow(unused_imports)]
use log::{debug, error, info};
use rocket_dyn_templates::Template;
use rocket;
use rocket::get;
use rocket::State;
use serde_json as json;
use serde_json::json;
use serde::{Deserialize, Serialize};
use super::AppState;
use tera;



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
			server_url: CONFIG.server_web_url.clone(),
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
pub fn index(state: &State<AppState>) -> Template {
	Template::render(
		"index",
		TemplateContext::empty()
	)
}


#[get("/newdomain?<domainname>&<suffix>&<tos>")]
pub fn newdomain(
	state: &State<AppState>,
	domainname: Option<String>,
	suffix: Option<String>,
	tos: Option<bool>,
) -> Template {
	let template_data = match (&domainname, &suffix, tos) {
		(Some(name), Some(suffix), Some(tos)) if tos => {
			let newdomain: Dname = format!("{}.{}", name, suffix).parse().unwrap();
			let r = state.service.new_domain(newdomain.clone());

			json!({
				"form_request": true,
				"error": r.is_err(),
				"errormsg": r,
				"token": r,
				"domainname": newdomain.to_string()
			})
		}
		_ => {
			json!({
				"form_request": false,
				"available_domains": CONFIG.domain.iter().map(|x| &x.name).collect::<Vec<&String>>()
			})
		}
	};

	Template::render(
		"newdomain",
		TemplateContext::new(template_data)
	)
}
