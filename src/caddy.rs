use {
    serde::{ser::SerializeStruct, Deserialize, Serialize},
    serde_json::{to_value, Value},
    std::collections::HashMap,
};

const CADDY_API: &str = "http://127.0.0.1:2019/";

#[derive(Serialize, Deserialize)]
pub struct CaddyConfig {
    apps: CaddyHttpServers,
}

#[derive(Serialize, Deserialize)]
pub struct CaddyHttpServers {
    http: CaddyServerMappding,
}

#[derive(Serialize, Deserialize)]
pub struct CaddyServerMappding {
    servers: HashMap<String, CaddyServer>,
}

#[derive(Serialize, Deserialize)]
pub struct CaddyServer {
    listen: Vec<String>,
    routes: Vec<CaddyRoute>,
}

#[derive(Serialize, Deserialize)]
pub struct CaddyRoute {
    // rename match
    #[serde(rename = "match")]
    json_match: Vec<CaddyMatch>,
    handle: Vec<Value>,
    terminal: bool,
}

pub type CaddyMatch = HashMap<String, Vec<String>>;

pub enum CaddyHandle {
    Body(BodyHandle),
}

impl Serialize for CaddyHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Body(payload) => {
                let mut s = serializer.serialize_struct("Hello::D", 2)?;
                s.serialize_field("body", &payload.content)?;
                s.serialize_field("handler", "static_response")?;
                s.end()
            }
        }
    }
}

pub struct BodyHandle {
    pub content: String,
}

pub fn new_server_config() -> CaddyConfig {
    let mut servers = HashMap::new();
    let handle_body = BodyHandle {
        content: "Hello World".to_string(),
    };

    servers.insert(
        "name".to_string(),
        CaddyServer {
            listen: vec![":80".to_string()],
            routes: vec![CaddyRoute {
                json_match: vec![],
                handle: vec![to_value(CaddyHandle::Body(handle_body)).unwrap()],
                terminal: true,
            }],
        },
    );

    CaddyConfig {
        apps: CaddyHttpServers {
            http: CaddyServerMappding { servers },
        },
    }
}

pub fn is_caddy_alive() -> bool {
    true
}

// https://caddyserver.com/docs/api#post-load
// curl localhost:2019/load  -H "Content-Type: application/json" -d @caddy.json
