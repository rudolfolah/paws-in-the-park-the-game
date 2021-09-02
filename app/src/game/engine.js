import kaboom from "../../node_modules/kaboom/dist/kaboom.mjs";
import { WIDTH, HEIGHT } from "./constants";

const k = kaboom({
  clearColor: [0.1, 0.1, 0.2, 1],
  canvas: document.getElementById("game"),
  width: WIDTH,
  height: HEIGHT
});

k.loadSprite("reel", "/assets/reel.png");
k.loadSprite("frame", "/assets/frame.png");
k.loadSprite("dog01", "/assets/dog01.png");
k.loadSprite("dog02", "/assets/dog02.png");
k.loadSprite("dog03", "/assets/dog03.png");
k.loadSprite("dog04", "/assets/dog04.png");
k.loadSprite("dog05", "/assets/dog05.png");

k.loadSound("spin", "/assets/spin.mp3");
k.loadSound("winner", "/assets/winner.mp3");
k.loadSound("dogdetail", "/assets/dogdetail.mp3");

export default k;
