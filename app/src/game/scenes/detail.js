import k from "../engine";
import { GAME_TITLE, WIDTH, HEIGHT } from "../constants";

const detail = () => {
  k.add([
    k.text(GAME_TITLE, 32),
    k.origin("center"),
    k.pos(WIDTH / 2, HEIGHT / 6),
    "title",
  ]);
  const dog = k.getData("dog");
  k.addText(dog.name, {
    origin: "center",
    pos: k.vec2(WIDTH / 2, HEIGHT / 6)
  });
};

export default detail;
