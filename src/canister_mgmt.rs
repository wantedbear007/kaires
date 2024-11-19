use candid::{Nat, Principal};

use ic_cdk::api::management_canister::{
    main::{self, CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument},
    provisional::CanisterSettings,
};

pub async fn create_users_canister(
    wasm: &[u8],
    encoded_args: Vec<u8>,
    controllers: Vec<Principal>,
    cycles: u128,
) -> Principal {
    let arg = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(controllers),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            ..Default::default()
        }),
    };

    // * provisioned canister
    let canister_id: Principal = main::create_canister(arg, cycles)
        .await
        .unwrap()
        .0
        .canister_id;

    // * install wasm to provisioned canister
    main::install_code(InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: wasm.into(),
        arg: encoded_args,
    })
    .await
    .unwrap();

    canister_id
}
