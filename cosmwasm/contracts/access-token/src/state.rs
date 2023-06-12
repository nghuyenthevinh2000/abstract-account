use cosmwasm_std::Binary;
use cw_storage_plus::{Item, Map};

use crate::msg::AccessToken;

pub const PUBKEY: Item<Binary> = Item::new("pk");

// storing access token
pub const ACCESS_TOKEN: Map<String, AccessToken> = Map::new("access_token");