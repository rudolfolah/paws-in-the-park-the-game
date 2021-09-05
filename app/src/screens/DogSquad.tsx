import React, {useEffect, useMemo, useState} from "react";
import {hideGame} from "../game";
import {Accessory, Dog, InventoryResponse} from "../types";
import {DogInfo} from "../components/DogInfo";
import {CONTRACT_ADDRESS} from "../components/constants";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {LCDClient} from "@terra-money/terra.js";

const ACCESSORY_IMAGE: {[index: string]: string} = {
  "martini glass": "assets/accessory-martini.png",
  "sparkle": "assets/accessory-sparkle.png",
  "star": "assets/accessory-rainbow-star.png",
};

export default function DogSquad() {
  useEffect(() => { hideGame() }, []);

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
  useEffect(() => {
    if (connectedWallet && lcd) {
      lcd.wasm.contractQuery<InventoryResponse>(CONTRACT_ADDRESS, {
        "inventory": {
          "address": connectedWallet.walletAddress,
        },
      }).then(result => {
        setDogs(result.dogs);
        setAccessories(result.accessories);
      });
    } else {
      setDogs([]);
      setAccessories([]);
    }
  }, [connectedWallet, lcd]);

  const [dogs, setDogs] = useState<Dog[]>();
  const [accessories, setAccessories] = useState<Accessory[]>();
  return (<div>
    {dogs?.length}
    <section className="accessory-list">
      <header>
        <h3>Accessories</h3>
      </header>
      <section className="accessory-items">
        {accessories?.map(accessory =>
          <div className="accessory-item" key={accessory.id}>
            <img src={ACCESSORY_IMAGE[accessory.name]} />
            {accessory.name}
          </div>
        )}
      </section>
    </section>
  </div>);
}
