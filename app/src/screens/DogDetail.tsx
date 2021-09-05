import React, {useEffect, useMemo, useState} from "react";
import { useParams } from "react-router-dom";
import {hideGame, setGameData, showGame} from "../game";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {CreateTxOptions, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import {Accessory, Dog, InventoryResponse} from "../types";
import {CONTRACT_ADDRESS} from "../components/constants";

export default function DogDetail() {
  useEffect(() => { hideGame() }, []);
  let { dogId } = useParams<{ dogId: string }>();
  const [dog, setDog] = useState<Dog|null>(null);
  const [accessories, setAccessories] = useState<Accessory[]>();
  useEffect(() => {
    if (dog && accessories) {
      setGameData("dog", dog);
      setGameData("accessories", accessories);
      showGame("detail");
    } else {
      setGameData("dog", null);
      setGameData("accessories", null);
      hideGame();
    }
  }, [dog, accessories]);

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
        setDog(result.dogs.find(dog => dog.id === dogId) || null);
        setAccessories(result.accessories);
      });
    } else {
      setDog(null);
      setAccessories([]);
    }
  }, [connectedWallet, lcd]);
  return (<></>);
}
