use cosmwasm_std::{Binary, Response, Deps, BlockInfo};
use abstract_account::Any;
use app_base::msg::QueryMsg;

use crate::{error::ContractResult, state::ACCESS_TOKEN};
use crate::msg::AccessToken;

pub fn before_tx(
    deps: Deps,
    block: &BlockInfo,
    msgs: &[Any],
    tx_bytes: &Binary,
    credential_bytes: &Binary,
) -> ContractResult<Response> {
    // retrieve app contract address from msgs
    for msg in msgs {
        // check if app address existed in ACCESS_TOKEN
        if ACCESS_TOKEN.has(deps.storage, msg.app_address) {
            // query proof from app contract
            // QueryMsg{token: hash(token + user address)} -> App contract check if hash(token + user address) exists -> Res{verified: bool}
            // Signature can be nil
            // Need to create a new struct AccessTokenAny in msgs:&[Any] so that it would expose the app contract address for retrieving token
            let token = ACCESS_TOKEN.load(deps.storage, msg.app_address)?;
            // token hash = sha256(access_token + user address)
            // TODO: need to figure out how to get user address
            let token_hash = sha256(token.access_token);
        }
    }

    
    return Ok(Response::default());
}

pub fn sha256(msg: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    hasher.finalize().to_vec()
}