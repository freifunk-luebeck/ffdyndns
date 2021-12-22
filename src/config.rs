use serde::{Serialize, Deserialize};
use serde;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub description: String,
	pub server_url: String,
	pub domain: Vec<Domain>,
	pub dns: Dns,
	pub database: String,
	pub dns_server: String,
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
	pub allowed_ips: Vec<String>,
	/// duration in days before a subdomain gets 'released`
	pub registration_time: usize,
}
