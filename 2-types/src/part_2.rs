use serde::{Deserialize, Serialize};
use serde_json;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub r#type: String,
    pub stream: Stream,
    pub gifts: Vec<Gift>,
    pub debug: DebugInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stream {
    pub user_id: String,
    pub is_private: bool,
    pub settings: u64,
    pub shard_url: String,
    pub public_tariff: PublicTariff,
    pub private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicTariff {
    pub id: u32,
    pub price: u64,
    pub duration: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateTariff {
    pub client_price: u64,
    pub duration: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gift {
    pub id: u32,
    pub price: u64,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DebugInfo {
    pub duration: String,
    pub at: String,
}

pub fn json_to_toml(json: &str) -> String {
    let request: Request = serde_json::from_str(json).unwrap();
    toml::to_string_pretty(&request).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_toml() {
        let json = r#"
        {
            "type": "success",
            "stream": {
                "user_id": "abc",
                "is_private": false,
                "settings": 123,
                "shard_url": "url",
                "public_tariff": {
                    "id": 1,
                    "price": 100,
                    "duration": "1h",
                    "description": "test"
                },
                "private_tariff": {
                    "client_price": 200,
                    "duration": "1m",
                    "description": "test"
                }
            },
            "gifts": [
                {
                    "id": 1,
                    "price": 10,
                    "description": "gift"
                }
            ],
            "debug": {
                "duration": "100ms",
                "at": "time"
            }
        }
        "#;

        let toml_str = json_to_toml(json);

        assert!(toml_str.contains("type = \"success\""));
        assert!(toml_str.contains("user_id = \"abc\""));
    }
}
