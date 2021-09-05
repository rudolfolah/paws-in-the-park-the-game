// @ts-ignore
import React from "react";
import { Dog } from "../types";

interface DogInfoProps {
  dog: Dog;
}

export function DogInfo({dog}: DogInfoProps) {
  return (<div>
    <div>
      <div>
        <img/>
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
      <div>{dog.attr1}</div>
      <div>{dog.attr2}</div>
      <div>{dog.attr3}</div>
      <div>{dog.attr4}</div>
    </div>
  </div>)
}