use crate::state::Post;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateProfile {
        profile_name: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
    },
    AdminCreateProfile {
        address: String,
        profile_name: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
    },
    CreatePost {
        editable: bool,
        post_title: String,
        external_id: String,
        text: String,
        tags: Vec<String>,
    },
    EditPost {
        post_id: u64,
        external_id: String,
        text: String,
        tags: Vec<String>,
    },
    DeletePost {
        post_id: u64,
    },
    WithdrawJuno {},
    UnlockArticle {
        post_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct AllPostsResponse {
    pub posts: Vec<Post>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct PostResponse {
    pub post: Option<Post>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ArticleCountResponse {
    pub article_count: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProfileNameResponse {
    pub profile_name: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllPosts {
        limit: Option<u32>,
        start_after: Option<u64>,
    },
    Post {
        post_id: u64,
    },
    ArticleCount {},
    ProfileName {
        address: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}
