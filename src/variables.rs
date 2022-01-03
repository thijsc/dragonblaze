use artnet_protocol::PaddedData;
use serde_json::{Map, Number, Value};

/// Keeps track of artnet messages to be able to produce a set
/// of variable updates for the Pixelblaze.
///
/// There is no way to know which channels are in use, so we remember
/// which positions had a value above zero and set those back to zero
/// when needed.
///
/// The first lightning channel in DragonFrame is mapped to `brightness`,
/// other variables are named `channel_02` and up.

pub struct VariableStore {
    last_data: Option<Vec<u8>>,
}

impl VariableStore {
    pub fn new() -> Self {
        Self { last_data: None }
    }

    /// Process ArtNet data that was sent and return the wanted brightness
    /// and variables to send to the Pixelblaze
    pub fn process(&mut self, data: &PaddedData) -> (Option<f64>, Map<String, Value>) {
        let mut out = Map::new();
        let mut brightness = None;

        // Get the vector of values
        let data = data.as_ref().clone();

        // Loop through them and add them output if needed
        for (position, value) in data.iter().enumerate() {
            // Calculate the variable
            let variable = if *value == 0 {
                // Value is zero, check if we had this value
                // in the previous data. If not skip this one.
                match self.last_data {
                    Some(ref last_data) => match last_data.get(position) {
                        Some(v) if *v > 0 => 0.0,
                        Some(_) | None => continue,
                    },
                    None => continue,
                }
            } else {
                convert_value(*value)
            };

            // Create the key
            if position == 0 {
                brightness = Some(variable);
            } else {
                let key = format!("channel_{}", position + 1);
                // Insert into the map
                out.insert(
                    key,
                    Value::Number(Number::from_f64(variable).expect("Could not convert f64")),
                );
            };
        }

        // Store the data so we can do lookups next time
        self.last_data = Some(data);

        // Return the brightness and variables
        (brightness, out)
    }
}

fn convert_value(value: u8) -> f64 {
    let divider = 1.0 / 255.0;
    value as f64 * divider
}

#[cfg(test)]
mod tests {
    use super::*;
    use artnet_protocol::PaddedData;

    #[test]
    fn test_convert_value() {
        assert_eq!(1.0, convert_value(255));
        assert_eq!(0.0, convert_value(0));
        assert!(convert_value(128) > 0.49);
        assert!(convert_value(128) < 0.51);
    }

    #[test]
    fn test_variables() {
        let mut store = VariableStore::new();

        // Artnet message with all zeroes
        let data = PaddedData::from(vec![]);
        let (brightness, variables) = store.process(&data);
        assert!(brightness.is_none());
        assert!(variables.is_empty());

        // We're using whole numbers to not have to deal with rounding
        // differences in this test

        // Message with channels that values above zero. Have a zero
        // value in between two used ones
        let data = PaddedData::from(vec![255, 255, 0, 255, 0, 0, 0, 0]);
        let (brightness, variables) = store.process(&data);
        assert_eq!(Some(1.0), brightness);
        let mut expected = Map::new();
        helpers::insert_variable(&mut expected, "channel_2", 1.0);
        helpers::insert_variable(&mut expected, "channel_4", 1.0);
        assert_eq!(expected, variables);

        // Get a zero value for everything but channel 1. We want to reset
        // the variables we saw earlier to zero.
        let data = PaddedData::from(vec![255, 255, 0, 0, 0, 0, 0]);
        let (brightness, variables) = store.process(&data);
        assert_eq!(Some(1.0), brightness);
        let mut expected = Map::new();
        helpers::insert_variable(&mut expected, "channel_2", 1.0);
        helpers::insert_variable(&mut expected, "channel_4", 0.0);
        assert_eq!(expected, variables);

        // Next time we don't need to reset any variables
        let data = PaddedData::from(vec![]);
        let (brightness, variables) = store.process(&data);
        assert!(brightness.is_none());
        assert!(variables.is_empty());
    }

    mod helpers {
        use super::*;

        pub fn insert_variable(map: &mut Map<String, Value>, key: &str, value: f64) {
            map.insert(
                key.to_owned(),
                Value::Number(Number::from_f64(value).expect("Could not convert f64")),
            );
        }
    }
}
