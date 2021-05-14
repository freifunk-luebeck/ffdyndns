use domain::base;
use domain::base::ParsedDname;
use std::str::FromStr;
use rocket::request::{FromParam, FromFormValue};
use rocket::http::RawStr;
use std::fmt;


#[derive(Clone)]
pub struct Dname{
	parts: Vec<String>,
}

impl Dname {
	pub fn new(domain: String) -> Self {
		if !domain.ends_with(".") {
			panic!("wwhhhaatttttt??????");
		}
		let parts = domain.split_inclusive(".").map(|n| n.to_string()).collect();

		Self {
			parts
		}

	}

	pub fn ends_with(&self, other: &Self) -> bool {
		if self.parts.len() < other.parts.len() {
			return false
		}

		for (my, others) in other.parts.iter().rev().zip(self.parts.iter().rev()) {
			println!("comparing {} == {}", my, others);
			if my != others {
				return false;
			}
		}

		true
	}

	pub fn is_subdomain(&self, other: &Dname) -> bool {
		// "True" ends_with. Similar to `ends_with` but cant be equal.
		other.parts.len() > self.parts.len() && other.ends_with(self)
	}

	pub fn is_subdomain_of(&self, other: &Dname) -> bool {
		self.parts.len() > other.parts.len() && self.ends_with(other)
	}
}


impl FromStr for Dname {
	type Err = String;

	fn from_str(a: &str) -> Result<Self, Self::Err> {
		Ok(Self::new(a.to_string()))
	}
}


impl<'r> FromParam<'r> for Dname {
	type Error = &'r RawStr;

	fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
		Ok(Dname::new(param.url_decode().unwrap().to_string()))
	}
}


impl<'v> FromFormValue<'v> for Dname {
	type Error = &'v RawStr;

	fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
		Ok(Self::new(form_value.to_string()))
	}
}


impl fmt::Display for Dname {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}.", self.parts.join(""))
	}
}

impl fmt::Debug for Dname {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<Self as fmt::Display>::fmt(self, f)
	}
}



#[test]
fn suffix_test() {
	let base = Dname::new("ffhl.de.".to_string());
	let subdomain1 = Dname::new("foo.bar.ffhl.de.".to_string());
	let subdomain2 = Dname::new("foo.ffhl.de.".to_string());
	let subdomain3 = Dname::new("ffhl.de.".to_string());
	let subdomain4 = Dname::new("de.".to_string());

	assert!(subdomain1.ends_with(&base));
	assert!(subdomain2.ends_with(&base));
	assert!(subdomain3.ends_with(&base));
	assert!(!subdomain4.ends_with(&base));
}


#[test]
fn subdomain_test() {
	let base = Dname::new("ffhl.de.".to_string());

	let subdomain1 = &Dname::new("foo.ffhl.de.".to_string());
	let subdomain2 = &Dname::new("foo.bar.ffhl.de.".to_string());

	let not_subdomain1 = &Dname::new("foo.chaotikum.de.".to_string());
	let not_subdomain2 = &Dname::new("ffhl.de.".to_string());
	let not_subdomain3 = &Dname::new("net.".to_string());

	assert!(base.is_subdomain(subdomain1));
	assert!(base.is_subdomain(subdomain2));

	assert!(!base.is_subdomain(not_subdomain1));
	assert!(!base.is_subdomain(not_subdomain2));
	assert!(!base.is_subdomain(not_subdomain3));

	assert!(subdomain1.is_subdomain_of(&base));
	assert!(subdomain2.is_subdomain_of(&base));
	assert!(!not_subdomain1.is_subdomain_of(&base));
	assert!(!not_subdomain2.is_subdomain_of(&base));
}
