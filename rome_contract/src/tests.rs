use crate::contract::{execute, instantiate, migrate, query};
use crate::msg::{
    AllPostsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PostResponse, QueryMsg, ProfileNameResponse, ArticleCountResponse,
};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{attr, coin, from_binary, Response};

pub const ADDR1: &str = "juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4";
pub const ADDR2: &str = "juno1ggtuwvungvx5t3awqpcqvxxvgt7gvwdkanuwtm";

#[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    assert_eq!(
        res.attributes,
        vec![attr("action", "instantiate"), attr("admin", ADDR1)]
    )
}

#[test]
fn test_instantiate_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR2, &[]);

    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _err = instantiate(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn migrate_works() {
    //instantiate
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    //migrate
    let msg = MigrateMsg {};
    let _res: Response = migrate(deps.as_mut(), mock_env(), msg).unwrap();
}

#[test]
fn test_execute_create_post_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "Vitalik".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set proper fee in info for post creation
    let info = mock_info(ADDR1, &[coin(5_000_000, "ujunox")]);
    //create new post
    let msg = ExecuteMsg::CreatePost {
        editable: false,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "Hi".to_string(),
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_admin_create_profile() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::AdminCreateProfile {
        address: ADDR2.to_string(),
        profile_name: "Vitalik".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_create_post_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //create profile without a profile
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id: "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "This will fail".to_string(),
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "Vital ik".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //create new post with wrong URL to fail
    let msg = ExecuteMsg::CreatePost {
            editable: true,
            post_title: "Mintscan Prop 320".to_string(),
            //wrong URL
            external_id: "https://alxandri.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
            tags: vec![
                "Blockchain".to_string(),
                "Governance".to_string(),
                "Rejected".to_string(),
            ],
            text: "This will fail".to_string(),
        };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    //create new post with too long url
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        //wrong URL
        external_id: "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "This will fail".to_string(),
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    //create new post with text over 500 characters
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id: "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        //text over 500 characters
        text: "This will fail vdfjkvjdfnksvkndsvjsndjkvnkjfnvnsdjkvnsdfnvjkdfnsvnjdksnvkldsnvjkdfnvjkfdnvkdnfjvkndjsknvjksdnknjfknvjkdsfnjvknskdnvjkndsjkvsjkdnvjksdfnvjksdfnvjkdfsnjvksvndfjkvnjsdkfnvjksdfnvkjlsdfvjnldsfknvjkdsvnjdksjkvcjkdnkm dkfs vkdnjkvndfkjsvjkfdnvjksdfnjkvkdfnvdnskvnsdfvjkdsnvjkdfnvjkdnvjksdnvjkdsvnjkdfnsdvfdknvjksdnvjfkdsnvjkdfsnvjksdnvjkfdsnvjkdsvlnsjknvjkdsnvjksdfnvkndsfjkvnjdskvnksdflvnjdknvjksdnvjkdfsnvjkdsnvjksdnvkdsnvfjkdnvjkdnvjkfndsvkdsfnjvksdnvsdfjklnvjdkslnvjdksnvjdfknvsdfjklnvdjksfnvjkdlsfnvkd".to_string(),
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_execute_edit_post_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate contract
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "satoshi".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set funds in info to pay for interaction
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //create a post
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //edit message
    let info = mock_info(ADDR1, &[coin(2_000_000, "ujunox")]);
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        text: "".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_edit_post_author_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate contract
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "satoshi".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set funds in info to pay for interaction
    let info = mock_info(ADDR1, &[coin(5_000_000, "ujunox")]);
    //create a post
    let msg = ExecuteMsg::CreatePost {
        editable: false,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //edit message
    let info = mock_info(ADDR1, &[coin(2_000_000, "ujunox")]);
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        text: "".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_execute_edit_post_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "satonkers".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set funds for post creation
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //create post
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //edit post without updating funds, will fail for incorrect funds
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id: "https://stake.tax/".to_string(),
        text: "edited post".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _err = execute(deps.as_mut(), env.clone(), info, msg).unwrap_err();
    // edit post with wrong external ID
    let info = mock_info(ADDR1, &[coin(2_000_000, "ujunox")]);
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id: "https://stake.tax/".to_string(),
        text: "edited post".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    // edit post with too much text
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id: "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
        text: "This will fail vdfjkvjdfnksvkndsvjsndjkvnkjfnvnsdjkvnsdfnvjkdfnsvnjdksnvkldsnvjkdfnvjkfdnvkdnfjvkndjsknvjksdnknjfknvjkdsfnjvknskdnvjkndsjkvsjkdnvjksdfnvjksdfnvjkdfsnjvksvndfjkvnjsdkfnvjksdfnvkjlsdfvjnldsfknvjkdsvnjdksjkvcjkdnkm dkfs vkdnjkvndfkjsvjkfdnvjksdfnjkvkdfnvdnskvnsdfvjkdsnvjkdfnvjkdnvjksdnvjkdsvnjkdfnsdvfdknvjksdnvjfkdsnvjkdfsnvjksdnvjkfdsnvjkdsvlnsjknvjkdsnvjksdfnvkndsfjkvnjdskvnksdflvnjdknvjksdnvjkdfsnvjkdsnvjksdnvkdsnvfjkdnvjkdnvjkfndsvkdsfnjvksdnvsdfjklnvjdkslnvjdksnvjdfknvsdfjklnvdjksfnvjkdlsfnvkd".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
    //edit post with too long url
    let msg = ExecuteMsg::EditPost {
        post_id: 1,
        external_id: "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTQmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTQmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvTQmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT".to_string(),
        text: "edited post".to_string(),
        tags: vec!["Tax".to_string(), "Website".to_string()],
    };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_execute_delete_post_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "destuct".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set info with funds for article creation
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //create a post
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set info for funds for post deletion
    let info = mock_info(ADDR1, &[coin(10_000_000, "ujunox")]);
    //delete post
    let msg = ExecuteMsg::DeletePost { post_id: 1 };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //query deleted post
    let msg = QueryMsg::Post { post_id: 1 };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    assert!(res.post.is_none());
}
#[test]
fn test_execute_delete_post_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "domination".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set funds for post creation
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    //create post
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //delete post without updating funds, will fail for incorrect funds
    let msg = ExecuteMsg::DeletePost { post_id: 3 };
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_execute_delete_post_uneditable() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "savagery".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set info for proper funds to create a post that is uneditable
    let info = mock_info(ADDR1, &[coin(5_000_000, "ujunox")]);
    //create a post that is uneditable
    let msg = ExecuteMsg::CreatePost {
        editable: false,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //attempt to delete message by non-original-author (fail)
    let info = mock_info(ADDR2, &[coin(10_000_000, "ujunox")]);
    let msg = ExecuteMsg::DeletePost { post_id: 1 };
    let _res = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_withdraw_valid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "annihilation".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let msg = ExecuteMsg::WithdrawJuno {};
    let _res = execute(deps.as_mut(), env, info, msg).unwrap();
}
#[test]
fn test_withdraw_invalid() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "after".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR2, &[]);
    let msg = ExecuteMsg::WithdrawJuno {};
    let _res = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_query_all_posts() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "MoN A R".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Google.com".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec!["Search".to_string(), "Google".to_string()],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let msg = QueryMsg::AllPosts {
        limit: None,
        //pagination
        start_after: Some(2),
    };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: AllPostsResponse = from_binary(&bin).unwrap();
    //checks pagination
    assert_eq!(res.posts.len(), 1);
}
#[test]
fn test_query_post() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //intantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "donkey".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    let info = mock_info(ADDR1, &[coin(1_000_000, "ujunox")]);
    let msg = ExecuteMsg::CreatePost {
        editable: true,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //query post
    let msg = QueryMsg::Post { post_id: 1 };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    assert!(res.post.is_some());
    //query nonexistent post
    let msg = QueryMsg::Post { post_id: 78476 };
    let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    assert!(res.post.is_none());
    //query article count
    let msg = QueryMsg::ArticleCount {  };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: ArticleCountResponse = from_binary(&bin).unwrap();
    print!("{:?}", res);
    //switch number to 2 to fail
    assert_eq!(1, res.article_count);
}
#[test]
fn test_create_profile() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "finish hu m".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //test query profile
    let msg = QueryMsg::ProfileName { address: ADDR1.to_string() };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: ProfileNameResponse = from_binary(&bin).unwrap();
    print!("{:?}", res);
    //switch to is_none to fail
    assert!(res.profile_name.is_some());
}
#[test]
fn test_register_profile_name_fails() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instantiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "Champ".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set info to different addresss than who has the registered profile name Champ
    let info = mock_info(ADDR2, &[]);
    //attempt to register same profile name
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "Champ".to_string(),
        bio: "anything".to_string(),
        profile_picture: "anything".to_string(),
        cover_picture: "anything".to_string(),
    };
    //expect it to fail due to collision
    let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
}
#[test]
fn test_execute_unlock_article() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADDR1, &[]);
    //instatiate
    let msg = InstantiateMsg {
        admin: ADDR1.to_string(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
    //switch info to ADDR2 for post creation
    let info = mock_info(ADDR2, &[]);
    //register profile
    let msg = ExecuteMsg::CreateProfile {
        profile_name: "savage".to_string(),
        bio: "This is my bio".to_string(),
        profile_picture: "google.com".to_string(),
        cover_picture: "google.com".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //set proper fee in info for post creation
    let info = mock_info(ADDR2, &[coin(5_000_000, "ujunox")]);
    //create new post
    let msg = ExecuteMsg::CreatePost {
        editable: false,
        post_title: "Mintscan Prop 320".to_string(),
        external_id:
            "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT"
                .to_string(),
        tags: vec![
            "Blockchain".to_string(),
            "Governance".to_string(),
            "Rejected".to_string(),
        ],
        text: "Hi".to_string(),
    };
    let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    //test unlocking
    let info = mock_info(ADDR1, &[]);
    let msg = ExecuteMsg::UnlockArticle { post_id: 1 };
    let _res = execute(deps.as_mut(), env.clone(), info, msg);
    //query article for attributes
    let msg = QueryMsg::Post { post_id: 1 };
    let bin = query(deps.as_ref(), env, msg).unwrap();
    let res: PostResponse = from_binary(&bin).unwrap();
    println!("{:?}", res);
    //switch to is_none to intentionally fail and check output to verify editable is true
    assert!(res.post.is_some());
}
