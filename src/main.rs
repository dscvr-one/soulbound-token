mod guards;
mod http;
mod hydrate;
mod service_controller;
mod soulbound_token;
mod state;

pub mod prelude {
    pub use candid::Principal;
    pub use ic_cdk::export::candid::CandidType;
    pub use ic_cdk_macros::*;
    pub use serde::{Deserialize, Serialize};
}

use crate::guards::*;
use crate::http::*;
use crate::service_controller::ServiceControllerKind;
use crate::state::State;
use prelude::*;
use service_controller::ServiceController;
use url::Url;

fn main() {}

#[init]
fn init() {
    State::mutate_state(|state| {
        // Owner Service Account
        state.add_owner(ic_cdk::api::caller());
    });
}

#[query]
fn test() -> String {
    let s = String::from("Hello, world!");
    ic_cdk::println!("{}", s);
    s
}

#[update(guard = "is_admin")]
fn mint(principal: Principal) -> Result<(), String> {
    State::mutate_state(|state| state.mint_soulbound_token(principal, "sns-1-image".to_string()))
}

#[update(guard = "is_admin")]
fn mint_many(principals: Vec<Principal>) -> Vec<Result<Principal, String>> {
    State::mutate_state(|state| state.mint_soulbound_tokens(principals, "sns-1-image".to_string()))
}

#[query]
fn get_registry() -> Vec<(Principal, Vec<u64>)> {
    State::read_state(|state| state.get_registry())
}

#[query]
fn soulbound_tokens(user: Principal) -> Vec<u64> {
    State::read_state(|state| state.get_user_registry(user))
}

#[update(guard = "is_admin")]
fn add_asset(asset_name: String, asset: Vec<u8>) -> Result<(), String> {
    State::mutate_state(|state| {
        state.set_image(asset_name, asset);
        Ok(())
    })
}

#[query]
fn get_asset(asset_name: String) -> Result<Vec<u8>, String> {
    State::read_state(|state| state.get_image(&asset_name).map(|bytes| bytes.to_vec()))
}

#[update(guard = "is_admin")]
fn clear_asset(asset_name: String) {
    State::mutate_state(|state| {
        state.clear_image(asset_name);
    })
}

#[update(guard = "is_admin")]
fn append_asset(asset_name: String, asset: Vec<u8>) -> Result<(), String> {
    State::mutate_state(|state| {
        state.append_image_bytes(asset_name, asset);
        Ok(())
    })
}

#[query]
fn get_admins() -> Vec<Principal> {
    State::read_state(|state| state.get_admins())
}

#[query]
fn get_service_controllers() -> Vec<ServiceController> {
    State::read_state(|state| state.get_service_controllers().clone())
}

#[update(guard = "is_owner")]
fn add_admin(principal: Principal) -> Result<(), String> {
    if State::mutate_state(|state| state.add_admin(principal)) {
        Ok(())
    } else {
        Err(format!(
            "The pair Principal: {:?}, ServiceControllerKind: {:?} already exists.  Failed to add.",
            principal,
            ServiceControllerKind::Admin
        ))
    }
}

#[update(guard = "is_owner")]
fn remove_admin(principal: Principal) -> Result<(), String> {
    if State::mutate_state(|state| state.remove_admin(&principal)) {
        Ok(())
    } else {
        Err(format!(
            "The pair Principal: {:?}, ServiceControllerKind: {:?} already exists.  Failed to add.",
            principal,
            ServiceControllerKind::Admin
        ))
    }
}

#[query]
async fn http_request(req: HttpRequest) -> HttpResponse {
    let qualified_url = format!("https://{}.ic0.app{}", ic_cdk::api::id(), req.url);
    ic_cdk::println!("qualified_url: {:?}", qualified_url);
    let url = match Url::parse(&qualified_url) {
        Ok(url) => url,
        Err(_e) => {
            return HttpResponse {
                status_code: HttpStatus::BadRequest as u16,
                headers: vec![],
                body: format!(
                    "Invalid request URL.  Request URL: {:?}, Qualified URL: {:?}",
                    req.url, qualified_url
                )
                .as_bytes()
                .to_vec(),
            };
        }
    };
    ic_cdk::println!("url: {:?}", url);

    if let Some(params) = url.query() {
        let params = params.split('=').collect::<Vec<&str>>();
        let id;
        if params[0] == "id" {
            match params[1].parse::<u64>() {
                Ok(param_id) => id = param_id,
                Err(_e) => return HttpResponse {
                    status_code: HttpStatus::BadRequest as u16,
                    headers: vec![],
                    body: format!("Invalid query params.  Expected \"id=u64\" but got: {:?}.\nCheck formatting and resubmit your request.", params).as_bytes().to_vec(),
                }
            };
        } else {
            return HttpResponse {
                status_code: HttpStatus::BadRequest as u16,
                headers: vec![],
                body: format!("Invalid query params.  Expected \"id=u64\" but got: {:?}.\nCheck formatting and resubmit your request.", params).as_bytes().to_vec(),
            };
        };

        State::read_state(|state| match state.get_sbt_image(id) {
            Ok(image) => {
                ic_cdk::println!("{:?}", req);

                let headers = vec![
                    ("content-type".to_string(), "image/png".to_string()),
                    ("cache-control".to_string(), "no-cache".to_string()),
                ];

                HttpResponse {
                    status_code: HttpStatus::Ok as u16,
                    headers,
                    body: image.to_vec(),
                }
            }
            Err(e) => HttpResponse {
                status_code: HttpStatus::NotFound as u16,
                headers: vec![],
                body: e.as_bytes().to_vec(),
            },
        })
    } else {
        HttpResponse {
            status_code: HttpStatus::BadRequest as u16,
            headers: vec![],
            body: "Invalid request.  Query params are missing.  Resubmit with query in format: \"?id=u64\""
                .as_bytes()
                .to_vec(),
        }
    }
}
