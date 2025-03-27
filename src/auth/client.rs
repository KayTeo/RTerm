#[derive(Debug)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl ClientCredentials {
    pub fn new(client_id: String, client_secret: String) -> Self {
        ClientCredentials {
            client_id,
            client_secret,
        }
    }
}