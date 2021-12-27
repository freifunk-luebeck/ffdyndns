use chrono::Duration;
use serde::{self, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::marker::PhantomData;
use std::net::IpAddr;
use std::str::FromStr;
// use void::Void;


#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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
	#[serde(deserialize_with = "deserialize_duration")]
	pub validity: Duration,
}



fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    // T: Deserialize<'de> + FromStr<Err = String>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct DurationDeserializer(PhantomData<fn() -> Duration>);

    impl<'de> Visitor<'de> for DurationDeserializer
    {
        type Value = Duration;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or integer")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Duration, E>
        where E: de::Error {
            Ok(Duration::hours(value))
        }

        // fn visit_i32<E>(self, value: i32) -> Result<Duration, E>
        // where E: de::Error {
        //     Ok(Duration::hours(value as i64))
        // }


        fn visit_str<E>(self, value: &str) -> Result<Duration, E>
        where
            E: de::Error,
        {
            Ok(Duration::hours(FromStr::from_str(value).unwrap()))
        }
    }

    deserializer.deserialize_any(DurationDeserializer(PhantomData))
}
