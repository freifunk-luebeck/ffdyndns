use serde::{Serialize, Deserialize};
use serde;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub description: String,
	pub server_url: String,
	pub domain: Vec<Domain>,
	pub dns: Dns,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dns {
	pub master: String,
	pub rname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Domain {
	/// the domain suffix. eg. for a dynamic domain
	/// mydomain.ddns.org the name here is ddns.org
	pub name: String,
	/// a short description for a domain
	pub description: String,
	/// a list of networks, which a subdomain from this
	/// domain is allowed to updated to
	pub nets: Vec<String>,
	/// duration in days before a subdomain gets 'released' wgen not updated
	pub registration_time: usize,
}
