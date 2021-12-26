use serde::{Serialize, Deserialize};
use serde::{self, Serializer, Deserializer};
use std::net::IpAddr;
use chrono::Duration;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
	pub name: String,
	pub description: String,
	pub server_web_url: String,
	pub domain: Vec<Domain>,
	pub database: String,
	pub dns_server: String,
	pub bind_address: IpAddr,
	pub bind_port: u16,
}

impl Config {
	pub fn get_domain_config(&self, domain: &String) -> Option<&Domain> {
		self.domain.iter().find(|e| &e.name == domain)
	}
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
	pub validity: usize,
}
