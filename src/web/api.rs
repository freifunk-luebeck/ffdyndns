use super::AppState;
use super::ClientIp;
use crate::domain::Dname;
use crate::ffdyndns::UpdateRequest;
#[allow(unused_imports)]
use log::{debug, error, info};
use rocket;
use rocket::http::Status;
use rocket::State;
use rocket::{get};
use std::net::IpAddr;
use rocket::response::{content::Plain};


#[get("/update?<token>&<domain>&<ip>")]
pub fn update(
	state: &State<AppState>,
	clientip: ClientIp,
	token: String,
	domain: Dname,
	ip: Option<String>,
) -> Result<Plain<String>, Status> {
	// prefer the ip address from parameters
	let new_ip: IpAddr = {
		if let Some(iip) = ip {
			iip.parse::<IpAddr>().unwrap()
		} else {
			clientip.into_inner()
		}
	};

	state.service
		.update_domain(UpdateRequest {
			addr: new_ip,
			token: token,
			domain: domain.to_string(),
		})
		.map(|_| Plain("Update successful\n".to_string()))
		.map_err(|_| Status::BadRequest)
}



//curl -X PUT localhost:1234/api/foobar.ffhl.de/A -d "123.123.123.123" -H 'Authorization: API-KEY}'
// #[put("/<domain>/<record>", data = "<ip>")]
// pub fn update_rest(
// 	state: State<AppState>,
// 	clientip: ClientIp,
// 	domain: Dname,
// 	record: QType,
// 	ip: Option<String>,
// 	token: AuthorizationToken,
// ) -> Result<String, Error> {
// 	let new_ip: IpAddr = {
// 		if let Some(iip) = ip {
// 			iip.parse::<IpAddr>().unwrap()
// 		} else {
// 			clientip.into_inner()
// 		}
// 	};


// 	let correct_record_type = || {
// 		match record {
// 			QType::A => new_ip.is_ipv4(),
// 			QType::AAAA => new_ip.is_ipv6(),
// 			_ => false
// 		}
// 	};

// 	if !correct_record_type() {
// 		return Err(Error::RecordTypeNotMatching);
// 	}

// 	state
// 		.service
// 		.update_domain(UpdateRequest {
// 			addr: new_ip,
// 			token: token.to_string(),
// 			domain: domain.to_string(),
// 		})
// 		.map(|_| "Update successful\n".to_string())
// }

#[allow(dead_code)]
#[get("/status?<domain>")]
fn status(state: &State<AppState>, domain: String) -> String {
	let domaininfo = match state.service.get_domain(&domain) {
		None => return "domain not found".to_string(),
		Some(r) => r,
	};

	format!("{:#?}", domaininfo)
}
