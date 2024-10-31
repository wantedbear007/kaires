pub mod types;
pub mod canister_management;
pub mod canister_mgmt_types;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};
use ic_cdk::api::call::{CallResult, RejectionCode};
use serde_bytes::{self, ByteBuf};


#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct ImageData {
    pub content: ByteBuf,
    pub name: String,
    pub content_type: String,
    pub asset_canister: Principal
}





// inter canister call
pub async fn call_inter_canister<T, U>(
    function: &str,
    args: T,
    canister_id: Principal,
) -> Result<U, String>
where
    T: CandidType + Serialize,
    U: CandidType + for<'de> serde::Deserialize<'de>,
{
    let response: CallResult<(Result<U, String>,)> =
        ic_cdk::call(canister_id, function, (args,)).await;

    let res0: Result<(Result<U, String>,), (RejectionCode, String)> = response;

    match res0 {
        Ok((Ok(value),)) => Ok(value),
        Ok((Err(err),)) => Err(err),
        Err((code, message)) => match code {
            RejectionCode::NoError => Err("NoError".to_string()),
            RejectionCode::SysFatal => Err("SysFatal".to_string()),
            RejectionCode::SysTransient => Err("SysTransient".to_string()),
            RejectionCode::DestinationInvalid => Err("DestinationInvalid".to_string()),
            RejectionCode::CanisterReject => Err("CanisterReject".to_string()),
            _ => Err(format!("Unknown rejection code: {:?}: {}", code, message)),
        },
    }
}


type ReturnResult = Result<u32, String>;

// upload image
pub async fn upload_image_to_asset_canister(image_data: ImageData) -> Result<String, String> {
    // dao canister id
    // with_state(|state|)
    // let canister_id = with_state(|state| {
    //     state.get_canister_ids()
    // })?;
    // let canister_id = with_state(|state| state.canister_data.get(&0));

    // let canister_id = match canister_id {
    //     Some(val) => val,
    //     None => return Err(String::from("Canister Meta data not found.")),
    // };

    let response: CallResult<(ReturnResult,)> = ic_cdk::call(
        image_data.asset_canister,
        // Principal::from_text(canister_id.ic_asset_canister).unwrap(),
        "create_file",
        (image_data,),
    )
    .await;
    // format!("{:?}", result.ok());

    let res0: Result<(Result<u32, String>,), (RejectionCode, String)> = response;

    let formatted_value = match res0 {
        Ok((Ok(value),)) => {
            // format!("{}", value);
            Ok(format!("{}", value))
            // value
        }
        Ok((Err(err),)) => Err(err),
        Err((code, message)) => {
            match code {
                RejectionCode::NoError => Err("NoError".to_string()),
                RejectionCode::SysFatal => Err("SysFatal".to_string()),
                RejectionCode::SysTransient => Err("SysTransient".to_string()),
                RejectionCode::DestinationInvalid => Err("DestinationInvalid".to_string()),
                RejectionCode::CanisterReject => Err("CanisterReject".to_string()),
                // Handle other rejection codes here
                _ => Err(format!("Unknown rejection code: {:?}: {}", code, message)),
                // _ => Err(format!("Unknown rejection code: {:?}", code)),
            }
        }
    };

    formatted_value
}


pub fn get_random_number(from: u64, to: u64) -> u64 {
    let current_time = ic_cdk::api::time();  
    let range = to - from + 1;
    from + (current_time % range)
}

