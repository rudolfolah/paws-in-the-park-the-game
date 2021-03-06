import React, {useEffect, useMemo, useState} from "react";
import {hideGame} from "../game";
import {BalanceResponse, MarketListing, MarketListingsResponse} from "../types";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {CreateTxOptions, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import {CONTRACT_ADDRESS} from "../components/constants";
import Button from "../components/Button";

import "./Market.css";
import GameHint from "../components/GameHint";

export default function Market() {
  useEffect(() => { hideGame(); }, []);

  const connectedWallet = useConnectedWallet();
  const lcd = useMemo(() => {
    if (!connectedWallet) {
      return null;
    }
    return new LCDClient({
      URL: connectedWallet.network.lcd,
      chainID: connectedWallet.network.chainID,
    })
  }, [connectedWallet]);
  const [listings, setListings] = useState<MarketListing[]>([]);
  useEffect(() => {
    if (connectedWallet && lcd) {
      lcd.wasm.contractQuery<MarketListingsResponse>(CONTRACT_ADDRESS, {
        "market_listings": {},
      }).then(result => {
        setListings(result.listings);
      }).catch(error => {
        console.error(error);
        setListings([]);
      })
    } else {
      setListings([]);
    }
  }, [connectedWallet, lcd]);

  const executeBuyDogOnMarket = (dog_id: string, price_in_uusd: string) => {
    if (!connectedWallet || !lcd) {
      return;
    }
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        buy_dog_on_market: {
          dog_id: dog_id,
        },
      },
      { uusd: price_in_uusd },
    );
    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      fee: new StdFee(1000000, { uusd: 200000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log(`Dog bought for ${price_in_uusd}`);
    }).catch((error: unknown) => {
      console.error(error);
    });
  }

  return (<section className="screen-market">
    <header>
      <h3>Market Listings</h3>
      <GameHint align={"right"}>
        You can buy dogs that other players have listed for sale!
      </GameHint>
    </header>
    <section className="market-listings">
      {listings?.map(listing =>
        <div className="market-listing" key={listing.id}>
          <section>
            <div className="market-listing-title">Dog #{listing.id}</div>
            <div className="market-listing-listed-by">
              Listed by <a target="_blank" href={`https://finder.terra.money/bombay-10/account/${listing.listed_by_address}`}>
                {listing.listed_by_address}
              </a>
            </div>
          </section>
          <footer>
            <Button onClick={() => executeBuyDogOnMarket(listing.id, listing.price.amount)}>
              Buy this dog for {parseFloat(listing.price.amount) / 1000000.0} USDT
            </Button>
          </footer>
        </div>
      )}
    </section>
  </section>);
}
