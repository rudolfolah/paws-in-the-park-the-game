import React, {useEffect} from "react";
import { ConnectSample } from 'components/ConnectSample';
import { QuerySample } from 'components/QuerySample';
import { TxSample } from 'components/TxSample';
import {hideGame} from "../game";
import About from "../components/About";
import HowToPlay from "../components/HowToPlay";

export default function Main() {
  useEffect(() => { hideGame(); }, []);
  return (
    <div>
      <h1>Paws in the Park</h1>
      <HowToPlay />
      <About />
      <div>
        <ConnectSample />
        <QuerySample />
        <TxSample />
      </div>
    </div>
  );
}
