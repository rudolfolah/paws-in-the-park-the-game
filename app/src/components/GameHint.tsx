import React from "react";
import "./GameHint.css";

interface GameHintProps {
  children: any;
  align: "left" | "right";
}

const DOG_IMAGE = "/assets/dog05.png";

export default function GameHint(props: GameHintProps) {
  return (
    <div className={`game-hint game-hint-${props.align}`}>
      <div className="game-hint-text">
        {props.children}
      </div>
      <div className="game-hint-image">
        <img src={DOG_IMAGE} alt={"dog"} />
      </div>
    </div>
  );
}
