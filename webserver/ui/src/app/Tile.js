'use client';
import React, { useState, useRef, useEffect } from "react";
import Pixel from "./Pixel";

const Tile = ({ id, tile }) => {
 // const ref = useRef(null);

  const [pixels, setPixels] = useState([]);

  useEffect(() => {

    const grid = [];

    for (let p of tile.pixels) {
      grid.push(<Pixel key={`${p.x}:${p.y}`} x={p.x} y={p.y} color={p.fg} />)
    }

     setPixels(grid);
  }, [tile]);

  return (
    <div className="tile">
      {pixels}
    </div>
  );
};

export default Tile;
