import React, {useEffect, useMemo} from "react";
import { useParams } from "react-router-dom";
import {setGameData, showGame} from "../game";
import {useConnectedWallet} from "@terra-money/wallet-provider";
import {CreateTxOptions, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import {InventoryResponse} from "../types";
import {CONTRACT_ADDRESS} from "../components/constants";

export default function Slots() {
  useEffect(() => {
    showGame("slots");
  }, []);

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
    // @ts-ignore
    document["_pawsinthepark_execute_spin"] = function(callback) {
      if (!connectedWallet || !lcd) {
        return;
      }
      const executeMsg = new MsgExecuteContract(
        connectedWallet.walletAddress,
        CONTRACT_ADDRESS,
        {
          spin_the_wheel: {},
        },
        { uusd: 1000000 },
      );
      const tx: CreateTxOptions = {
        msgs: [executeMsg],
        fee: new StdFee(1000000, { uusd: 200000 }),
      };
      connectedWallet.post(tx).then(nextTxResult => {
        callback(); // TODO: pass the prize info on
      }).catch((error: unknown) => {
        callback(); // TODO: pass the error on
        console.error(error);
      });
    };
  }, [connectedWallet, lcd]);
  return (<></>);
}
