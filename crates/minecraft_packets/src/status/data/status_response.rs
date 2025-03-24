use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerSample {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Option<Vec<PlayerSample>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    pub version: Version,
    pub players: Players,
    pub description: Value,
    pub favicon: Option<String>,
    #[serde(
        alias = "enforcesSecureChat",
        default = "get_default_enforces_secure_chat"
    )]
    pub enforces_secure_chat: bool,
}

fn get_default_enforces_secure_chat() -> bool {
    false
}

impl StatusResponse {
    pub fn new(
        version_name: &str,
        version_protocol: u32,
        description_text: &str,
        enforces_secure_chat: bool,
    ) -> Self {
        let mut description_map = serde_json::Map::new();
        description_map.insert(
            "text".to_string(),
            Value::String(description_text.to_string()),
        );
        let description = Value::Object(description_map);
        StatusResponse {
            version: Version {
                name: version_name.to_string(),
                protocol: version_protocol,
            },
            players: Players {
                max: 1,
                online: 0,
                sample: None,
            },
            description,
            favicon: None,
            enforces_secure_chat,
        }
    }
}
