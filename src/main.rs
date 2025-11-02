use std::io;
use std::fs;

fn main() {
	let mut filepath = String::new();
	io::stdin()
		.read_line(&mut filepath)
		.expect("Failed to read line");
	if let Ok(contents) = fs::read_to_string(filepath) {
		println!("success");
	}
	else {
		println!("failure");
	}
}
