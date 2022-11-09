use crate::guards::{is_backup_service, is_restore_service};
use crate::state::stable_storage::{stable_restore, stable_save, StableStorage};
use crate::State;
use ic_cdk_macros::{post_upgrade, pre_upgrade, query, update};

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("[SBT] Pre Upgrade Started!");
    State::mutate_state(|s| stable_save(StableStorage::from(s)).unwrap());
    ic_cdk::println!("[SBT] Pre Upgrade Finished");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("[SBT] Post Upgrade Started!");
    State::mutate_state(|s| {
        let (storage, instructions) = stable_restore::<StableStorage, u64>();
        let storage = storage.unwrap();
        let pre_upgrade_count = instructions.unwrap();
        ic_cdk::println!("[SBT] Total Pre Upgrade Instruction Count: {:?}", pre_upgrade_count);
        *s = storage.into();
        let count = ic_cdk::api::instruction_counter();
        ic_cdk::println!("[SBT] Total Post Upgrade Instruction Count: {:?}", count);
        ic_cdk::println!(
            "[SBT] Total Pre + Post Upgrade Instruction Count: {:?}",
            pre_upgrade_count + count
        );
    });
    ic_cdk::println!("[SBT] Post Upgrade Finished");
}

#[query(guard = "is_backup_service")]
fn backup() {}

#[update(guard = "is_restore_service")]
fn restore() {}
