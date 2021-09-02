import React, {useEffect} from "react";
import { useParams } from "react-router-dom";
import {setGameData, showGame} from "../game";

export default function Slots() {
  useEffect(() => {
    showGame("slots");
  }, []);
  return (<></>);
}
