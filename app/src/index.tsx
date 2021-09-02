import React from 'react';
import ReactDOM from 'react-dom';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link
} from "react-router-dom";
import { NetworkInfo, WalletProvider } from '@terra-money/wallet-provider';

import Admin from 'screens/Admin';
import Main from 'screens/Main';
import Market from 'screens/Market';
import DogDetail from "screens/DogDetail";
import DogSquad from "screens/DogSquad";
import Slots from "screens/Slots";
import './style.css';
import './game';
import {TokenBalance} from "components/TokenBalance";

const mainnet = {
  name: 'mainnet',
  chainID: 'columbus-4',
  lcd: 'https://lcd.terra.dev',
};

const testnet = {
  name: 'bombay',
  chainID: 'bombay-10',
  lcd: 'https://bombay-lcd.terra.dev',
};

const walletConnectChainIds: Record<number, NetworkInfo> = {
  0: testnet,
  1: mainnet,
};

ReactDOM.render(
  <WalletProvider
    defaultNetwork={testnet}
    walletConnectChainIds={walletConnectChainIds}
  >
    <Router>
      <div id="nav">
        <Link to="/squad">Dog Squad</Link>
        <Link to="/market">Market</Link>
        <Link to="/play">Play the Game</Link>
        <Link to="/">About</Link>
        <TokenBalance />
      </div>
      <Switch>
        <Route path="/squad">
          <DogSquad />
        </Route>
        <Route path="/dog/:dogId">
          <DogDetail />
        </Route>
        <Route path="/market">
          <Market />
        </Route>
        <Route path="/play">
          <Slots />
        </Route>
        <Route path="/admin">
          <Admin />
        </Route>
        <Route path="/">
          <Main />
        </Route>
      </Switch>
    </Router>
  </WalletProvider>,
  document.getElementById('root'),
);
