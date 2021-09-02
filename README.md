# my-pet-pal

Take care of your pet, buy/sell your pet and items on the blockchain

Copyright (C) 2021 Rudolf Olah <rudolf.olah.to@gmail.com>

- `app`: React + Kaboom.js game frontend
- `game-contract`: Smart contract for the game, including [CW721 NFTs](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw721)
- `token`: Tailwag (TAG) token smart contract, [CW20 Fungible Token](https://github.com/CosmWasm/cw-plus/tree/main/packages/cw20)

## Setup required for Smart Contracts

You will need:
- Go
- Rust
- LocalTerra
- terrad, from Terra Core (need to compile the source code from https://github.com/terra-money/core)
- Docker

You will need to build terrad, see: https://docs.terra.money/node/installation.html#building-terra-core

Terrad can be installed like this (from the `core` repo):

```shell
cd terra-money-core
make install
# sudo GOPATH=/usr/local/go/bin GOBIN=/usr/local/bin make install
```

You can use the Terra Station desktop app to upload a contract (instead of using terrad).

1. Go to "Contracts"
2. Click "Upload" and select the `.wasm` file
3. After upload, check your "History" for the transaction, it will contain the *code* of the contract
4. Go to "Contracts"
5. Click "Create" and enter the *code* and enter the instantiation JSON message
6. After creation, refresh the "Contracts" and take nate of the transaction which shows the contract address

You will need to run local terra, see: https://docs.terra.money/contracts/tutorial/interacting.html#requirements

*NOTE: terrad is the replacement for terracli.*

Set up the testnet configuration like this:

```shell
mkdir -p ~/.terrad/config
curl https://raw.githubusercontent.com/terra-money/testnet/master/bombay-10/genesis.json > ~/.terra/config/genesis.json
curl https://network.terra.dev/addrbook.json > ~/.terra/config/addrbook.json
sed -i 's/minimum-gas-prices = "0uluna"/minimum-gas-prices = "0.15uluna,0.1018usdr,0.15uusd,178.05ukrw,431.6259umnt,0.125ueur,0.97ucny,16.0ujpy,0.11ugbp,11.0uinr,0.19ucad,0.13uchf,0.19uaud,0.2usgd,4.62uthb,1.25usek,1.164uhkd,0.9udkk,1.25unok,2180.0uidr,7.6uphp"/g' ~/.terra/config/app.toml
# add the following to ~/.terra/config/config.toml in the seeds field
# 8eca04192d4d4d7da32149a3daedc4c24b75f4e7@3.34.163.215:26656
export TERRAD=~/go/bin/terrad
$TERRAD start
$TERRAD status
```

For local terra you can do this;

```shell
export TERRAD=/path/to/LocalTerra/terracore/terrad
$TERRAD keys add test1 --recover
# Enter the mnemnonic
# satisfy adjust timber high purchase tuition stool faith fine install that you unaware feed domain license impose boss human eager hat rent enjoy dawn
```

For the testnet you can recover your own wallet account:

```shell
$TERRAD keys add test-wallet
# Copy the seed phrase
```

Now you can deploy the smart contract and interact with it:

```shell
export WALLET_NAME=test1 # for local terra
export CHAIN_ID=localterra # for local terra
export WALLET_NAME='test-wallet' # for testnet
export CHAIN_ID='bombay-10' # for testnet
$TERRAD tx wasm store ./token/artifacts/token.wasm --from $WALLET_NAME --chain-id=$CHAIN_ID --gas=auto --fees=100000uluna --broadcast-mode=block
$TERRAD tx wasm instantiate 1 '{"total_supply":"10"}' --from $WALLET_NAME --chain-id=$CHAIN_ID --fees=10000uluna --gas=auto --broadcast-mode=block
# Get the contract address from the previous command's output
# export CONTRACT_ADDRESS=
$TERRAD query wasm contract $CONTRACT_ADDRESS
$TERRAD query wasm contract-store $CONTRACT_ADDRESS '{"token_info":{}}'
$TERRAD tx wasm execute $CONTRACT_ADDRESS '{"transfer":{"count":5}}' --from $WALLET_NAME --chain-id=$CHAIN_ID --fees=1000000uluna --gas=auto --broadcast-mode=block
$TERRAD tx wasm execute $CONTRACT_ADDRESS '{"increment":{}}' --from $WALLET_NAME --chain-id=$CHAIN_ID --gas=auto --fees=1000000uluna --broadcast-mode=block
$TERRAD query wasm contract-store $CONTRACT_ADDRESS '{"balance":{}}'
```