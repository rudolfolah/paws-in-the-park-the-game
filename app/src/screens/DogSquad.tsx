import React, {useEffect} from "react";
import {hideGame} from "../game";

const dogs: any = {
  "id-01": {
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
  }
}

// @ts-ignore
function DogInfo({ dog }) {
  return (<div>
    <div>
      <div>
        <img />
      </div>
      <div>
        <div>
          {dog.name}
        </div>
        <div>
          {dog.biography}
        </div>
      </div>
    </div>
    <div>
      {Object.keys(dog.attributes).map(attributeName => (<div key={attributeName}>
        {attributeName}: {dog.attributes[attributeName]}
      </div>))}
    </div>
  </div>)
}

export default function DogSquad() {
  useEffect(() => { hideGame() }, []);
  return (<div>
    {Object.keys(dogs).map(dogId => <DogInfo key={dogId} dog={dogs[dogId]} />)}
  </div>);
}
