import React, {useEffect} from "react";
import { useParams } from "react-router-dom";
import {setGameData, showGame} from "../game";

const dog = {
  "name": "Dog #01",
  "bio": "Biography",
  "attributes": {
    "floofiness": 10,
    "curiousity": 1,
    "agility": 5,
    "squirrel_factor": 7,
  },
  "trophies": [
    {
      "trophyType": "ribbon",
      "name": "Trophy #01",
    },
    {
      "trophyType": "ribbon",
      "name": "Trophy #02",
    }
  ]
};

export default function DogDetail() {
  let { dogId } = useParams<{ dogId: string }>();
  useEffect(() => {
    setGameData("dog", dog);
    showGame("detail");
  }, []);
  return (<></>);
}
