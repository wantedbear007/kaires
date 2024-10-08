use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use serde_bytes::{self, ByteBuf};


#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct ImageData {
    pub content: ByteBuf,
    pub name: String,
    pub content_type: String,
    pub asset_canister: Principal
}
