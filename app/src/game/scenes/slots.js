import k from "../engine";
import {WIDTH, HEIGHT, SLOTS_TITLE} from "../constants";
import {shuffle} from "../utils";

const SLOT_REEL_SCALE = 0.7;
const SLOT_REEL_TOP = HEIGHT * 0.1;

const SLOT_REEL_ITEM_SPRITE_IDS = ["dog01", "dog02", "dog03", "dog04", "dog05"];
const SLOT_REEL_ITEM_TOP = SLOT_REEL_ITEM_SPRITE_IDS.length * -100;
const SLOT_REEL_ITEM_LEFT = 10;
const SLOT_REEL_ITEM_HORIZONTAL_GUTTER = 300;
const SLOT_REEL_ITEM_VERTICAL_GUTTER = 24;
const SLOT_REEL_ITEM_OFFSET = 330 * SLOT_REEL_SCALE;
const SLOT_REEL_ITEM_SPEEDS = [350, 400, 500];
// time in milliseconds
const SLOT_REEL_ITEM_SPIN_DELAYS = [0, 50, 100, 200, 300, 400];

const SLOT_BAR_HEIGHT = 128;
const SLOT_BAR_HEADER_TOP = 0;
const SLOT_BAR_FOOTER_TOP = HEIGHT - SLOT_BAR_HEIGHT;

const randomSpeed = () => (SLOT_REEL_ITEM_SPEEDS[Math.floor(Math.random() * SLOT_REEL_ITEM_SPEEDS.length)]);
// time in seconds
const randomDelay = () => (
  SLOT_REEL_ITEM_SPIN_DELAYS[Math.floor((Math.random() * SLOT_REEL_ITEM_SPIN_DELAYS.length))] / 1000.0
);

const makeReel = (tag, left) => {
  const spriteIds = shuffle(SLOT_REEL_ITEM_SPRITE_IDS);
  k.add([
    k.sprite("reel"),
    k.pos(left, SLOT_REEL_TOP),
    k.scale(k.vec2(SLOT_REEL_SCALE, 2.3)),
    "reel-background",
  ]);
  return spriteIds.map((spriteId, index) => {
    return k.add([
      k.sprite(spriteId),
      k.pos(left, SLOT_REEL_ITEM_TOP + (SLOT_REEL_ITEM_OFFSET * index) + (index === 0 ? 0 : SLOT_REEL_ITEM_VERTICAL_GUTTER * index)),
      k.scale(SLOT_REEL_SCALE),
      tag
    ]);
  });
}

const moveReelItem = () => {
  const speed = randomSpeed();
  return (reelItem) => {
    if (reelItem.pos.y > SLOT_BAR_FOOTER_TOP) {
      reelItem.pos.y = SLOT_REEL_ITEM_TOP;
    }
    reelItem.move(0, speed);
  };
};

const makeWinnerFrames = () => {
  k.add([
    k.sprite("frame"),
    k.pos(SLOT_REEL_ITEM_LEFT, WIDTH * 0.4),
    k.scale(0.65),
    k.layer("winner")
  ]);
  k.add([
    k.sprite("frame"),
    k.pos(SLOT_REEL_ITEM_LEFT + SLOT_REEL_ITEM_HORIZONTAL_GUTTER, WIDTH * 0.4),
    k.scale(0.65),
    k.layer("winner")
  ]);
}

const spin = (finishedSpinCallback) => {
  let reelCancels = [];
  let reelDelays = [
    randomDelay(),
    randomDelay(),
    randomDelay(),
  ];
  k.wait(reelDelays[0], () => {
    reelCancels.push(k.action("column-1", moveReelItem()));
  });
  k.wait(reelDelays[1], () => {
    reelCancels.push(k.action("column-2", moveReelItem()));
  });
  k.wait(reelDelays[2], () => {
    reelCancels.push(k.action("column-3", moveReelItem()));
  });
  const musicSpin = k.play("spin", {
    loop: true,
  });
  k.wait(3, () => {
    document._pawsinthepark_execute_spin(function() {
      musicSpin.volume(1.5);
      k.wait(1, () => {
        for (let i = 0; i < reelDelays.length; i += 1) {
          reelCancels[i]();
        }
        musicSpin.stop();
        const musicWinner = k.play("winner", { loop: true });
        k.wait(10, () => {
          musicWinner.stop();
          finishedSpinCallback();
        });
      });
    });
  });
}

const slots = () => {
  let spinInProgress = false;
  k.layers([
    "background",
    "obj",
    "winner",
    "ui"
  ], "obj");
  makeWinnerFrames();
  const reelItems = [
    makeReel("column-1", SLOT_REEL_ITEM_LEFT),
    makeReel("column-2", SLOT_REEL_ITEM_LEFT + SLOT_REEL_ITEM_HORIZONTAL_GUTTER),
    makeReel("column-3",SLOT_REEL_ITEM_LEFT + (SLOT_REEL_ITEM_HORIZONTAL_GUTTER * 2))
  ];

  k.action(() => {
    if (!spinInProgress && k.mouseIsClicked()) {
      spinInProgress = true;
      spin(() => {
        spinInProgress = false;
      });
    }
  });

  k.add([
    k.rect(WIDTH, SLOT_BAR_HEIGHT),
    k.pos(0, SLOT_BAR_HEADER_TOP),
    k.color(0.1, 0.1, 0.2),
    k.layer("ui")
  ]);

  k.add([
    k.rect(WIDTH, SLOT_BAR_HEIGHT),
    k.pos(0, SLOT_BAR_FOOTER_TOP),
    k.color(0.1, 0.1, 0.2),
    k.layer("ui")
  ]);

  k.add([
    k.text(SLOTS_TITLE, 32),
    k.origin("center"),
    k.pos(WIDTH / 2, 32),
    "title",
    k.layer("ui")
  ]);
};

export default slots;
