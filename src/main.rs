extern crate dirs;

use std::io;
use std::process::Command;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from pipe");
        input = input.trim().to_string();
        match &input {
		//input if input.contains( "myind.14 MINE" ) => {
		//	println!("{}", input)
		//},
		input if input.is_empty() => println!("empty line..."),
		input if input.contains( "error" ) || input.contains("confirm")  => {
			println!("sent to telegram: {}", input);
			let home_dir = dirs::home_dir().expect("couldnt get home dir");
			let home_dir = home_dir.to_str().unwrap();
			Command::new(&format!("{}{}", home_dir, "/telegram_msg.sh"))
				.arg(&format!("{}", input))
				.output()
				.expect("failed to send");
		}, 
		_ => (),
        }
    }
}
