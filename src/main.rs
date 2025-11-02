use std::io;
use std::fs;

fn main() {
	let mut filepath = String::new();
	io::stdin()
		.read_line(&mut filepath)
		.expect("Failed to read line");
	if let Ok(_contents) = fs::read(filepath.trim()) {
		println!("success");
	}
	else {
		println!("failure");
	}
}
