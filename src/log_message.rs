use anyhow::Error;
use serde_json::{json, Value};

pub struct LogMessage {
    msg: Vec<Value>,
}

impl LogMessage {
    fn get_value_from_unparsed(data: &str) -> Value {
        let mut map = serde_json::Map::new();

        map.insert("msg".to_string(), Value::String(data.to_string()));

        Value::Object(map)
    }

    pub fn enrich(&mut self, key: &str, value: &str) -> Result<(), Error> {
        for m in self.msg.iter_mut() {
            match m {
                Value::Object(obj) => {
                    obj.entry(key).or_insert(json!(value));
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Failed to enrich message: {}",
                        self.to_string()
                    ))
                }
            }
        }

        Ok(())
    }

    pub fn enrich_with_timestamp(&mut self, timestamp: &str) -> Result<(), Error> {
        for m in self.msg.iter_mut() {
            match m {
                Value::Object(obj) => {
                    if let Some((_, v)) = obj.remove_entry("timestamp") {
                        obj.insert("@timestamp".to_string(), v);
                    } else {
                        obj.entry("@timestamp").or_insert(json!(timestamp));
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!(
                        "Failed to enrich message: {}",
                        self.to_string()
                    ))
                }
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = self
            .msg
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", res)?;

        Ok(())
    }
}

impl From<&str> for LogMessage {
    fn from(data: &str) -> Self {
        let mut msg = Vec::new();

        let result = serde_json::from_str(data);

        match result {
            Ok(value) => match value {
                Value::Object(v) => msg.push(Value::Object(v)),
                Value::Array(arr) => {
                    for v in arr {
                        match v {
                            Value::Object(obj) => msg.push(Value::Object(obj)),
                            _ => msg
                                .push(LogMessage::get_value_from_unparsed(v.to_string().as_str())),
                        }
                    }
                }
                _ => msg.push(LogMessage::get_value_from_unparsed(data)),
            },
            Err(_) => msg.push(LogMessage::get_value_from_unparsed(data)),
        };

        LogMessage { msg }
    }
}
