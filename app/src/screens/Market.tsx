import React, {useEffect} from "react";
import {hideGame} from "../game";

export default function Market() {
  useEffect(() => { hideGame(); }, []);
  return (<></>);
}
