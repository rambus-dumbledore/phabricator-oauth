use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PhabricatorUser {
    pub phid: String,
    #[serde(rename(serialize = "userName", deserialize = "userName"))]
    pub user_name: String,
    #[serde(rename(serialize = "realName", deserialize = "realName"))]
    pub real_name: String,
    pub image: String,
    pub uri: String,
    pub roles: Vec<String>,
    #[serde(rename(serialize = "primaryEmail", deserialize = "primaryEmail"))]
    pub primary_email: String
}