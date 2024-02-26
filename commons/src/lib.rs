use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct OauthData {
    pub client_id: String,
    pub scope: String,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCode {
    pub code: String,
    pub state: String,
}