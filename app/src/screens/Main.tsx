import React, {useEffect} from "react";
// import { ConnectSample } from 'components/ConnectSample';
// import { QuerySample } from 'components/QuerySample';
// import { TxSample } from 'components/TxSample';
import {useWallet, WalletStatus} from "@terra-money/wallet-provider";
import {hideGame} from "../game";
import {GAME_TITLE} from "../game/constants";
import ConnectWalletButtons from "../components/ConnectWalletButtons";
import GameHint from "../components/GameHint";
import "./Main.css";

export default function Main() {
  useEffect(() => { hideGame(); }, []);
  const { status } = useWallet();
  return (
    <section className="screen-main">
      <header>
        <h1>{GAME_TITLE}</h1>
      </header>
      <section>
        <header>
          <h2>How to Play</h2>
        </header>
        <section>
          <p>1. <a target="_blank" href="https://chrome.google.com/webstore/detail/terra-station/aiifbnbfobpmeekipheeijimdpnlpgpp/">Click here to download the Terra Station wallet for your browser</a></p>
          <p>2. Create your wallet. Scroll down for step-by-step instructions on how to do this.</p>
          <p>3. Log into the game by connecting your wallet</p>
        </section>
      </section>
      <section>
        {status == WalletStatus.WALLET_NOT_CONNECTED && (<GameHint align={"right"}>
          Click "Log in with Terra Station Browser Wallet"
        </GameHint>)}
        <ConnectWalletButtons />
      </section>
      <section>
        <header>
          <h2>About {GAME_TITLE}</h2>
        </header>
        <section>
          <h3>What is {GAME_TITLE} Beta 1.0?</h3>
          <p>Do you love dogs? Are you a fan of cryptocurrency-backed innovation? If you answered YES to one of these then this is the game for you! {GAME_TITLE} is an interactive community that allows you to take care of multiple unique dogs, grow their awesome traits (such as *fluffiness* and *squirrel factor*), purchase special accessories, and increase the value of your dog-squad through continued play.</p>
          <p>Using the Terra Station Wallet, this game will let you purchase/sell/trade your dogs and win new rare dogs and accessories by playing Spin-the-Paw - a brilliant action packed Casino-style game! Each dog is your special asset and is secured through blockchain Smart Contract technology. {GAME_TITLE} Beta 1.0 is our team's initiative to test the limits and interactivity of cryptocurrency-based gaming environments. As a team we bring foresight to the future of game development, a future that is open source, live-operations-based, and fun for gamers and developers.</p>
          <h3>How To Register for {GAME_TITLE} (via Chrome Browser and Terra Station Wallet)</h3>
          <p>To access, play, and help test {GAME_TITLE} Beta 1.0 you will need to complete the following steps:</p>
          <p>1. Download the latest version of the Google Chrome browser, available here: <a target="_blank" href="https://www.google.com/intl/en_ca/chrome/">https://www.google.com/intl/en_ca/chrome/</a></p>
          <p>2. Download the Google Chrome browser extension: <a target="_blank" href="https://chrome.google.com/webstore/detail/terra-station/aiifbnbfobpmeekipheeijimdpnlpgpp?hl=en">Terra Station Extension, available here: https://chrome.google.com/webstore/detail/terra-station/aiifbnbfobpmeekipheeijimdpnlpgpp?hl=en</a> | The Terra Station web extension is your user account for the Terra Station wallet. You will use this web extension to play the game and conduct all of the relevant in-game transactions. The Terra Station wallet and its Google Chrome browser extension allow you to access decentralized applications (DApps) powered by smart contracts on the Terra blockchain.</p>
          <p>3. Open the Google Chrome Terra Station extension by clicking the Terra Station icon located in the upper right corner of the browser window.</p>
          <p>4. Select &lt;New Wallet&gt; from the Google Chrome Terra Station extension. Or, use the &lt;Recover Wallet&gt; function if you would prefer to use an existing Terra Wallet to play {GAME_TITLE} Beta 1.0.</p>
          <p>5.If you are creating a New Terra Station Wallet, follow the prompt to fill out the form with the following information: new Wallet Name, new Password, and Confirm Password.</p>
          <p>6. For the {GAME_TITLE} Beta 1.0, please change from <em>*Mainnet*</em> in the drop down menu at the top of the form to <em>*Bombay*</em>.</p>
          <p>7. Make sure to write down the 24-word Seed Phrase - this phrase is a security measure and if lost you will not be able to recover your Terra Station Wallet and its assets.</p>
          <p>8. Once you have changed from Mainnet to Bombay, and filled out the form, and secured your 24-word Seed Phrase, Press &lt;Next&gt; to complete the creation of your new Terra Station Wallet.</p>
          <p>9. In a new Google Chrome browser window, go to {GAME_TITLE} Beta 1.0 website, available here: <a href="/">https://pawsintheparkthegame.s3-website-us-east-1.amazonaws.com/</a></p>
          <p>10. If this is your first time logging into {GAME_TITLE} Beta 1.0, you will need to press the &lt;Log in with Terra Station&gt; button. If you have already logged in, the game Welcome screen will appear automatically.</p>
          <h3>What is the Terra Station Wallet and Google Chrome Terra Station Extension?</h3>
          <p>The Terra Station web extension is your user account for the Terra Station wallet. You will use this web extension to play the game and conduct all of the relevant in-game transactions. The Terra Station wallet and its Google Chrome browser extension allow you to access decentralized applications (DApps) powered by smart contracts on the Terra blockchain. The Google Chrome extension also includes the official wallet from the desktop edition of Terra Station, which users can use to manage their Terra accounts ( supported by Ledger). This Google Chrome plugin enables webpages to connect to the Terra network, allowing them to pull live state and events from the blockchain as well as create new transactions.</p>
          <h3>How to Report a game bug for {GAME_TITLE} Beta 1.0?</h3>
          <p>If you have encountered an issue in {GAME_TITLE} Beta 1.0 please report the bug by submitting a report, available here: <a target="_blank" href="https://forms.gle/tPWSBdr1xTGB3LbFA">https://forms.gle/tPWSBdr1xTGB3LbFA</a></p>
          <h3>How to Subscribe to {GAME_TITLE} community to receive news and updates?</h3>
          <p>Woof woof! If you would like to join the {GAME_TITLE} community to receive news and updates on upcoming in-game events, please submit your email here: <a target="_blank" href="https://forms.gle/RpUQgFe8MoGDDsKLA">https://forms.gle/RpUQgFe8MoGDDsKLA</a></p>
        </section>
      </section>
      {/*<div>*/}
      {/*  <ConnectSample />*/}
      {/*  <QuerySample />*/}
      {/*  <TxSample />*/}
      {/*</div>*/}
    </section>
  );
}
