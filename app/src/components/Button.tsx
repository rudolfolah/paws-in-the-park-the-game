import React from "react";

import "./Button.css";

interface ButtonProps {
  children: any;
  onClick: any;
}

export default function Button(props: ButtonProps) {
  return (
    <div className="btn-container">
      <button className="btn-button" onClick={(e) => props.onClick(e)}>
        {props.children}
      </button>
    </div>
  );
}
