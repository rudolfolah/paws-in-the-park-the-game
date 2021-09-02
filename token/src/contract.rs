#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Order};
use cw_storage_plus::{Bound, Prefix};
use cw2::set_contract_version;
use cw20::{Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TokenInfoResponse, InventoryResponse, GameInfoResponse};
use crate::state::{HOLDERS, TOTAL_SUPPLY, DOGS, ACCESSORIES, DogData, AccessoryData, TOTAL_SUPPLY_DOGS, TOTAL_SUPPLY_ACCESSORIES};
use crate::utils::{generate_id};
use std::ops::Add;

const CONTRACT_NAME: &str = "my-pet-pal:token";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const TOKEN_NAME: &str = "Tail Wag";
const TOKEN_SYMBOL: &str = "TAG";

const INITIAL_SUPPLY: u128 = 10_000u128;
const OWNER_ADDR: &str = "terra1wr3uw7rsjr8p94m8hlagvdrfghxuq674v5nxxc";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let total_supply = Uint128::from(INITIAL_SUPPLY);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    TOTAL_SUPPLY.save(deps.storage, &total_supply)?;
    HOLDERS.save(deps.storage, OWNER_ADDR.as_bytes(), &total_supply)?;

    // TODO: the following need to go into a separate smart contract
    TOTAL_SUPPLY_DOGS.save(deps.storage, &Uint128::from(0u128));
    TOTAL_SUPPLY_ACCESSORIES.save(deps.storage, &Uint128::from(0u128));

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", OWNER_ADDR)
        .add_attribute("total_supply", total_supply))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { amount, recipient } => try_transfer(deps, info, amount, recipient),
        ExecuteMsg::Burn { amount } => try_burn(deps, info, amount),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => try_send(deps, info, contract, amount, msg),
        ExecuteMsg::Mint { amount } => try_mint(deps, info, amount),
        // TODO: the following need to go into a separate smart contract
        ExecuteMsg::MintDog { amount } => try_mint_dog(deps, info, amount),
        ExecuteMsg::MintAccessory { amount } => try_mint_accessory(deps, info, amount),
    }
}

pub fn try_mint(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError>  {
    if amount.is_zero() {
        return Err(ContractError::AmountCannotBeZero {});
    }
    if info.sender != OWNER_ADDR {
        return Err(ContractError::Unauthorized {});
    }
    let updated_owner_balance = HOLDERS.update(
        deps.storage,
        OWNER_ADDR.as_bytes(),
        |balance| -> StdResult<_> {
            Ok(balance.unwrap_or_default() + amount)
        }
    )?;
    let updated_total_supply = TOTAL_SUPPLY.update(
        deps.storage,
        |total_supply: Uint128 | -> StdResult<_>  {
            Ok(total_supply.add(amount))
        }
    )?;
    Ok(Response::new()
        .add_attribute("method", "try_mint")
        .add_attribute("amount", amount)
        .add_attribute("updated_owner_balance", updated_owner_balance)
        .add_attribute("updated_total_supply", updated_total_supply)
    )
}

pub fn try_transfer(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    recipient: String,
) -> Result<Response, ContractError> {
    let sender_key = info.sender.as_bytes();
    let current_balance = HOLDERS
        .may_load(deps.storage, sender_key)?
        .unwrap_or(Uint128::from(0u128));
    match current_balance.u128().checked_sub(amount.u128()) {
        Some(_updated_balance) => {}
        None => return Err(ContractError::AmountIsGreaterThanAvailableBalance {}),
    };
    let updated_balance = current_balance.u128() - amount.u128();
    let current_recipient_balance = HOLDERS
        .may_load(deps.storage, recipient.as_bytes())?
        .unwrap_or(Uint128::from(0u128));
    let updated_recipient_balance = current_recipient_balance.u128() + amount.u128();
    HOLDERS.save(
        deps.storage,
        sender_key,
        &Uint128::from(updated_balance)
    )?;
    HOLDERS.save(
        deps.storage,
        recipient.as_bytes(),
        &Uint128::from(updated_recipient_balance),
    )?;
    Ok(Response::new().add_attribute("method", "try_transfer"))
}

pub fn try_burn(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount.is_zero() {
        return Err(ContractError::AmountCannotBeZero {});
    }
    HOLDERS.update(
        deps.storage,
        info.sender.as_bytes(),
        |balance| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        }
    )?;
    TOTAL_SUPPLY.update(
        deps.storage,
        |total_supply: Uint128 | -> StdResult<_>  {
            Ok(total_supply.checked_sub(amount)?)
        }
    )?;
    Ok(Response::new()
        .add_attribute("method", "try_burn")
        .add_attribute("amount", amount)
    )
}

/// Based on the code in cw20-base for execute_send
pub fn try_send(
    deps: DepsMut, info: MessageInfo, contract: String, amount: Uint128, msg: Binary
) -> Result<Response, ContractError> {
    let recipient_address = deps.api.addr_validate(&contract)?;
    HOLDERS.update(
        deps.storage,
        info.sender.as_bytes(),
        |balance| -> StdResult<_> {
            Ok(balance.unwrap_or_default().checked_sub(amount)?)
        }
    )?;
    HOLDERS.update(
        deps.storage,
        recipient_address.as_bytes(),
        |balance| -> StdResult<_> {
            Ok(balance.unwrap_or_default() + amount)
        }
    )?;

    let res = Response::new()
        .add_attribute("action", "send")
        .add_attribute("from", &info.sender)
        .add_attribute("to", &contract)
        .add_attribute("amount", amount)
        .add_message(
            Cw20ReceiveMsg {
                sender: info.sender.into(),
                amount,
                msg,
            }
                .into_cosmos_msg(contract)?,
        );
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        // TODO: the following need to go into a separate smart contract
        QueryMsg::GameInfo {} => to_binary(&query_game_info(deps)?),
        QueryMsg::Inventory { address } => to_binary(&query_inventory(deps, address)?),
    }
}

fn query_balance(deps: Deps, address: String) -> StdResult<BalanceResponse> {
    let balance = HOLDERS
        .may_load(deps.storage, address.as_bytes())?
        .unwrap_or(Uint128::from(0u128));
    return Ok(BalanceResponse {
        balance: Uint128::from(balance),
    });
}

fn query_token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let total_supply = TOTAL_SUPPLY
        .may_load(deps.storage)?
        .unwrap_or(Uint128::from(0u128));
    return Ok(TokenInfoResponse {
        name: TOKEN_NAME.to_string(),
        symbol: TOKEN_SYMBOL.to_string(),
        decimals: 0,
        total_supply,
    });
}

// TODO: the following need to go into a separate smart contract
fn try_mint_dog(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    if info.sender != OWNER_ADDR {
        return Err(ContractError::Unauthorized {});
    }
    for _i in 0u128..amount.u128() {
        let id = generate_id();
        let dog_data = DogData {
            class: 0,
            attr1: 0,
            attr2: 0,
            attr3: 0,
            attr4: 0,
            name: String::from("Testing"),
            id: id.clone(),
        };
        DOGS.save(deps.storage, (info.sender.as_bytes(), id.as_bytes()), &dog_data);
    }

    TOTAL_SUPPLY_DOGS.update(
        deps.storage,
        |total_supply: Uint128 | -> StdResult<_>  {
            Ok(total_supply.add(amount))
        }
    )?;

    Ok(Response::new()
        .add_attribute("method", "try_mint_dog")
    )
}

fn try_mint_accessory(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    if info.sender != OWNER_ADDR {
        return Err(ContractError::Unauthorized {});
    }
    let id = generate_id();
    let accessory_data = AccessoryData {
        name: String::from("Testing"),
        id: id.clone(),
    };
    ACCESSORIES.save(deps.storage, (info.sender.as_bytes(), id.as_bytes()), &accessory_data);
    TOTAL_SUPPLY_ACCESSORIES.update(
        deps.storage,
        |total_supply: Uint128 | -> StdResult<_>  {
            Ok(total_supply.add(Uint128::from(1u128)))
        }
    )?;
    Ok(Response::new()
        .add_attribute("method", "try_mint_accessory")
        .add_attribute("id", &id)
    )
}

fn query_game_info(deps: Deps) -> StdResult<GameInfoResponse> {
    let total_supply_dogs = TOTAL_SUPPLY_DOGS
        .may_load(deps.storage)?
        .unwrap_or(Uint128::from(0u128));
    let total_supply_accessories = TOTAL_SUPPLY_ACCESSORIES
        .may_load(deps.storage)?
        .unwrap_or(Uint128::from(0u128));
    return Ok(GameInfoResponse {
        total_supply_dogs,
        total_supply_accessories
    });
}

fn query_inventory(deps: Deps, address: String) -> StdResult<InventoryResponse> {
    let dogs: Vec<_> = DOGS.prefix(address.as_bytes())
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    // print_type_of(&dogs);
    // println!("{:?}", dogs);
    let mut dogs_response = Vec::new();
    for dog in dogs {
        let (_key, dog_data) = dog.unwrap();
        dogs_response.push(dog_data);
    }

    let accessories: Vec<_> = ACCESSORIES.prefix(address.as_bytes())
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut accessories_response = Vec::new();
    for accessory in accessories {
        let (_key, accessory_data) = accessory.unwrap();
        accessories_response.push(accessory_data);
    }

    return Ok(InventoryResponse {
        address,
        dogs: dogs_response,
        accessories: accessories_response,
    });
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Api, OwnedDeps, Querier, Storage};
    use crate::utils::print_type_of;

    /// checks the `address` to ensure it has the correct `expected` balance
    fn assert_balance_is<S: Storage, A: Api, Q: Querier>(
        deps: &OwnedDeps<S, A, Q>,
        address: &str,
        expected: u128,
    ) {
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Balance {
                address: address.to_string(),
            },
        )
        .unwrap();
        let balance: BalanceResponse = from_binary(&res).unwrap();
        assert_eq!(
            expected,
            balance.balance.u128(),
            "address '{}' should have {} tokens",
            address,
            expected
        );
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfoResponse = from_binary(&res).unwrap();
        assert_eq!("Tail Wag", token_info.name);
        assert_eq!("TAG", token_info.symbol);
        assert_eq!(10_000u128, token_info.total_supply.u128());
        assert_eq!(0, token_info.decimals);

        assert_balance_is(&deps, OWNER_ADDR, 10_000u128);
        assert_balance_is(&deps, "another_address", 0u128);
    }

    #[test]
    fn burn() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let creator_info = mock_info(OWNER_ADDR, &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), creator_info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        assert_balance_is(&deps, OWNER_ADDR, 10_000u128);
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(1_000),
            },
        ).unwrap();
        assert_balance_is(&deps, OWNER_ADDR, 9_000u128);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfoResponse = from_binary(&res).unwrap();
        assert_eq!(
            9_000u128,
            token_info.total_supply.u128(),
            "token supply should be smaller"
        );
    }

    #[test]
    fn burn_all_supply() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let creator_info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), creator_info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(2u128),
                recipient: "other_address".to_string(),
            },
        ).unwrap();
        assert_balance_is(&deps, "other_address", 2u128);
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("other_address", &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(1),
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("other_address", &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(1),
            },
        )
        .unwrap();
        assert_balance_is(&deps, "other_address", 0u128);
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(9998),
            },
        )
        .unwrap();
        assert_balance_is(&deps, OWNER_ADDR, 0u128);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::TokenInfo {}).unwrap();
        let token_info: TokenInfoResponse = from_binary(&res).unwrap();
        assert_eq!(
            0u128,
            token_info.total_supply.u128(),
            "total supply should be zero"
        );
    }

    #[test]
    fn transfer() {
        let mut deps = mock_dependencies(&[]);
        let _res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {},
        );
        assert_balance_is(&deps, OWNER_ADDR, 10_000u128);
        assert_balance_is(&deps, "other_address", 0u128);

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(10_001u128),
                recipient: "other_address".to_string(),
            },
        );
        match res {
            Err(ContractError::AmountIsGreaterThanAvailableBalance {}) => {}
            _ => panic!("must return amount is greater than available balance error"),
        }

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(3u128),
                recipient: "other_address".to_string(),
            },
        );
        assert_balance_is(&deps, OWNER_ADDR, 9_997u128);
        assert_balance_is(&deps, "other_address", 3u128);

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(0u128),
                recipient: "other_address".to_string(),
            },
        );
        assert_balance_is(&deps, OWNER_ADDR, 9_997u128);
        assert_balance_is(&deps, "other_address", 3u128);

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(9_997u128),
                recipient: "other_address".to_string(),
            },
        );
        assert_balance_is(&deps, OWNER_ADDR, 0u128);
        assert_balance_is(&deps, "other_address", 10_000u128);

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(10_000u128),
                recipient: "other_address".to_string(),
            },
        );
        match res {
            Err(ContractError::AmountIsGreaterThanAvailableBalance {}) => {}
            _ => panic!("must return amount is greater than available balance error"),
        }
        assert_balance_is(&deps, OWNER_ADDR, 0u128);
        assert_balance_is(&deps, "other_address", 10_000u128);
    }

    // TODO: the following need to go into a separate smart contract
    #[test]
    fn mint_dogs() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("other_address", &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(1u128) },
        );
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("must return unauthorized error"),
        }

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(5u128) },
        );
        let res = query(deps.as_ref(), mock_env(), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(5, inventory.dogs.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GameInfo {}).unwrap();
        let game_info: GameInfoResponse = from_binary(&res).unwrap();
        assert_eq!(5, game_info.total_supply_dogs.u128());
    }

    #[test]
    fn inventory_empty() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(0, inventory.dogs.len());
        assert_eq!(0, inventory.accessories.len());
    }

    #[test]
    fn inventory_not_empty() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(1u128) },
        );

        let _res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintAccessory { amount: Uint128::from(1u128) },
        );

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(1, inventory.dogs.len());
        assert_eq!(1, inventory.accessories.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Inventory { address: "other_address".to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(0, inventory.dogs.len());
        assert_eq!(0, inventory.accessories.len());
    }
}
