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
    sbt_image: Vec<u8>,
    sbt_index: u64,
    sbts: HashMap<Principal, SoulboundToken>,
    controllers: ServiceControllers,
}

impl From<StableStorage> for State {
    fn from(storage: StableStorage) -> Self {
        Self {
            sbt_image: storage.sbt_image,
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

    pub fn mint_token(&mut self, user: Principal) -> Result<(), String> {
        let index = self.get_next_index();

        let result = match self.sbts.entry(user) {
            Entry::Occupied(entry) => Err(format!(
                "Principal {:?} already owns Token: {:?}",
                entry.key(),
                entry.get()
            )),
            Entry::Vacant(entry) => {
                entry.insert(SoulboundToken::new(index));
                Ok(())
            }
        };

        if result.is_ok() {
            self.increment_index();
        }

        result
    }

    pub fn contains_token(&self, id: u64) -> bool {
        self.sbts.values().any(|token| token.id == id)
    }

    pub fn clear_image(&mut self) {
        self.sbt_image.clear();
    }

    pub fn append_image_bytes(&mut self, bytes: &mut Vec<u8>) {
        self.sbt_image.append(bytes);
    }

    pub fn set_image(&mut self, bytes: Vec<u8>) {
        self.sbt_image = bytes;
    }

    pub fn get_image(&self) -> &[u8] {
        &self.sbt_image
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
