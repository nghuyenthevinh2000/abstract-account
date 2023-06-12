use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ExecuteMsg {
    // Register Access Token
    // it will include an access token and a ttl (time to live)
    RegisterAccessToken {
        token: AccessToken,
        ttl:  u64,
    },
    // Revoke Access Token
    // app contract will revoke access token from a contract
    RevokeAccessToken {
        app_address: String,
    },
    // Get Access Token 
    // app contract will get access token from an user contract
    // every time access token is retrieved, it will reduce ttl by 1
    GetAccessToken {
        app_address: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum SudoMsg {
    QueryAccessTokens {
        app_address: String,
    },
}

#[cw_serde]
pub struct AccessToken {
    pub app_address:    String,
    pub access_token:   String,
}

#[cw_serde]
pub struct RegisterAccessTokenResponse {
    pub status: String,
}

#[cw_serde]
pub struct RevokeAccessTokenResponse {
    pub status: String,
}

#[cw_serde]
pub struct GetAccessTokenResponse {
    pub status: String,
    pub access_token: AccessToken,
}