pub(crate) mod stable_storage;

use crate::prelude::*;
use crate::service_controller::{ServiceController, ServiceControllerKind, ServiceControllers};
use crate::soulbound_token::SoulboundToken;
use crate::state::stable_storage::StableStorage;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct State {
    sbt_images: HashMap<String, Vec<u8>>,
    sbt_index: u64,
    // in the future expand this to a Vec<SoulboundToken>
    // to support multi sbt capabilities.
    sbts: HashMap<Principal, SoulboundToken>,
    controllers: ServiceControllers,
}

impl From<StableStorage> for State {
    fn from(storage: StableStorage) -> Self {
        Self {
            sbt_images: storage.sbt_image,
            sbt_index: storage.sbt_index,
            sbts: storage.sbts,
            controllers: storage.controllers,
        }
    }
}

impl State {
    thread_local! {
        pub static STATE: RefCell<State> = RefCell::default();
    }

    pub fn read_state<F: FnOnce(&Self) -> R, R>(f: F) -> R {
        State::STATE.with(|s| f(&s.borrow()))
    }

    pub fn mutate_state<F: FnOnce(&mut Self) -> R, R>(f: F) -> R {
        State::STATE.with(|s| f(&mut s.borrow_mut()))
    }

    pub fn get_next_index(&self) -> u64 {
        self.sbt_index
    }

    pub fn increment_index(&mut self) {
        self.sbt_index += 1;
    }

    pub fn mint_soulbound_token(&mut self, user: Principal, token_image: String) -> Result<(), String> {
        let index = self.get_next_index();

        let result = match self.sbts.entry(user) {
            Entry::Occupied(entry) => Err(format!(
                "Principal {:?} already has its soul bound to Token: {:?}",
                entry.key(),
                entry.get()
            )),
            Entry::Vacant(entry) => {
                entry.insert(SoulboundToken::new(index, token_image));
                Ok(())
            }
        };

        if result.is_ok() {
            self.increment_index();
        }

        result
    }

    pub fn get_sbt_image(&self, id: u64) -> Result<&[u8], String> {
        if let Some(image) = self
            .sbts
            .iter()
            .find_map(|(_, sbt)| if id == sbt.id { Some(&sbt.image) } else { None })
        {
            self.get_image(image)
        } else {
            Err(format!("Token with id {:?} does not exist", id))
        }
    }

    pub fn clear_image(&mut self, image: String) {
        if let Some(image) = self.sbt_images.get_mut(&image) {
            image.clear();
        };
    }

    pub fn append_image_bytes(&mut self, image: String, mut bytes: Vec<u8>) {
        if let Some(image) = self.sbt_images.get_mut(&image) {
            image.append(&mut bytes);
        } else {
            self.sbt_images.insert(image, bytes);
        };
    }

    pub fn set_image(&mut self, image: String, bytes: Vec<u8>) {
        self.sbt_images.insert(image, bytes);
    }

    pub fn get_image(&self, image: &str) -> Result<&[u8], String> {
        if let Some(bytes) = self.sbt_images.get(image) {
            Ok(bytes)
        } else {
            Err(format!("Image {:?} does not exist", image))
        }
    }

    pub fn get_registry(&self) -> Vec<(Principal, Vec<u64>)> {
        self.sbts
            .iter()
            .map(|(principal, token)| (*principal, vec![token.id]))
            .collect()
    }

    pub fn get_user_registry(&self, user: Principal) -> Vec<u64> {
        if let Some(id) = self.sbts.iter().find_map(
            |(principal, token)| {
                if *principal == user {
                    Some(token.id)
                } else {
                    None
                }
            },
        ) {
            vec![id]
        } else {
            vec![]
        }
    }

    pub fn get_admins(&self) -> Vec<Principal> {
        self.controllers
            .ref_values()
            .iter()
            .filter_map(|controller| {
                if controller.kind == ServiceControllerKind::Admin {
                    Some(controller.controller_id)
                } else {
                    None
                }
            })
            .collect::<Vec<Principal>>()
    }

    pub fn get_service_controllers(&self) -> &Vec<ServiceController> {
        self.controllers.ref_values()
    }

    pub fn add_owner(&mut self, principal: Principal) -> bool {
        self.controllers.add(ServiceControllerKind::Owner, principal)
    }

    pub fn add_admin(&mut self, principal: Principal) -> bool {
        self.controllers.add(ServiceControllerKind::Admin, principal)
    }

    pub fn remove_admin(&mut self, principal: &Principal) -> bool {
        self.controllers.remove(principal, ServiceControllerKind::Admin)
    }

    pub fn has_access(&self, kind: ServiceControllerKind, principal: Principal) -> bool {
        self.controllers.has_access(kind, principal)
    }
}
