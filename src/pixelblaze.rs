use std::sync::Mutex;

use serde_json::{json, Map, Value};
use websocket::client::sync::Client;
use websocket::client::ClientBuilder;
use websocket::sync::stream::TcpStream;
use websocket::Message;

use anyhow::Result;

use log::*;

pub struct PixelblazeClient {
    client: Mutex<Client<TcpStream>>,
}

impl PixelblazeClient {
    pub fn new(pixelblaze_ip: &str) -> Result<Self> {
        let pixelblaze_url = format!("ws://{}:81", pixelblaze_ip);
        let pixelblaze_client = ClientBuilder::new(&pixelblaze_url)?
            .add_protocol("rust-websocket")
            .connect_insecure()?;
        debug!("Connected to pixelblaze at {}", pixelblaze_url);
        Ok(Self {
            client: Mutex::new(pixelblaze_client),
        })
    }

    pub fn set_variables(&self, variables: Map<String, Value>) -> Result<()> {
        let mut client = self.client.lock().expect("Lock poisoned");
        let message = json!({ "setVars": variables });
        debug!("Sending `{:?}` to pixelblaze", message);
        client.send_message(&Message::text(message.to_string()))?;
        Ok(())
    }

    pub fn set_brightnes(&self, brightness: f64) -> Result<()> {
        let mut client = self.client.lock().expect("Lock poisoned");
        let message = json!({ "brightness": brightness, "save": false });
        debug!("Sending `{:?}` to pixelblaze", message);
        client.send_message(&Message::text(message.to_string()))?;
        Ok(())
    }
}
