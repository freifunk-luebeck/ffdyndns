use std::str::FromStr;
use rocket::http::RawStr;
use std::fmt;
use rocket::request::FromParam;
use rocket::form::{self, FromFormField, ValueField};


#[derive(Clone)]
pub struct Dname{
	parts: Vec<String>,
}

#[allow(dead_code)]
impl Dname {
	pub fn new(mut domain: String) -> Self {
		if !domain.ends_with(".") {
			domain.push_str(".");
		}

		let parts = domain.split_inclusive(".").map(|n| n.to_string()).collect();

		Self {
			parts
		}
	}


	pub fn strip_subdomain(&self) -> String {
		self.parts[1..].join("")
	}

	pub fn ends_with(&self, other: &Self) -> bool {
		if self.parts.len() < other.parts.len() {
			return false
		}

		for (my, others) in other.parts.iter().rev().zip(self.parts.iter().rev()) {
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

	fn from_param(param: &'r str) -> Result<Self, Self::Error> {
		Ok(Dname::new(param.to_string()))
	}
}


impl<'v> FromFormField<'v> for Dname {
	fn from_value(form_value: ValueField) -> form::Result<'v, Self> {
		Ok(Self::new(form_value.value.to_string()))
	}
}


impl fmt::Display for Dname {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.parts.join(""))
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


#[test]
fn subdomnain_stripping() {
	let domain1 = Dname::new("test.ffhl.de.".to_string());
	let domain2 = Dname::new("foo.bar.ffhl.de.".to_string());

	assert_eq!(domain1.strip_subdomain(), "ffhl.de.");
	assert_eq!(domain2.strip_subdomain(), "bar.ffhl.de.");
}
