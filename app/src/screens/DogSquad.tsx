import React, {useEffect, useMemo, useState} from "react";
import {hideGame} from "../game";
import {Accessory, Dog, InventoryResponse} from "../types";
import {CONTRACT_ADDRESS} from "../components/constants";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {CreateTxOptions, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import './DogSquad.css';
import {ACCESSORY_IMAGE, DOG_ATTR_NAME, DOG_CLASS_NAME, DOG_IMAGE} from "../constants";
import {Link} from "react-router-dom";
import GameHint from "../components/GameHint";
import Button from "../components/Button";

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

  const [dogSaleAmount, setDogSaleAmount] = useState<string>("1");
  const executeSellDogOnMarket = (dog_id: string, price_in_uusd: string) => {
    if (!connectedWallet || !lcd) {
      return;
    }
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        sell_dog_on_market: {
          dog_id: dog_id,
          price: price_in_uusd,
        },
      },
      { uusd: 1 },
    );
    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      fee: new StdFee(1000000, { uusd: 200000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log("Dog listed for sale");
    }).catch((error: unknown) => {
      console.error(error);
    });
  }

  const [dogs, setDogs] = useState<Dog[]>();
  const [accessories, setAccessories] = useState<Accessory[]>();
  return (<div className="screen-dog-squad">
    <section className="dog-list">
      <header>
        <GameHint align={"left"}>
          Click on a dog in your dog squad collection to view more details and accessories!
        </GameHint>
        <h3>{dogs?.length} Dogs</h3>
      </header>
      <section className="dog-items">
        {dogs?.map(dog =>
          <div className="dog-item" key={dog.id}>
            <header>
              <Link to={`/dog/${dog.id}`}>
                <h4>{dog.name}</h4>
                <h5>{DOG_CLASS_NAME[dog.class]}</h5>
              </Link>
            </header>
            <section>
              <div>
                <Link to={`/dog/${dog.id}`}>
                  <img src={DOG_IMAGE[dog.class]} />
                </Link>
              </div>
              <div className="dog-item-attributes">
                <div className="dog-item-attribute">{DOG_ATTR_NAME[0]}: {dog.attr1}</div>
                <div className="dog-item-attribute">{DOG_ATTR_NAME[1]}: {dog.attr2}</div>
                <div className="dog-item-attribute">{DOG_ATTR_NAME[2]}: {dog.attr3}</div>
                <div className="dog-item-attribute">{DOG_ATTR_NAME[3]}: {dog.attr4}</div>
              </div>
            </section>
            <footer>
              Amount (USDT): <input className="dog-item-sale-amount" type="text" value={dogSaleAmount} onChange={e => {
                setDogSaleAmount(e.target.value);
              }}/>
              <Button onClick={() => executeSellDogOnMarket(dog.id, `${dogSaleAmount}000000`)}>
                List for sale
              </Button>
            </footer>
          </div>
        )}
      </section>
    </section>
    <hr />
    <section className="accessory-list">
      <header>
        <GameHint align={"left"}>
          Play <Link to={"/play"}>Spin the Paw</Link> to collect more accessories!
        </GameHint>
        <h3>{accessories?.length} Accessories</h3>
      </header>
      <section className="accessory-items">
        {accessories?.map(accessory =>
          <div className="accessory-item" key={accessory.id}>
            <div className="accessory-item-name">{accessory.name.toUpperCase()}</div>
            <div className="accessory-item-image">
              <img src={ACCESSORY_IMAGE[accessory.name]} />
            </div>
          </div>
        )}
      </section>
    </section>
  </div>);
}
