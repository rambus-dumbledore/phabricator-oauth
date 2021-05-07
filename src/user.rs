use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhabricatorUser {
    pub phid: String,
    pub user_name: String,
    pub real_name: String,
    pub image: String,
    pub uri: String,
    pub roles: Vec<String>,
    pub primary_email: String
}