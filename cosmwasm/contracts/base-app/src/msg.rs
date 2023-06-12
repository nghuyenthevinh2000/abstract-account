use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

    #[returns(VerifiedResponse)]
    Verified {
        token_hash: Vec<u8>
    },
}

#[cw_serde]
pub struct VerifiedResponse {
    pub verified: bool,
}
