import React, {useEffect, useMemo, useState} from "react";
import {hideGame} from "../game";
import {Accessory, Dog, InventoryResponse} from "../types";
import {DogInfo} from "../components/DogInfo";
import {CONTRACT_ADDRESS} from "../components/constants";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {LCDClient} from "@terra-money/terra.js";
import './DogSquad.css';

const ACCESSORY_IMAGE: {[index: string]: string} = {
  "martini glass": "assets/accessory-martini.png",
  "sparkle": "assets/accessory-sparkle.png",
  "star": "assets/accessory-rainbow-star.png",
};

const DOG_IMAGE: {[index: number]: string} = {
  0: "assets/dog01.png",
  1: "assets/dog02.png",
  2: "assets/dog06.png",
  3: "assets/dog04.png",
  4: "assets/dog05.png",
  5: "assets/dog07.png",
  6: "assets/dog03.png",
};

const DOG_CLASS_NAME: {[index: number]: string} = {
  0: "Huggable",
  1: "Curlytail",
  2: "Pointer",
  3: "Margaritaville",
  4: "Spacedog",
  5: "Friendo",
  6: "Woofington",
};

const DOG_ATTR_NAME: {[index: number]: string} = {
  0: "Floofiness",
  1: "Curiosity",
  2: "Agility",
  3: "Squirrel Factor",
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
  return (<div className="screen-dog-squad">
    <section className="dog-list">
      <header>
        <h3>{dogs?.length} Dogs</h3>
      </header>
      <section className="dog-items">
        {dogs?.map(dog =>
          <div className="dog-item" key={dog.id}>
            <header>
              <h4>{dog.name}</h4>
              <h5>{DOG_CLASS_NAME[dog.class]}</h5>
            </header>
            <section>
              <div>
                <img src={DOG_IMAGE[dog.class]} />
              </div>
              <div>
                <div>{DOG_ATTR_NAME[0]}: {dog.attr1}</div>
                <div>{DOG_ATTR_NAME[1]}: {dog.attr2}</div>
                <div>{DOG_ATTR_NAME[2]}: {dog.attr3}</div>
                <div>{DOG_ATTR_NAME[3]}: {dog.attr4}</div>
              </div>
            </section>
            <footer>
              <button>Sell for 1 USDT</button>
              <button>Sell for 5 USDT</button>
            </footer>
          </div>
        )}
      </section>
    </section>
    <section className="accessory-list">
      <header>
        <h3>{accessories?.length} Accessories</h3>
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
