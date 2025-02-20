use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub admin: Addr,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    pub profile_name: String,
    pub bio: String,
    pub profile_picture: String,
    pub cover_picture: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Post {
    pub editable: bool,
    //tracks specific posts through unique identifier
    pub post_id: u64,
    //title for FE searches
    pub post_title: String,
    //ipfs link
    pub external_id: String,
    //store summary of article / edits
    pub text: String,
    pub tags: Vec<String>,
    pub author: String,
    pub creation_date: String,
    pub last_edit_date: Option<String>,
    pub editor: Option<String>,
}

pub const CONFIG: Item<Config> = Item::new("config");
//create a map of post. Addr is creator. u64 is post_id
pub const POST: Map<u64, Post> = Map::new("post");
pub const LAST_POST_ID: Item<u64> = Item::new("last_post_id");
pub const PROFILE: Map<Addr, Profile> = Map::new("profile");
pub const PROFILE_LOOKUP: Map<String, Addr> = Map::new("profile_name");
pub const ARTICLE_COUNT: Item<u64> = Item::new("number_of_articles");
