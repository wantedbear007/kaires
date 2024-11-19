use candid::Principal;

use ic_cdk::api::{
  self,
  call::RejectionCode,
  management_canister::{
      main::{self, CanisterInstallMode, CreateCanisterArgument, InstallCodeArgument},
      provisional::CanisterSettings,
  },
};

pub const INDIVIDUAL_USER_CANISTER_RECHARGE_AMOUNT: u128 = 1_000_000_000_000; // 1T Cycles


// const INDIVIDUAL_USER_TEMPLATE_CANISTER_WASM: &[u8] = include_bytes!(
//   "../../../../../target/wasm32-unknown-unknown/release/individual_user_template.wasm.gz"
// );

pub async fn create_users_canister( wasm: &[u8], args: Vec<u8>) -> Principal {
  // * config for provisioning canister
  let arg = CreateCanisterArgument {
      settings: Some(CanisterSettings {
          controllers: Some(vec![
              // * this user_index canister
              api::id(),
          ]),
          compute_allocation: None,
          memory_allocation: None,
          freezing_threshold: None,
          ..Default::default()
      }),
  };

  // * provisioned canister
  let canister_id: Principal =
      main::create_canister(arg, INDIVIDUAL_USER_CANISTER_RECHARGE_AMOUNT)
          .await
          .unwrap()
          .0
          .canister_id;

  

  // let individual_user_tempalate_init_args = IndividualUserTemplateInitArgs {
  //     profile_owner: Some(profile_owner),
  //     known_principal_ids: Some(CANISTER_DATA.with(|canister_data_ref_cell| {
  //         canister_data_ref_cell.borrow().known_principal_ids.clone()
  //     })),
  //     upgrade_version_number: Some(0),
  //     url_to_send_canister_metrics_to: Some(configuration.url_to_send_canister_metrics_to),
  // };

  // * encode argument for user canister init lifecycle method
  // let arg = candid::encode_one(individual_user_tempalate_init_args)
  //     .expect("Failed to serialize the install argument.");

  // * install wasm to provisioned canister
  main::install_code(InstallCodeArgument {
      mode: CanisterInstallMode::Install,
      canister_id,
      wasm_module: wasm.into(),
      arg: args,
  })
  .await
  .unwrap();

  canister_id
}
