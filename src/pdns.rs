use std::process::{Stdio, Command, Child, ExitStatus};
use std::sync::{Arc, Mutex, atomic::AtomicBool, atomic::AtomicU32, atomic::AtomicI32, atomic::Ordering};
use std::rc::Rc;
use std::thread;


const PDNS_BINARY: &str = "/usr/sbin/pdns_server";
const REMOTE_API_URL: &str = "http:url=http://localhost:8053/dns";
const BIND_ADDRESS: &str = "0.0.0.0";
const BIND_PORT: u16 = 5302;
const CTRL_SOCKET: &str = "/tmp/ffdyndns";



pub struct PdnsProcessBuilder {
	dns_port: u16,
	dns_address: String,
}


impl PdnsProcessBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn port(mut self, port: u16) -> Self {
		self.dns_port = port;
		self
	}

	pub fn spawn(self) -> PdnsProcess {
		let child = Command::new(PDNS_BINARY)
			.stdout(Stdio::null())
			.arg("--launch=remote")
			.arg("--no-config")
			.arg("--daemon=no")
			.arg("--log-dns-queries=yes")
			.arg("--loglevel=5")
			.arg("--disable-syslog")
			.arg(format!("--socket-dir={}", CTRL_SOCKET))
			.arg(format!("--local-port={}", self.dns_port))
			.arg(format!("--local-address={}", self.dns_address))
			.arg(format!("--remote-connection-string={}", REMOTE_API_URL))
			.spawn()
			.expect("cannot start pdns_server");

		PdnsProcess::new(child)
	}
}


impl Default for PdnsProcessBuilder {
	fn default() -> Self {
		Self {
			dns_port: BIND_PORT,
			dns_address: BIND_ADDRESS.to_string()
		}
	}
}


pub struct PdnsProcess {
	running: Arc<AtomicBool>,
	code: Arc<AtomicI32>,
}

impl PdnsProcess {
	fn new(mut child: Child) -> Self {

		let running = Arc::new(AtomicBool::new(true));
		let code = Arc::new(AtomicI32::new(0));

		let running_shared = running.clone();
		let code_shared = code.clone();

		std::thread::spawn(move || {
			let c = child.wait().unwrap();

			running_shared.store(false, Ordering::Relaxed);
			code_shared.store(c.code().unwrap(), Ordering::Relaxed);
		});

		Self { running, code }
	}

	pub fn is_running(&self) -> bool {
		self.running.load(Ordering::Relaxed)
	}

	pub fn code(&self) -> Option<i32> {
		if self.is_running() {
			return None;
		}

		return Some(self.code.load(Ordering::Relaxed))
	}
}
