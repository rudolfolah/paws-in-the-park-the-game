import React, {useEffect, useMemo, useState} from "react";
import {hideGame} from "../game";
import {BalanceResponse, MarketListing, MarketListingsResponse} from "../types";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {LCDClient} from "@terra-money/terra.js";
import {CONTRACT_ADDRESS} from "../components/constants";

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
  return (<section>
    <header>
      <h3>Market Listings</h3>
    </header>
    <section>
      {listings?.map(listing =>
        <div key={listing.id}>
          <div>
            <div>Dog #{listing.id}</div>
            <div>Listed by #{listing.listed_by_address}</div>
          </div>
          <div>
            <button>{listing.price.amount} {listing.price.denom}<br/>BUY</button>
          </div>
        </div>
      )}
    </section>
  </section>);
}
