use std::env::args;
use std::net::UdpSocket;
use std::process::exit;
use std::sync::Arc;
use std::thread;

use artnet_protocol::*;
use log::*;
use serde_json::{Map, Number, Value};
use simple_logger;

use dragonblaze::pixelblaze::PixelblazeClient;
use dragonblaze::variables::VariableStore;

/// Daemon that receives ArtNet and position change messages over UDP and
/// updates the state of the Pixelblaze as needed.

fn main() {
    simple_logger::SimpleLogger::new()
        .with_utc_timestamps()
        .init()
        .expect("Could not init log");

    // The first argument contains the IP address of the pixelblaze we
    // are connecting to.
    let pixelblaze_ip = match args().nth(1) {
        Some(url) => url,
        None => {
            error!("Please provide pixelblaze ip as the first argument");
            exit(1);
        }
    };

    // Pixelblaze client
    info!("Connecting to Pixelblaze at {}...", pixelblaze_ip);
    let pixelblaze_client = Arc::new(match PixelblazeClient::new(&pixelblaze_ip) {
        Ok(client) => client,
        Err(e) => {
            error!("Could not create pixelblaze client: {:?}", e);
            exit(1)
        }
    });
    info!("Connection to pixelblaze at {} succeeded", pixelblaze_ip);

    // First UDP socket for artnet messages
    let artnet_socket = match UdpSocket::bind("127.0.0.1:6454") {
        Ok(s) => s,
        Err(e) => {
            error!("Cannot bind to UDP port 6454: {:?}", e);
            exit(1)
        }
    };

    // Second UDP socket for messages from the script
    let position_socket = match UdpSocket::bind("127.0.0.1:6455") {
        Ok(s) => s,
        Err(e) => {
            error!("Cannot bind to UDP port 6455: {:?}", e);
            exit(1)
        }
    };

    // Start a thread to listen to artnet messages and pass those on to the pixelblaze
    let pixelblaze_client_clone = pixelblaze_client.clone();
    let mut variable_store = VariableStore::new();
    let mut buf = [0; 1024];
    thread::spawn(move || loop {
        // Read artnet command from the socket
        let (length, _src_addr) = artnet_socket.recv_from(&mut buf).unwrap();
        let command = ArtCommand::from_buffer(&buf[..length]).unwrap();
        // If this is an output process it using the variable store and then
        // send the result over the socket to the pixelblaze
        match command {
            ArtCommand::Output(ref output) => {
                let (brightness, variables) = variable_store.process(&output.data);
                // Set brightness if needed
                match brightness {
                    Some(b) => match pixelblaze_client_clone.set_brightnes(b) {
                        Ok(_) => (),
                        Err(e) => error!("Could not send mesage to pixelblaze: {:?}", e),
                    },
                    None => (),
                }
                // Set variables
                match pixelblaze_client_clone.set_variables(variables) {
                    Ok(_) => (),
                    Err(e) => error!("Could not send mesage to pixelblaze: {:?}", e),
                }
            }
            command => debug!("Received unsupported ArtNet command: {:?}", command),
        }
    });

    // Pass on position changes
    let mut buffer = [0; 4];
    loop {
        // Read from socket and parse packet as integer
        let (_bytes_read, _src_addr) = match position_socket.recv_from(&mut buffer) {
            Ok((b, r)) => {
                trace!("Received packet on position socket");
                (b, r)
            }
            Err(e) => {
                error!("Error reading UPD packet: {:?}", e);
                continue;
            }
        };

        // Parse the 4 bytes back to an i32
        let position = i32::from_be_bytes(buffer);

        // Send message to the pixelblaze with the updates position
        let mut variables = Map::with_capacity(1);
        variables.insert("position".to_owned(), Value::Number(Number::from(position)));
        match pixelblaze_client.set_variables(variables) {
            Ok(_) => (),
            Err(e) => error!("Could not send mesage to pixelblaze: {:?}", e),
        }
    }
}
