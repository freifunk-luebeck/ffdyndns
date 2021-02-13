#![feature(proc_macro_hygiene, decl_macro)]

mod db;
use chrono::{DateTime, Utc};
use db::{
    Database,
    Domain,
};
use rocket::{
    self,
    get,
    post,
    routes,
    State,
};
use rocket::request::{
    Request,
    FromRequest,
    Outcome,
};
use std::fmt::{self, Display};
use std::net::IpAddr;
use rand;


pub struct ClientIp(IpAddr);


impl ClientIp {
    pub fn inner(&self) -> &IpAddr {
        let ClientIp(ip) = self;
        ip
    }
    pub fn into_inner(self) -> IpAddr {
        let ClientIp(ip) = self;
        ip
    }
}


impl Display for ClientIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner().to_string())
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for ClientIp {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let ip = request.client_ip().unwrap();
        Outcome::Success(ClientIp(ip))
    }
}


#[derive(Debug, Clone)]
pub struct DomainUpdate {
    domain: String,
    ip: IpAddr
}


#[get("/update?<token>&<domain>&<ip>")]
fn update(db: State<Database>, clientip: ClientIp, token: String, domain: String, ip: Option<String>) -> String {
    let new_ip: IpAddr = {
        if let Some(iip) = ip {
            iip.parse::<IpAddr>().unwrap()
        }
        else {
            clientip.into_inner()
        }
    };

    let d = db.get_domain(&domain).unwrap();

    if d.token != token {
        return "not a valid token".to_string();
    }

    match new_ip {
        IpAddr::V4(addr) => db.update_ipv4(&domain, addr),
        IpAddr::V6(addr) => db.update_ipv6(&domain, addr),
    }

    db.update_lastupdate(&domain, Utc::now());

    format!("{} updated to {:?}", domain, new_ip)
}


#[get("/create?<domain>")]
fn create(db: State<Database>, domain: String) -> String {
    let token = generate_token();
    let d = Domain {
        domainname: domain.clone(),
        token: token.clone(),
        lastupdate: None,
        ipv4: None,
        ipv6: None
    };

    db.insert_new_domain(&d);

    format!("your token for {}: {}", domain, token)
}


#[get("/status?<domain>")]
fn status(db: State<Database>, domain: String) -> String {
    let domaininfo = match db.get_domain(&domain) {
        None => return "domain not found".to_string(),
        Some(r) => r,
    };

    format!("{:#?}", domaininfo)
}



fn main() {
    let db = db::Database::new("./ffddns.sqlite".into());

    rocket::ignite()
        .mount("/", routes![
            update,
            create,
            status
        ])
        .manage(db)
        .launch();
}



fn generate_token() -> String {
    let mut token = String::new();

    for _ in 0..8 {
        token.push_str(&format!("{:02x}", rand::random::<u8>()));
    }

    token
}
