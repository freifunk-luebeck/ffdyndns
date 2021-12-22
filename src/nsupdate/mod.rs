pub mod nsupdate;

use crate::NSUPDATE_BIN;
use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use nsupdate::UpdateMessage;
use log::{warn, info, debug};

pub fn run_nsupdate(msg: UpdateMessage) {
	let mut child = Command::new(NSUPDATE_BIN)
		.stdout(Stdio::null())
		.stdin(Stdio::piped())
		// .arg("--launch=remote")
		// .arg("--no-config")
		// .arg("--daemon=no")
		// .arg("--log-dns-queries=yes")
		// .arg("--loglevel=5")
		// .arg("--disable-syslog")
		// .arg(format!("--socket-dir={}", CTRL_SOCKET))
		// .arg(format!("--local-port={}", self.dns_port))
		// .arg(format!("--local-address={}", self.dns_address))
		// .arg(format!("--remote-connection-string={}", REMOTE_API_URL))
		.spawn()
		.expect("cannot start pdns_server");


	let mut stdin = child.stdin.take().expect("stdin for nsupdate process is not available");

	let updatestring = msg.finalize();
	debug!("sending to nsupdate: {}", updatestring);
	warn!("writing to nsupdate");
	stdin.write_all(updatestring.as_bytes()).expect("cannot write to nsupdate process");
	// finish input
	stdin.write(b"\n").unwrap();
	stdin.write(&[0x4]).unwrap();

	stdin.flush().unwrap();
	warn!("wating for nsupdate to exit");
	// child.wait().unwrap();
	warn!("nsupdate exited");
}
