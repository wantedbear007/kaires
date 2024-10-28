
use candid::{CandidType, Nat, Principal};
use candid::{Decode, Encode};
use serde::{Deserialize, Serialize};
use serde_bytes::{self, ByteBuf};
use std::borrow::Cow;


pub type CanisterId = Principal;


#[derive(
  CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub struct CanisterIdRecord {
  /// Principal of the canister.
  pub canister_id: CanisterId,
}

#[derive(
  CandidType,
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Clone,
  Copy,
  Default,
)]
pub struct SkipPreUpgrade(pub Option<bool>);


#[derive(
  CandidType,
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Clone,
  Copy,
  Default,
)]
pub enum CanisterInstallMode {
  /// A fresh install of a new canister.
  #[serde(rename = "install")]
  #[default]
  Install,
  /// Reinstalling a canister that was already installed.
  #[serde(rename = "reinstall")]
  Reinstall,
  /// Upgrade an existing canister.
  #[serde(rename = "upgrade")]
  Upgrade(Option<SkipPreUpgrade>),
}


#[derive(
  CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CanisterSettings {
  pub controllers: Option<Vec<Principal>>,

  pub compute_allocation: Option<Nat>,

  pub memory_allocation: Option<Nat>,

  pub freezing_threshold: Option<Nat>,

  pub reserved_cycles_limit: Option<Nat>,
}

#[derive(
  CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CreateCanisterArgument {
  /// See [CanisterSettings].
  pub settings: Option<CanisterSettings>,
}


#[derive(
  CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub(crate) struct CreateCanisterArgumentExtended {
  /// See [CanisterSettings].
  pub settings: Option<CanisterSettings>,
  /// sender_canister_version must be set to ic_cdk::api::canister_version()
  pub sender_canister_version: Option<u64>,
}

pub type WasmModule = Vec<u8>;

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct InstallCodeArgument {
    /// See [CanisterInstallMode].
    pub mode: CanisterInstallMode,
    /// Principal of the canister.
    pub canister_id: CanisterId,
    /// Code to be installed.
    pub wasm_module: WasmModule,
    /// The argument to be passed to `canister_init` or `canister_post_upgrade`.
    pub arg: Vec<u8>,
}

#[derive(
  CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub(crate) struct InstallCodeArgumentExtended {
  /// See [CanisterInstallMode].
  pub mode: CanisterInstallMode,
  /// Principal of the canister.
  pub canister_id: CanisterId,
  /// Code to be installed.
  pub wasm_module: WasmModule,
  /// The argument to be passed to `canister_init` or `canister_post_upgrade`.
  pub arg: Vec<u8>,
  /// sender_canister_version must be set to ic_cdk::api::canister_version()
  pub sender_canister_version: Option<u64>,
}