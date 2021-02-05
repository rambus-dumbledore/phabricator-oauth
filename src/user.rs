use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PhabricatorUser {
    pub phid: String,
    #[serde(rename(deserialize = "userName"))]
    pub user_name: String,
    #[serde(rename(deserialize = "realName"))]
    pub real_name: String,
    pub image: String,
    pub uri: String,
    pub roles: Vec<String>,
    #[serde(rename(deserialize = "primaryEmail"))]
    pub primary_email: String
}