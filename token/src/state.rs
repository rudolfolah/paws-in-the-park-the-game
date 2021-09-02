use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map}; // See: https://github.com/CosmWasm/cw-plus/tree/main/packages/storage-plus
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");

// Key is the Address, value is the amount of tokens being held by that address
pub const HOLDERS: Map<&[u8], Uint128> = Map::new("holders");


// TODO: the following need to go into a separate smart contract
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DogData {
    pub class: u8,
    pub attr1: u8,
    pub attr2: u8,
    pub attr3: u8,
    pub attr4: u8,
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessoryData {
    pub name: String,
    pub id: String,
}

pub const TOTAL_SUPPLY_DOGS: Item<Uint128> = Item::new("total_supply_dogs");
pub const TOTAL_SUPPLY_ACCESSORIES: Item<Uint128> = Item::new("total_supply_accessories");

pub const DOGS: Map<(&[u8], &[u8]), DogData> = Map::new("all_dogs");
pub const ACCESSORIES: Map<(&[u8], &[u8]), AccessoryData> = Map::new("all_accessories");
