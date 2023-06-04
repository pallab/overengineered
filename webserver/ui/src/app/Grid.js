'use client';
import React, { useState, useRef, useEffect} from "react";
import Tile from "./Tile";
import { useGridContext } from "./GridContext";
import { getFilledPixels }  from "./canvas";

const Grid = () => {
  const ref = useRef(null);

  const [tiles, setTiles] = useState([]);

  const {counts } = useGridContext();

  useEffect(() => {

    const grid = [];
    const line = "Hello World!".toUpperCase();

    const cobj = {};

    counts.forEach((c) => {
      cobj[c.c] = c.count;
    });

    Array.from(line).forEach((c, i) => {
      let count = cobj[c];
      let fill_fraction = count * 1.0 / 80;

      let pixels = getFilledPixels(c, fill_fraction);
      grid.push(<Tile key={i} id={i} pixelDefs={pixels} />);
    });

    setTiles(grid);
  }, [counts]);

  return (
    <div ref={ref} className="grid">
        <div className="line">
            {tiles.slice(0,5)}
        </div>
        <div className="line">
            {tiles.slice(6,)}
        </div>
        
    </div>
  );
};

export default Grid;
