use domain::base;
use domain::base::ParsedDname;
use std::str::FromStr;
use rocket::request::{FromParam, FromFormValue};
use rocket::http::RawStr;
use std::fmt;


#[derive(Clone)]
pub struct Dname(base::Dname<Vec<u8>>);

impl Dname {
	pub fn new(domain: &String) -> Self {
		Self(base::Dname::from_str(domain).unwrap())
	}

	pub fn ends_with(&self, other: &Self) -> bool {
		self.0.ends_with(&other.0)
	}

	pub fn starts_with(&self, other: &Self) -> bool {
		self.0.starts_with(&other.0)
	}

	pub fn is_root(&self) -> bool {
		self.0.is_root()
	}

	pub fn is_absolute(&self) -> bool {
		self.0.last().is_root()
	}
}


impl<'r> FromParam<'r> for Dname {
	type Error = &'r RawStr;

	fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
		Ok(Dname::new(&param.url_decode().unwrap().to_string()))
	}
}


impl<'v> FromFormValue<'v> for Dname {
	type Error = &'v RawStr;

	fn from_form_value(form_value: &'v RawStr) -> Result<Self, &'v RawStr> {
		Ok(Self::new(&form_value.to_string()))
	}
}


impl fmt::Display for Dname {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}.", self.0)
	}
}

impl fmt::Debug for Dname {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		<Self as fmt::Display>::fmt(self, f)
	}
}
