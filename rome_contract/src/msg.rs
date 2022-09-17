use cosmwasm_std::Uint64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use desmos_bindings::types::PageRequest;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatePost{
        post_id: u64,
        external_id: String,
        tags: Vec<String>,
        text: Option<String>,
        author: String,
    },
    EditPost{
        post_id: u64,
        external_id: String,
        text: Option<String>,
        tags: Vec<String>,
        author: String,
        editor: String,
        creation_date: String,
        last_edit_date: String,
    },
    DeletePost{
        post_id: u64,
        external_id: String,
        text: Option<String>,
        tags: Vec<String>,
        author: String,
        creation_date: String,
        last_edit_date: Option<String>,
        deleter: Option<String>,
        editor: Option<String>
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Post{
        subspace_id: Uint64,
        post_id: Uint64,
    },
    PostAttachments{
        subspace_id: Uint64,
        post_id: Uint64,
        pagination: Option<PageRequest>,
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomResponse {
    val: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {}