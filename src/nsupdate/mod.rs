pub mod nsupdate;

use crate::{
	NSUPDATE_BIN,
	NSUPDATE_TIMEOUT
};
use std::process::Command;
use std::process::Stdio;
use std::io::Write;
use nsupdate::UpdateMessage;
#[allow(unused_imports)]
use log::{error, warn, info, debug};
use std::io::Read;
use std::thread;
use std::sync::mpsc::{self, Sender};


pub fn start_nsupdater() -> Sender<UpdateMessage> {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		for req in rx {
			if let Err(e) = run_nsupdate(req) {
				error!("nsupdate failed: {}", e);
			}
		}
	});

	tx
}


pub fn run_nsupdate(msg: UpdateMessage) -> Result<(),String>{
	let mut child = Command::new(NSUPDATE_BIN)
		.stdout(Stdio::null())
		.stdin(Stdio::piped())
		.stderr(Stdio::piped())
		.args(["-t", NSUPDATE_TIMEOUT.to_string().as_str()])
		.spawn()
		.expect("cannot start pdns_server");


	let mut stdin = child.stdin.take().expect("stdin for nsupdate process is not available");
	let mut stderr = child.stderr.take().expect("stderr for nsupdate process is not available");
	let updatestring = msg.finalize();
	debug!("sending to nsupdate: {}", updatestring);

	let stdin_thread = thread::spawn(move || {
		stdin.write_all(updatestring.as_bytes()).expect("cannot write to nsupdate process");
		// finish input
		stdin.flush().unwrap();
		drop(stdin);
	});

	let mut error = String::new();
	stderr.read_to_string(&mut error).unwrap();

	let rcode = child.wait().unwrap();
	info!("nsupdate returned {}", rcode);
	stdin_thread.join().unwrap();
	rcode.exit_ok().map_err(|_| {error})
}
