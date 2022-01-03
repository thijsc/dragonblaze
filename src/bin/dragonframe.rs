use std::env::args;
use std::net::UdpSocket;

/// Seperate executable that is called by dragonframe's
/// script feature. It tells us what the position in
/// the scene is. We write this position to the main
/// executable over a socket.

fn main() {
    match args().nth(4) {
        Some(command) if command == "POSITION" => {
            match args().nth(5) {
                Some(position) => {
                    // Open socket
                    let socket = match UdpSocket::bind("127.0.0.1:6456") {
                        Ok(s) => s,
                        Err(e) => {
                            panic!("Couldn't bind to 127.0.0.1:6456: {:?}", e);
                        }
                    };

                    // Make an integer out of the position
                    let position: i32 = match position.parse() {
                        Ok(p) => p,
                        Err(e) => {
                            panic!("Could not parse position: {:?}", e);
                        }
                    };

                    // Send position to dragonblaze
                    match socket.send_to(&position.to_be_bytes(), "127.0.0.1:6455") {
                        Ok(_) => println!("Sent position {} to dragonblaze", position),
                        Err(e) => {
                            panic!("Error sending position {} to dragonblaze {}", position, e)
                        }
                    };
                }
                None => println!("No 5th argument present"),
            }
        }
        Some(command) => println!("Ignoring {}", command),
        None => println!("No 4th argument present"),
    }
}
