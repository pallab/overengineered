'use client';
import React, { useState, useRef, useEffect } from "react";
import Pixel from "./Pixel";

const Tile = ({ id, pixelDefs }) => {

  const [pixels, setPixels] = useState([]);

  useEffect(() => {
    const grid = [];

    for (let p of pixelDefs) {
      grid.push(<Pixel key={`${p.x}:${p.y}`} x={p.x} y={p.y} color={p.fg} />)
    }

     setPixels(grid);
  }, [pixelDefs]);

  return (
    <div className="tile">
      {pixels}
    </div>
  );
};

export default Tile;
