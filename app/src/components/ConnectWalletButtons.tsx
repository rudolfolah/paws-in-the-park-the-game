import { useWallet, WalletStatus } from '@terra-money/wallet-provider';
import React from 'react';
import Button from "./Button";
import {CONNECT_WALLET_BUTTON_LABEL} from "./constants";

export default function ConnectWalletButtons() {
  const {
    status,
    network,
    wallets,
    availableConnectTypes,
    availableInstallTypes,
    connect,
    install,
    disconnect,
  } = useWallet();

  return (
    <div>
      {status === WalletStatus.WALLET_NOT_CONNECTED && (
        <>
          {availableInstallTypes.map((connectType) => (
            <Button
              key={'install-' + connectType}
              onClick={() => install(connectType)}
            >
              Install {connectType}
            </Button>
          ))}
          {availableConnectTypes.map((connectType) => (
            <Button
              key={'connect-' + connectType}
              onClick={() => connect(connectType)}
            >
              Log in using {CONNECT_WALLET_BUTTON_LABEL[connectType]}
            </Button>
          ))}
        </>
      )}
      {status === WalletStatus.WALLET_CONNECTED && (
        <Button onClick={() => disconnect()}>Log out</Button>
      )}
    </div>
  );
}
