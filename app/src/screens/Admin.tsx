import React, {useState, useEffect, useCallback, useMemo} from "react"
import {
  CreateTxFailed,
  Timeout,
  TxFailed,
  TxResult,
  TxUnspecifiedError,
  useConnectedWallet,
  UserDenied,
} from '@terra-money/wallet-provider';
import {hideGame} from "../game";
import {Coin, Coins, CreateTxOptions, Denom, LCDClient, MsgExecuteContract, StdFee} from "@terra-money/terra.js";
import {CONTRACT_ADDRESS} from "../components/constants";

enum MintStatus {
  NotInProgress = '',
  InProgress = 'in progress',
  Completed = 'completed',
  Error = 'error'
};

export default function Admin() {
  useEffect(() => { hideGame(); }, []);
  const connectedWallet = useConnectedWallet();
  const lcd = useMemo(() => {
    if (!connectedWallet) {
      return null;
    }
    // const gas_response = await fetch(
    //   "https://fcd.terra.dev/v1/txs/gas_prices",
    // );
    // const gas_json = await gas_response.json();
    return new LCDClient({
      URL: connectedWallet.network.lcd,
      chainID: connectedWallet.network.chainID,
      // gasPrices: {
      //   uusd: gas_json.uusd,
      // },
      // gasAdjustment: "1.4",
    });
  }, [connectedWallet]);

  const [mintStatus, setMintStatus] = useState<MintStatus>(MintStatus.NotInProgress);
  const [mintAmount, setMintAmount] = useState<string>("0");
  const executeMint = () => {
    if (!lcd || !connectedWallet || mintStatus == MintStatus.InProgress || mintAmount === "0") {
      return;
    }

    // const coins = Coins.fromString("100000uluna");
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        mint: {
          amount: mintAmount,
        },
      },
      { uluna: 50000 },
    );
    // const estimatedFee = await lcd.tx.estimateFee(
    //   connectedWallet.walletAddress,
    //   [executeMsg],
    // );
    // console.log(estimatedFee);

    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      /*
       StdFee(requested_gas, fee). requested gas is like a ceiling gas you'd want to
       spend for this tx specifically, and fee is how much you want to spend for this
       gas ceiling.

       So for example if your contract execution were to take 300,000 gas, it's ok as
       long as gas_requested is whatever above 300,000. For fee -- as long as you are
       willing to spend more than min gas price per unit gas it's fine.
      */
      fee: new StdFee(1000000, { uluna: 200000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log(nextTxResult);
    }).catch((error: unknown) => {
      console.error(error);
      setMintStatus(MintStatus.Error);
    });
  };
  const executeTransfer = () => {
    if (!lcd || !connectedWallet) {
      return;
    }
    const executeMsg = new MsgExecuteContract(
      connectedWallet.walletAddress,
      CONTRACT_ADDRESS,
      {
        transfer: {
          amount: "5000",
          recipient: "terra18d2448f244wf9f7l69w66nq0ffvh8j989rn85l",
        }
      },
      { uluna: 50000 },
    );
    const tx: CreateTxOptions = {
      msgs: [executeMsg],
      fee: new StdFee(1000000, { uluna: 200000 }),
    };
    connectedWallet.post(tx).then(nextTxResult => {
      console.log(nextTxResult);
    }).catch(error => {
      console.error(error);
    });
  };
  return (<div>
    <section>
      <h3>Mint</h3>
      <p><input type="text" value={mintAmount} onChange={e => {
        setMintAmount(e.target.value);
      }}/></p>
      <p><button onClick={() => executeMint()} disabled={mintStatus == MintStatus.InProgress}>Mint</button> {mintStatus}</p>
    </section>
    <section>
      <p><button onClick={() => executeTransfer()}>Transfer</button></p>
    </section>
  </div>);
}
