import k from "./engine";

import { CONTAINER_CSS_SELECTOR } from "./constants";
import detail from "./scenes/detail";
import slots from "./scenes/slots";

type Scene = "detail" | "slots";

k.scene("empty", () => {});
k.scene("detail", detail);
k.scene("slots", slots);

export function hideGame() {
  const element: any = document.querySelector(CONTAINER_CSS_SELECTOR);
  if (element) {
    element.style.visibility = "hidden";
    k.go("empty");
  }
}

export function showGame(scene: Scene) {
  const element: any = document.querySelector(CONTAINER_CSS_SELECTOR);
  if (element) {
    element.style.visibility = "visible";
    k.go(scene);
  }
}

export function setGameData(key: string, data: any) {
  k.setData(key, data);
}

export function getGameData(key: string): any {
  return k.getData(key);
}
