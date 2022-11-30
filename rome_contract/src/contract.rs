// TO DO
// 1) Actually delete deleted posts
// 2) format profile names
// 3) functions to replace check profile name & profile

use cosmwasm_std::{
    coin, entry_point, to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order,
    Response, StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Bound;
use is_false::is_false;
use std::{env, vec};

use crate::coin_helpers::assert_sent_exact_coin;
use crate::error::ContractError;
use crate::msg::{
    AllPostsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse, QueryMsg,
};
use crate::state::{
    Config, Post, Profile, ProfileName, ADDR_LOOKUP, CONFIG, LAST_POST_ID, POST, PROFILE,
    PROFILE_NAME,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const ADDRESS: &str = "juno1ggtuwvungvx5t3awqpcqvxxvgt7gvwdkanuwtm";
const ADMIN: &str = "juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4";
const MAX_ID_LENGTH: usize = 128;
const MAX_TEXT_LENGTH: usize = 499;
const IPFS: &str = "https://alxandria.infura-ipfs.io/ipfs/";
const JUNO: &str = "ujunox";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let validated_admin = deps.api.addr_validate(ADMIN)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    LAST_POST_ID.save(deps.storage, &0)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterProfileName { profile_name } => {
            execute_register_profile_name(deps, env, info, profile_name)
        }
        ExecuteMsg::CreateProfile {
            bio,
            profile_picture,
            cover_picture,
        } => execute_create_profile(deps, env, info, bio, profile_picture, cover_picture),
        ExecuteMsg::CreatePost {
            editable,
            post_title,
            external_id,
            text,
            tags,
        } => execute_create_post(
            deps,
            env,
            info,
            editable,
            post_title,
            external_id,
            text,
            tags,
        ),
        ExecuteMsg::EditPost {
            post_id,
            external_id,
            text,
            tags,
        } => execute_edit_post(deps, env, info, post_id, external_id, text, tags),
        ExecuteMsg::DeletePost { post_id } => execute_delete_post(deps, env, info, post_id),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env, info),
    }
}
fn execute_register_profile_name(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    profile_name: String,
) -> Result<Response, ContractError> {
    let formatted_profile_name = profile_name.trim().to_lowercase();
    //1) Check to see if there is the desired profile name is registered
    let check = PROFILE.may_load(deps.storage, formatted_profile_name.clone())?;
    match check {
        Some(_check) => Err(ContractError::ProfileNameTaken {
            taken_profile_name: formatted_profile_name,
        }),
        //2) If profile name isn't registered, save it to account
        None => {
            let new_profile_name: ProfileName = ProfileName {
                profile_name: formatted_profile_name,
                account_address: info.sender,
            };
            PROFILE_NAME.save(
                deps.storage,
                new_profile_name.profile_name.clone(),
                &new_profile_name,
            )?;
            ADDR_LOOKUP.save(
                deps.storage,
                new_profile_name.account_address.clone(),
                &new_profile_name,
            )?;
            Ok(Response::new()
                .add_attribute("action", "create profile name")
                .add_attribute("new profile name", new_profile_name.profile_name))
        }
    }
}
fn execute_create_profile(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    bio: String,
    profile_picture: String,
    cover_picture: String,
) -> Result<Response, ContractError> {
    //query profile name and ensure it is registered to the transactor
    let profile_name_check = ADDR_LOOKUP.may_load(deps.storage, info.sender.clone())?;
    match profile_name_check {
        //if there is a profile name, save the profile and store same profile name to profile
        Some(profile_name_check) => {
            let new_profile: Profile = Profile {
                profile_name: profile_name_check.profile_name,
                bio,
                profile_picture,
                cover_picture,
                account_address: info.sender,
            };
            PROFILE.save(
                deps.storage,
                new_profile.profile_name.to_string(),
                &new_profile,
            )?;
            Ok(Response::new().add_attribute("action", "create profile"))
        }
        None => Err(ContractError::NeedToRegisterProfileName {}),
    }
}
//clippy defaults to max value of 7
#[allow(clippy::too_many_arguments)]
fn execute_create_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    editable: bool,
    post_title: String,
    external_id: String,
    text: String,
    tags: Vec<String>,
) -> Result<Response, ContractError> {
    if text.len() > MAX_TEXT_LENGTH {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > MAX_ID_LENGTH {
        return Err(ContractError::OnlyOneLink {});
    }
    if is_false(external_id.starts_with(IPFS)) {
        return Err(ContractError::MustUseAlxandriaGateway {});
    }
    let last_post_id = LAST_POST_ID.load(deps.storage)?;
    let incremented_id = last_post_id + 1;
    //check to see if there is a profile name associated with the wallet
    let profile_name_check = ADDR_LOOKUP.may_load(deps.storage, info.sender)?;
    match profile_name_check {
        //if there is a profile name, search for a profile
        Some(profile_name_check) => {
            let registered_profile_check =
                PROFILE.may_load(deps.storage, profile_name_check.profile_name)?;
            match registered_profile_check {
                //if there is a profile allow the user to create a post
                Some(registered_profile_check) => {
                    let post: Post = Post {
                        editable,
                        post_id: incremented_id,
                        post_title,
                        external_id,
                        text,
                        tags,
                        author: registered_profile_check.profile_name.clone(),
                        creation_date: env.block.time.to_string(),
                        last_edit_date: None,
                        editor: None,
                    };
                    //check to see whether the user elected to make the post editable or not,
                    //this effects the price
                    match post.editable {
                        true => {
                            assert_sent_exact_coin(&info.funds, Some(coin(1_000_000, JUNO)))?;
                            LAST_POST_ID.save(deps.storage, &incremented_id)?;
                            POST.save(deps.storage, post.post_id, &post)?;
                            Ok(Response::new()
                                .add_attribute("action", "create_post")
                                .add_attribute("post_id", post.post_id.to_string())
                                .add_attribute("author", registered_profile_check.profile_name))
                        }
                        false => {
                            //increased fee
                            assert_sent_exact_coin(&info.funds, Some(coin(5_000_000, JUNO)))?;
                            LAST_POST_ID.save(deps.storage, &incremented_id)?;
                            POST.save(deps.storage, post.post_id, &post)?;
                            Ok(Response::new()
                                .add_attribute("action", "create_post")
                                .add_attribute("post_id", post.post_id.to_string())
                                .add_attribute("author", registered_profile_check.profile_name))
                        }
                    }
                }
                None => Err(ContractError::NeedToRegisterProfile {}),
            }
        }
        None => Err(ContractError::NeedToRegisterProfileName {}),
    }
}

fn execute_edit_post(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    post_id: u64,
    external_id: String,
    text: String,
    tags: Vec<String>,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(Coin::new(2_000_000, JUNO)))?;
    if text.len() > MAX_TEXT_LENGTH {
        return Err(ContractError::TooMuchText {});
    }
    if external_id.len() > MAX_ID_LENGTH {
        return Err(ContractError::OnlyOneLink {});
    }
    if is_false(external_id.starts_with(IPFS)) {
        return Err(ContractError::MustUseAlxandriaGateway {});
    }
    //check to see if there is a profile name associated with the wallet
    let profile_name_check = ADDR_LOOKUP.may_load(deps.storage, info.sender.clone())?;
    match profile_name_check {
        //if there is a profile name, search for a profile
        Some(profile_name_check) => {
            let registered_profile_check =
                PROFILE.may_load(deps.storage, profile_name_check.profile_name)?;
            match registered_profile_check {
                //if there is a profile, load the original post
                Some(registered_profile_check) => {
                    let post = POST.load(deps.storage, post_id)?;
                    let editable_post = post.editable;
                    //If post is editable, allow user to edit
                    match editable_post {
                        true => {
                            let new_post: Post = Post {
                                editable: post.editable,
                                post_id: post.post_id,
                                post_title: post.post_title,
                                external_id,
                                text,
                                tags,
                                author: post.author.clone(),
                                creation_date: post.creation_date,
                                last_edit_date: Some(env.block.time.to_string()),
                                editor: Some(registered_profile_check.profile_name),
                            };
                            POST.save(deps.storage, post_id, &new_post)?;
                            //original author profile is searched based on stored profile name
                            let original_author_lookup =
                                PROFILE_NAME.load(deps.storage, post.author)?;
                            //wallet address is retrieved from profile name map
                            let original_author_address = original_author_lookup.account_address;
                            //fund share is sent to original author
                            let share = BankMsg::Send {
                                to_address: original_author_address.to_string(),
                                amount: vec![coin(500_000, JUNO)],
                            };
                            Ok(Response::new()
                                .add_message(share)
                                .add_attribute("action", "edit_post")
                                .add_attribute("post_id", new_post.post_id.to_string())
                                .add_attribute("editor", new_post.editor.unwrap()))
                        }
                        //if post is not editable, see if sender is original author
                        false => {
                            if info.sender == post.author {
                                let new_post: Post = Post {
                                    editable: post.editable,
                                    post_id: post.post_id,
                                    post_title: post.post_title,
                                    external_id,
                                    text,
                                    tags,
                                    author: post.author.clone(),
                                    creation_date: post.creation_date,
                                    last_edit_date: Some(env.block.time.to_string()),
                                    editor: Some(registered_profile_check.profile_name),
                                };
                                POST.save(deps.storage, post_id, &new_post)?;
                                //original author profile is searched based on stored profile name
                                let original_author_lookup =
                                    PROFILE_NAME.load(deps.storage, post.author)?;
                                //wallet address is retrieved from profile name map
                                let original_author_address =
                                    original_author_lookup.account_address;
                                //fund share is sent to original author
                                let share = BankMsg::Send {
                                    to_address: original_author_address.to_string(),
                                    amount: vec![coin(500_000, JUNO)],
                                };
                                Ok(Response::new()
                                    .add_message(share)
                                    .add_attribute("action", "edit_post")
                                    .add_attribute("post_id", new_post.post_id.to_string())
                                    .add_attribute("editor", new_post.editor.unwrap()))
                            } else {
                                Err(ContractError::UnauthorizedEdit {})
                            }
                        }
                    }
                }
                None => Err(ContractError::NeedToRegisterProfile {}),
            }
        }
        None => Err(ContractError::NeedToRegisterProfileName {}),
    }
}
fn execute_delete_post(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    post_id: u64,
) -> Result<Response, ContractError> {
    assert_sent_exact_coin(&info.funds, Some(Coin::new(10_000_000, JUNO)))?;
    let post = POST.may_load(deps.storage, post_id)?;
    match post {
        Some(post) => {
            //see if sender is original author
            let author = ADDR_LOOKUP.may_load(deps.storage, info.sender.clone())?;
            match author {
                Some(author) => {
                    if author.profile_name == post.author {
                        POST.remove(deps.storage, post_id);
                        Ok(Response::new()
                        .add_attribute("delete post", post_id.to_string()))
                    }
                    else {
                        println!("{}", post.author);
                        println!("{}", info.sender);
                        return Err(ContractError::UnauthorizedEdit {  });
                    }
                }
                None => Err(ContractError::NeedToRegisterProfileName {  })
                }
            } 
        None => Err(ContractError::PostDoesNotExist {  })
    }
}
fn execute_withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    //see if transactor is administrative wallet, fail if not
    if info.sender != ADMIN {
        return Err(ContractError::Unauthorized {});
    }
    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    let bank_msg = BankMsg::Send {
        to_address: ADDRESS.to_string(),
        amount: balance,
    };

    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw");
    Ok(resp)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPosts { limit, start_after } => query_all_posts(deps, env, limit, start_after),
        QueryMsg::Post { post_id } => query_post(deps, env, post_id),
    }
}

//pagination
const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

fn query_all_posts(
    deps: Deps,
    _env: Env,
    limit: Option<u32>,
    start_after: Option<u64>,
) -> StdResult<Binary> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(Bound::exclusive);
    let posts = POST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPostsResponse { posts })
}

fn query_post(deps: Deps, _env: Env, post_id: u64) -> StdResult<Binary> {
    let post = POST.may_load(deps.storage, post_id)?;
    to_binary(&PostResponse { post })
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = get_contract_version(deps.storage)?;
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    //canonical way from official docs
    #[allow(clippy::cmp_owned)]
    if ver.version > (*CONTRACT_VERSION).to_string() {
        return Err(StdError::generic_err("Must upgrade from a lower version").into());
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default()
        .add_attribute("action", "migration")
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("contract", CONTRACT_NAME))
}
