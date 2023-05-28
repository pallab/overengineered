'use client';
import React, { useState, useRef, useEffect } from "react";
import Pixel from "./Pixel";
import { stringify } from "postcss";


const Tile = ({ id, tile }) => {
  const ref = useRef(null);

  const [pixels, setPixels] = useState([]);

  useEffect(() => {

    const grid = [];

    for (let p of tile.pixels) {
      grid.push(<Pixel key={`${p.x}:${p.y}`} x={p.x} y={p.y} color={p.fg} />)
    }

    console.log(`Effect : Tile : ${id} : ${JSON.stringify(tile).substring(0, 10)}`)
    setPixels(grid);
  }, [tile]);

  return (
    <div ref={ref} className="tile">
      {pixels}
    </div>
  );
};

export default Tile;
