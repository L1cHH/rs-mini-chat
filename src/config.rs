pub struct ServerConfig {
    pub addr: String,
    pub port: u32
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            addr: "0.0.0.0".to_string(),
            port: 8080
        }
    }
}

impl ServerConfig {
    pub fn new(addr: String, port: u32) -> Self {
        ServerConfig {
            addr,
            port
        }
    }
}