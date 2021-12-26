use std::net::IpAddr;
use std::string::ToString;
use crate::CONFIG;
use crate::DNSTTL;
use crate::ffdyndns::UpdateRequest;


// server 127.0.0.1
// update delete a.dyn.example.com
// update add a.dyn.example.com 60 A 123.23.123.1
// send


pub enum UpdateCommand {
	Delete(String),
	Add(String, IpAddr),
}


impl UpdateCommand {
	pub fn delete(d: &String) -> Self {
		Self::Delete(d.to_string())
	}

	pub fn add(d: &String, a: IpAddr) -> Self {
		Self::Add(d.to_string(), a)
	}

	fn finalize(self) -> String {
		match self {
			Self::Delete(s) => format!("update delete {}", s),
			// update add a.dyn.example.com 60 A 123.23.123.1
			Self::Add(d, a) => format!("update add {} {} {} {}", d, DNSTTL, addr_class(a), a),
		}
	}
}


pub struct UpdateMessage {
	commands: Vec<UpdateCommand>,
}


impl UpdateMessage {
	pub fn new() -> Self {
		Self { commands: Vec::new() }
	}

	pub fn from_updaterequest(ur: UpdateRequest) -> Self {
		let mut nsup = Self::new();
		nsup.add_command(UpdateCommand::delete(&ur.domain));
		nsup.add_command(UpdateCommand::add(&ur.domain, ur.addr));
		nsup
	}

	pub fn new_remove_message(d: String) -> Self{
		let mut nsup = Self::new();
		nsup.add_command(UpdateCommand::delete(&d));
		nsup
	}

	pub fn add_command(&mut self, cmd: UpdateCommand) {
		self.commands.push(cmd);
	}

	pub fn finalize(self) -> String {
		let mut out = Vec::new();

		out.push(format!("server {}", CONFIG.dns_server));
		for update in self.commands {
			out.push(update.finalize());
		}
		out.push(format!("send"));

		out.join("\n")
	}
}




fn addr_class(a: IpAddr) -> String {
	match a {
		IpAddr::V4(_) => "A".to_string(),
		IpAddr::V6(_) => "AAAA".to_string(),
	}
}
