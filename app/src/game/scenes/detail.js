import k from "../engine";
import {GAME_TITLE, WIDTH, HEIGHT, ACCESSORY_SPRITE, DOG_SPRITE} from "../constants";

const detail = () => {
  k.add([
    k.text(GAME_TITLE, 32),
    k.origin("center"),
    k.pos(WIDTH / 2, HEIGHT / 10),
    "title",
  ]);
  const dog = k.getData("dog");
  k.add([
    k.text(dog.name),
    k.origin("center"),
    k.pos(WIDTH / 2, HEIGHT / 6),
  ]);
  k.add([
    k.sprite(DOG_SPRITE[dog.class]),
    k.origin("center"),
    k.pos(WIDTH / 2, HEIGHT / 2),
  ]);
  const accessories = k.getData("accessories");
  for (let accessory of accessories) {
    k.add([
      k.sprite(ACCESSORY_SPRITE[accessory.name]),
      k.origin("center"),
      k.scale(Math.random() * 0.7 + 0.1),
      k.pos(
        Math.floor(Math.random() * (WIDTH * 0.9) + (WIDTH * 0.1)),
        Math.floor(Math.random() * (HEIGHT * 0.7) + (HEIGHT * 0.2)),
      ),
      accessory.name,
    ]);
  }
};

export default detail;
