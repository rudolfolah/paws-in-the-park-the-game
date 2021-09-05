export interface Coin {
  denom: string;
  amount: string;
}

export interface Accessory {
  // from the smart contract
  id: string;
  name: string;
}

export interface Dog {
  biography: string;
  // from the smart contract
  id: string;
  name: string;
  class: number;
  attr1: number;
  attr2: number;
  attr3: number;
  attr4: number;
}

export interface MarketListing {
  // from the smart contract
  listed_by_address: string;
  id: string;
  price: Coin;
}

export interface BalanceResponse {
  balance: string;
}

export interface GameInfoResponse {
  total_supply_dogs: number;
  total_suppy_accessories: number;
}

export interface InventoryResponse {
  // from the smart contract
  address: string;
  dogs: Dog[];
  accessories: Accessory[];
}

export interface MarketListingsResponse {
  // from the smart contract
  listings: MarketListing[];
}

export interface TokenInfoResponse {
  name: string;
  symbol: string;
  decimals: number;
  total_supply: string;
}

export interface TransferRequest {
  recipient: string;
  amount: string;
}

export interface BurnRequest {
  amount: string;
}

export interface SendRequest {
  contract: string;
  amount: string;
  msg: any;
}

export interface MintRequest {
  amount: string;
}

export interface MintDogRequest {
  amount: string;
}

export interface MintAccessoryRequest {
  name: string;
  amount: string;
}

export interface SellDogOnMarketRequest {
  dog_id: string;
  price: string;
}

export interface BuyDogOnMarketRequest {
  dog_id: string;
}

export interface SpinTheWheelRequest {}
