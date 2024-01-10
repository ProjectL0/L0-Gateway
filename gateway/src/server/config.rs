use super::DEFAULT_LISTEN_PORT;

#[derive(Clone)]
pub struct ServerConfig {
    /// Port for client-server connection.
    pub listen_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { listen_port: DEFAULT_LISTEN_PORT }
    }
}
