#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Order, Coin, SubMsg, BankMsg, Timestamp};
use cw2::set_contract_version;
use cw20::{Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TokenInfoResponse, InventoryResponse, GameInfoResponse, MarketListingsResponse};
use crate::state::{HOLDERS, TOTAL_SUPPLY, DOGS, ACCESSORIES, DogData, AccessoryData, MARKET_LISTINGS_DOGS, MarketListing};
use crate::utils::{generate_id, rand_int_between};
use std::ops::{Add};

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
    // println!("{:?}", _info.funds);
    let total_supply = Uint128::from(INITIAL_SUPPLY);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    TOTAL_SUPPLY.save(deps.storage, &total_supply)?;
    HOLDERS.save(deps.storage, OWNER_ADDR.as_bytes(), &total_supply)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", OWNER_ADDR)
        .add_attribute("total_supply", total_supply))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
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
        ExecuteMsg::MintDog { amount } => try_mint_dog(deps, env, info, amount),
        ExecuteMsg::MintAccessory { name, amount } => try_mint_accessory(deps, env, info, name, amount),
        ExecuteMsg::SellDogOnMarket { dog_id, price} => try_sell_dog_on_market(deps, info, dog_id, price),
        ExecuteMsg::BuyDogOnMarket { dog_id } => try_buy_dog_on_market(deps, info, dog_id),
        ExecuteMsg::SpinTheWheel {} => try_spin_the_wheel(deps, env, info),
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
        QueryMsg::MarketListings {} => to_binary(&query_market_listings(deps)?),
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
const SALES_COMMISSION_RATE: u128 = 20; // 5% = 0.05 = 100 / 20
const SPIN_THE_WHEEL_PRICE_UUSD: u128 = 1_000_000_u128;
const PRICE_DENOM: &str = "uusd";

const DOG_CLASS_ATTR_RANGES: [[(u8, u8); 4]; 7] = [
    [(5u8, 10u8), (1u8, 7u8), (1u8, 7u8), (1u8, 7u8)],
    [(1u8, 7u8), (5u8, 10u8), (1u8, 7u8), (1u8, 7u8)],
    [(1u8, 7u8), (1u8, 7u8), (5u8, 10u8), (1u8, 7u8)],
    [(1u8, 7u8), (1u8, 7u8), (1u8, 7u8), (5u8, 10u8)],
    [(7u8, 10u8), (7u8, 10u8), (1u8, 7u8), (1u8, 7u8)],
    [(7u8, 10u8), (1u8, 7u8), (1u8, 7u8), (7u8, 10u8)],
    [(1u8, 7u8), (1u8, 7u8), (7u8, 10u8), (7u8, 10u8)],
];

const ACCESSORY_NAMES: [&str; 3] = [
    "martini glass",
    "sparkle",
    "star",
];

fn create_random_dog(seed_timestamp: Timestamp, id: &String, name: &String, class: u8) -> DogData {
    let ranges = DOG_CLASS_ATTR_RANGES[usize::from(class)];
    return DogData {
        class: class.clone(),
        id: id.clone(),
        name: name.clone(),
        attr1: rand_int_between(seed_timestamp, ranges[0].0, ranges[0].1),
        attr2: rand_int_between(seed_timestamp, ranges[1].0, ranges[1].1),
        attr3: rand_int_between(seed_timestamp, ranges[2].0, ranges[2].1),
        attr4: rand_int_between(seed_timestamp, ranges[3].0, ranges[3].1),
    };
}

fn try_mint_dog(deps: DepsMut, env: Env, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    if info.sender != OWNER_ADDR {
        return Err(ContractError::Unauthorized {});
    }
    for i in 0u128..amount.u128() {
        let id = generate_id(
            Timestamp::from_nanos(
                env.block.time.nanos() + i as u64
            ));
        println!("minting dog #{}: #{}", i, id);
        let name = format!("Dog #{}", id);
        let class = rand_int_between(Timestamp::from_nanos(
            env.block.time.nanos() + i as u64
        ), 0u8, 6u8);
        let dog_data = create_random_dog(Timestamp::from_nanos(
            env.block.time.nanos() + i as u64
        ), &id, &name, class);
        // let dog_data = DogData {
        //     class: class.clone(),
        //     id: id.clone(),
        //     attr1: rand_int_between(ranges[0].0, ranges[0].1),
        //     attr2: rand_int_between(ranges[1].0, ranges[1].1),
        //     attr3: rand_int_between(ranges[2].0, ranges[2].1),
        //     attr4: rand_int_between(ranges[3].0, ranges[3].1),
        //     name: String::from("Testing"),
        // };
        // println!("dog {:?}", dog_data);
        let key = (info.sender.as_bytes(), id.as_bytes());
        println!("key is {:?}", key);
        DOGS.save(deps.storage, key, &dog_data)?;
    }

    Ok(Response::new()
        .add_attribute("method", "try_mint_dog")
    )
}

fn try_mint_accessory(deps: DepsMut, env: Env, info: MessageInfo, name: String, _amount: Uint128) -> Result<Response, ContractError> {
    if info.sender != OWNER_ADDR {
        return Err(ContractError::Unauthorized {});
    }
    // TODO: mint multiple accessories based on the amount given
    let id = generate_id(env.block.time);
    let accessory_data = AccessoryData {
        name,
        id: id.clone(),
    };
    ACCESSORIES.save(
        deps.storage,
        (info.sender.as_bytes(), id.as_bytes()),
        &accessory_data
    )?;

    Ok(Response::new()
        .add_attribute("method", "try_mint_accessory")
        .add_attribute("id", &id)
    )
}

fn try_sell_dog_on_market(deps: DepsMut, info: MessageInfo, dog_id: String, price: Uint128) -> Result<Response, ContractError> {
    if price.u128() < 500_000u128 {
        return Err(ContractError::ListingPriceTooLow {});
    }
    if price.u128() > 1_000_000_000_000u128 {
        return Err(ContractError::ListingPriceTooHigh {});
    }
    let key = (info.sender.as_bytes(), dog_id.as_bytes());
    let dog = DOGS.may_load(deps.storage, key)?;
    if dog.is_none() {
        return Err(ContractError::DogDoesNotExist {});
    }
    if MARKET_LISTINGS_DOGS.has(deps.storage, dog_id.as_bytes()) {
        return Err(ContractError::AlreadyListedOnMarket {});
    }
    MARKET_LISTINGS_DOGS.save(deps.storage, dog_id.as_bytes(), &MarketListing {
        id: dog_id.to_string(),
        price: Coin { amount: price , denom: PRICE_DENOM.to_string() },
        listed_by_address: info.sender.into_string(),
    })?;

    Ok(Response::new()
           .add_attribute("method", "try_sell_dog_on_market")
    )
}

fn try_buy_dog_on_market(deps: DepsMut, info: MessageInfo, dog_id: String) -> Result<Response, ContractError> {
    let listing = MARKET_LISTINGS_DOGS.may_load(
        deps.storage, dog_id.as_bytes()
    )?;
    if listing.is_none() {
        return Err(ContractError::ListingDoesNotExist {});
    }
    let listing = listing.unwrap();

    let fund = info.funds.iter()
        .find(|&x| x.denom == listing.price.denom && x.amount == listing.price.amount);
    if fund.is_none() {
        return Err(ContractError::DoesNotMatchListingPrice {});
    }
    let fund = fund.unwrap();
    let commission = Uint128::from(SALES_COMMISSION_RATE);
    let amount_for_owner = fund.amount.checked_div(commission);
    if amount_for_owner.is_err() {
        return Err(ContractError::FailedCommissionCalculation {});
    }
    let amount_for_owner = amount_for_owner.unwrap();
    let amount_for_seller = fund.amount.checked_sub(amount_for_owner);
    if amount_for_seller.is_err() {
        return Err(ContractError::FailedCommissionCalculation {});
    }
    let msgs = vec![
        SubMsg::new(BankMsg::Send {
            to_address: listing.listed_by_address.clone(),
            amount: vec![Coin { denom: PRICE_DENOM.to_string(), amount: amount_for_seller.unwrap() }],
        }),
        SubMsg::new(BankMsg::Send {
            to_address: OWNER_ADDR.to_string(),
            amount: vec![Coin { denom: PRICE_DENOM.to_string(), amount: amount_for_owner }],
        }),
    ];

    let seller_key = (listing.listed_by_address.as_bytes(), listing.id.as_bytes());
    let buyer_key = (info.sender.as_bytes(), listing.id.as_bytes());
    let dog = DOGS.may_load(deps.storage, seller_key)?.unwrap();
    DOGS.remove(deps.storage, seller_key);
    DOGS.save(deps.storage, buyer_key, &dog)?;

    Ok(Response::new()
        .add_submessages(msgs)
        .add_attribute("method", "try_buy_dog_on_market")
    )
}

fn try_spin_the_wheel(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let denom = PRICE_DENOM.to_string();
    let price = Uint128::from(SPIN_THE_WHEEL_PRICE_UUSD);
    let fund = info.funds.iter()
        .find(|&x| x.denom == denom && x.amount == price);
    if fund.is_none() {
        return Err(ContractError::InsufficientFunds {});
    }
    let choice = rand_int_between(env.block.time, 1u8, 100u8);
    let prize_id = generate_id(env.block.time);
    let mut prize= "none";
    println!("WHEEL SPUN: {}", choice);
    if choice <= 10u8 {
        prize = "dog";
        let class_choice = rand_int_between(env.block.time, 1u8, 100u8);
        let mut class= 0;
        if class_choice <= 20 {
            class = 0;
        } else if class_choice <= 50 {
            class = 1;
        } else if class_choice <= 77 {
            class = 2;
        } else if class_choice <= 97 {
            class = 3;
        } else if class_choice == 98 {
            class = 4;
        } else if class_choice == 99 {
            class = 5;
        } else if class_choice == 100 {
            class = 6;
        }
        let dog_name = format!("Dog #{}", prize_id.clone());
        let dog_data = create_random_dog(env.block.time, &prize_id, &dog_name, class);
        DOGS.save(
            deps.storage,
            (info.sender.as_bytes(), prize_id.as_bytes()),
            &dog_data
        )?;
    } else {
        prize = "accessory";
        let accessory_name_index = usize::from(rand_int_between(env.block.time, 1u8, 3u8) - 1u8);
        let accessory_name = ACCESSORY_NAMES[accessory_name_index];
        let accessory_data = AccessoryData {
            name: accessory_name.to_string(),
            id: prize_id.clone(),
        };
        ACCESSORIES.save(
            deps.storage,
            (info.sender.as_bytes(), prize_id.as_bytes()),
            &accessory_data
        )?;
    }
    println!("PRIZE IS {}", prize);

    Ok(Response::new()
        .add_attribute("method", "try_spin_the_wheel")
        .add_attribute("prize", prize)
        .add_attribute("prize_id", prize_id)
    )
}

fn query_game_info(deps: Deps) -> StdResult<GameInfoResponse> {
    let total_supply_dogs = DOGS
        .keys(deps.storage, None, None, Order::Ascending)
        .count();
    let total_supply_accessories = ACCESSORIES
        .keys(deps.storage, None, None, Order::Ascending)
        .count();
    return Ok(GameInfoResponse {
        total_supply_dogs,
        total_supply_accessories,
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

fn query_market_listings(deps: Deps) -> StdResult<MarketListingsResponse> {
    let listings: Vec<_> = MARKET_LISTINGS_DOGS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let mut listings_response = Vec::new();
    for listing in listings {
        let (_key, listing_data) = listing.unwrap();
        listings_response.push(listing_data);
    }
    return Ok(MarketListingsResponse {
        listings: listings_response
    });
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, from_binary, Api, OwnedDeps, Querier, Storage, BlockInfo, ContractInfo, Addr};
    // use crate::utils::print_type_of;

    fn mock_env(timestamp_nanos: u64) -> Env {
        let nanos = 1_571_797_419_879_305_533 + timestamp_nanos;
        Env {
            block: BlockInfo {
                height: 12_345,
                time: Timestamp::from_nanos(nanos),
                chain_id: "cosmos-testnet-14002".to_string(),
            },
            contract: ContractInfo {
                address: Addr::unchecked(MOCK_CONTRACT_ADDR),
            },
        }
    }

    /// checks the `address` to ensure it has the correct `expected` balance
    fn assert_balance_is<S: Storage, A: Api, Q: Querier>(
        deps: &OwnedDeps<S, A, Q>,
        address: &str,
        expected: u128,
    ) {
        let res = query(
            deps.as_ref(),
            mock_env(0),
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
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(1), QueryMsg::TokenInfo {}).unwrap();
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

        let res = instantiate(deps.as_mut(), mock_env(0), creator_info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        assert_balance_is(&deps, OWNER_ADDR, 10_000u128);
        execute(
            deps.as_mut(),
            mock_env(1),
            mock_info(OWNER_ADDR, &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(1_000),
            },
        ).unwrap();
        assert_balance_is(&deps, OWNER_ADDR, 9_000u128);

        let res = query(deps.as_ref(), mock_env(2), QueryMsg::TokenInfo {}).unwrap();
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

        let res = instantiate(deps.as_mut(), mock_env(0), creator_info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        execute(
            deps.as_mut(),
            mock_env(1),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::Transfer {
                amount: Uint128::from(2u128),
                recipient: "other_address".to_string(),
            },
        ).unwrap();
        assert_balance_is(&deps, "other_address", 2u128);
        execute(
            deps.as_mut(),
            mock_env(2),
            mock_info("other_address", &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(1),
            },
        )
        .unwrap();
        execute(
            deps.as_mut(),
            mock_env(2),
            mock_info("other_address", &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(1),
            },
        )
        .unwrap();
        assert_balance_is(&deps, "other_address", 0u128);
        execute(
            deps.as_mut(),
            mock_env(4),
            mock_info(OWNER_ADDR, &coins(1000, "earth")),
            ExecuteMsg::Burn {
                amount: Uint128::new(9998),
            },
        )
        .unwrap();
        assert_balance_is(&deps, OWNER_ADDR, 0u128);

        let res = query(deps.as_ref(), mock_env(5), QueryMsg::TokenInfo {}).unwrap();
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
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {},
        );
        assert_balance_is(&deps, OWNER_ADDR, 10_000u128);
        assert_balance_is(&deps, "other_address", 0u128);

        let res = execute(
            deps.as_mut(),
            mock_env(1),
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
            mock_env(2),
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
            mock_env(3),
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
            mock_env(4),
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
            mock_env(5),
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
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = execute(
            deps.as_mut(),
            mock_env(1),
            mock_info("other_address", &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(1u128) },
        );
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("must return unauthorized error"),
        }

        let _res = execute(
            deps.as_mut(),
            mock_env(2),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(5u128) },
        );
        let res = query(deps.as_ref(), mock_env(3), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(5, inventory.dogs.len());

        let res = query(deps.as_ref(), mock_env(4), QueryMsg::GameInfo {}).unwrap();
        let game_info: GameInfoResponse = from_binary(&res).unwrap();
        assert_eq!(5, game_info.total_supply_dogs);
    }

    #[test]
    fn inventory_empty() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(1), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(0, inventory.dogs.len());
        assert_eq!(0, inventory.accessories.len());
    }

    #[test]
    fn inventory_not_empty() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let _res = execute(
            deps.as_mut(),
            mock_env(1),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(1u128) },
        );

        let _res = execute(
            deps.as_mut(),
            mock_env(2),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintAccessory { name: String::from("Champagne"), amount: Uint128::from(1u128) },
        );

        let res = query(deps.as_ref(), mock_env(3), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(1, inventory.dogs.len());
        assert_eq!(1, inventory.accessories.len());

        let res = query(deps.as_ref(), mock_env(4), QueryMsg::Inventory { address: "other_address".to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(0, inventory.dogs.len());
        assert_eq!(0, inventory.accessories.len());
    }

    #[test]
    fn sell_dog_on_market() {
        let mut deps = mock_dependencies(&[]);

        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        // dog does not exist
        let res = execute(
            deps.as_mut(),
            mock_env(1),
            mock_info("other_address", &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: String::from("does-not-exist"),
                price: Uint128::from(500_000u128)
            },
        );
        match res {
            Err(ContractError::DogDoesNotExist {}) => {}
            _ => panic!("must return dog does not exist error")
        }

        // dog does not belong to sender (aka dog does not exist because of composite key)
        let _res = execute(
            deps.as_mut(),
            mock_env(2),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(1u128) },
        );
        let res = query(
            deps.as_ref(),
            mock_env(3),
            QueryMsg::Inventory { address: OWNER_ADDR.to_string() }
        ).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        let existing_dog_id: String = String::from(&inventory.dogs.get(0).unwrap().id);
        let res = execute(
            deps.as_mut(),
            mock_env(4),
            mock_info("other_address", &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: existing_dog_id.clone(),
                price: Uint128::from(500_000u128)
            },
        );
        match res {
            Err(ContractError::DogDoesNotExist {}) => {}
            _ => panic!("must return dog does not exist error")
        }

        // price is below 0.5 UST
        let res = execute(
            deps.as_mut(),
            mock_env(5),
            mock_info("other_address", &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: existing_dog_id.clone(),
                price: Uint128::from(499_999u128)
            },
        );
        match res {
            Err(ContractError::ListingPriceTooLow {}) => {}
            _ => panic!("must return listing price too low error")
        }

        // price is above 1,000,000 UST
        let res = execute(
            deps.as_mut(),
            mock_env(6),
            mock_info("other_address", &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: existing_dog_id.clone(),
                price: Uint128::from(1_000_000_000_001u128)
            },
        );
        match res {
            Err(ContractError::ListingPriceTooHigh {}) => {}
            _ => panic!("must return listing price too high error")
        }

        // dog is put on sale (price is within limits, dog is owned by sender)
        let res = execute(
            deps.as_mut(),
            mock_env(7),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: existing_dog_id.clone(),
                price: Uint128::from(500_000u128)
            },
        );
        match res {
            Ok(_) => {}
            _ => panic!("must place listing on sale")
        }
        let res = query(deps.as_ref(), mock_env(8), QueryMsg::MarketListings {}).unwrap();
        let market_listings: MarketListingsResponse = from_binary(&res).unwrap();
        assert_eq!(1, market_listings.listings.len(), "should have one market listing");

        // dog is already on sale
        let res = execute(
            deps.as_mut(),
            mock_env(9),
            mock_info("other_address", &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: existing_dog_id.clone(),
                price: Uint128::from(1_000_000_000_001u128)
            },
        );
        match res {
            Err(ContractError::ListingPriceTooHigh {}) => {}
            _ => panic!("must return listing price too high error")
        }
    }

    #[test]
    fn buy_dog_on_market() {
        let mut deps = mock_dependencies(&[]);
        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        // try to buy a dog that is not listed on the market
        let res = execute(
            deps.as_mut(),
            mock_env(1),
            mock_info("other_address", &[]),
            ExecuteMsg::BuyDogOnMarket {
                dog_id: String::from("non-existent-market-listing"),
            },
        );
        match res {
            Err(ContractError::ListingDoesNotExist {}) => {}
            _ => panic!("must return listing does not exist error")
        }

        // mint a dog
        let _res = execute(
            deps.as_mut(),
            mock_env(2),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::MintDog { amount: Uint128::from(1u128) },
        );
        // find the id of the dog
        let res = query(
            deps.as_ref(),
            mock_env(3),
            QueryMsg::Inventory { address: OWNER_ADDR.to_string() }
        ).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        let existing_dog_id: String = String::from(&inventory.dogs.get(0).unwrap().id);
        // put the dog on sale
        let res = execute(
            deps.as_mut(),
            mock_env(4),
            mock_info(OWNER_ADDR, &[]),
            ExecuteMsg::SellDogOnMarket {
                dog_id: existing_dog_id.clone(),
                price: Uint128::from(500_000u128)
            },
        );
        match res {
            Ok(_) => {}
            _ => panic!("must place listing on sale")
        }
        // check market listings
        let res = query(deps.as_ref(), mock_env(5), QueryMsg::MarketListings {}).unwrap();
        let market_listings: MarketListingsResponse = from_binary(&res).unwrap();
        assert_eq!(1, market_listings.listings.len(), "should have one market listing");

        // buying where funds provided do not match listing amount
        let res = execute(
            deps.as_mut(),
            mock_env(6),
            mock_info("other_address", &[]),
            ExecuteMsg::BuyDogOnMarket {
                dog_id: existing_dog_id.clone(),
            },
        );
        match res {
            Err(ContractError::DoesNotMatchListingPrice {}) => {}
            _ => panic!("must return does not match listing price error")
        }
        let res = execute(
            deps.as_mut(),
            mock_env(7),
            mock_info("other_address", &[Coin { amount: Uint128::from(499_999u128), denom: "uusd".to_string() }]),
            ExecuteMsg::BuyDogOnMarket {
                dog_id: existing_dog_id.clone(),
            },
        );
        match res {
            Err(ContractError::DoesNotMatchListingPrice {}) => {}
            _ => panic!("must return does not match listing price error")
        }
        let res = execute(
            deps.as_mut(),
            mock_env(8),
            mock_info("other_address", &[Coin { amount: Uint128::from(500_001u128), denom: "uusd".to_string() }]),
            ExecuteMsg::BuyDogOnMarket {
                dog_id: existing_dog_id.clone(),
            },
        );
        match res {
            Err(ContractError::DoesNotMatchListingPrice {}) => {}
            _ => panic!("must return does not match listing price error")
        }
        let res = execute(
            deps.as_mut(),
            mock_env(9),
            mock_info("other_address", &[Coin { amount: Uint128::from(500_000u128), denom: "uluna".to_string() }]),
            ExecuteMsg::BuyDogOnMarket {
                dog_id: existing_dog_id.clone(),
            },
        );
        match res {
            Err(ContractError::DoesNotMatchListingPrice {}) => {}
            _ => panic!("must return does not match listing price error")
        }

        // check current inventory status
        let res = query(deps.as_ref(), mock_env(10), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(1, inventory.dogs.len());

        let res = query(deps.as_ref(), mock_env(11), QueryMsg::Inventory { address: "other_address".to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(0, inventory.dogs.len());

        // buying the dog transfers it from one account to another
        let res = execute(
            deps.as_mut(),
            mock_env(12),
            mock_info("other_address", &[Coin { amount: Uint128::from(500_000u128), denom: "uusd".to_string() }]),
            ExecuteMsg::BuyDogOnMarket {
                dog_id: existing_dog_id.clone(),
            },
        );
        match res {
            Ok(res) => { assert_eq!(2, res.messages.len(), "needs to send two messages to transfer funds") }
            _ => panic!("must not return any error")
        }
        // total dog supply remains the same
        // inventory changes for parties involved in transaction
        let res = query(deps.as_ref(), mock_env(13), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(0, inventory.dogs.len(), "dog should no longer be part of seller's inventory");

        let res = query(deps.as_ref(), mock_env(14), QueryMsg::Inventory { address: "other_address".to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        assert_eq!(1, inventory.dogs.len(), "dog should be part of buyer's inventory");
    }

    #[test]
    fn spin_the_wheel() {
        let mut deps = mock_dependencies(&[]);
        let res = instantiate(
            deps.as_mut(),
            mock_env(0),
            mock_info("creator_address", &coins(1000, "earth")),
            InstantiateMsg {}
        ).unwrap();
        assert_eq!(0, res.messages.len());

        let res = execute(
            deps.as_mut(),
            mock_env(1),
            mock_info("player_address", &[Coin { amount: Uint128::from(999_999u128), denom: "uusd".to_string() }]),
            ExecuteMsg::SpinTheWheel {},
        );
        match res {
            Err(ContractError::InsufficientFunds {}) => {}
            _ => panic!("must return insufficient funds error")
        }

        // spin the wheel
        let res = execute(
            deps.as_mut(),
            mock_env(2),
            mock_info("player_address", &[Coin { amount: Uint128::from(1_000_000u128), denom: "uusd".to_string() }]),
            ExecuteMsg::SpinTheWheel {},
        ).unwrap();
        assert_eq!(0, res.messages.len());

        // check the inventory of the player, they should receive an item
        let res = query(deps.as_ref(), mock_env(3), QueryMsg::Inventory { address: "player_address".to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        println!("SPIN THE WHEEL");
        println!("{:?}", inventory);
        if inventory.dogs.len() == 0 {
            assert_eq!(1, inventory.accessories.len());
        } else if inventory.accessories.len() == 0 {
            assert_eq!(1, inventory.dogs.len());
        }

        // check the inventory of contract owner address, should receive nothing
        let res = query(deps.as_ref(), mock_env(4), QueryMsg::Inventory { address: OWNER_ADDR.to_string() }).unwrap();
        let inventory: InventoryResponse = from_binary(&res).unwrap();
        println!("SPIN THE WHEEL");
        println!("{:?}", inventory);
        assert_eq!(0, inventory.dogs.len());
        assert_eq!(0, inventory.accessories.len());
    }
}
