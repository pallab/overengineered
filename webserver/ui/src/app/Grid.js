'use client';
import React, { useState, useRef, useEffect} from "react";
import Tile from "./Tile";
import { useGridContext } from "./GridContext";

//  a grid component that fills the screen with Pixel components

const Grid = () => {
  const ref = useRef(null);

  const [letters, setLetters] = useState([]);

  const {tiles, setTiles} = useGridContext();

  useEffect(() => {

    const grid = [];

    for (let i in tiles) {  
          let tile = tiles[i];
          grid.push(<Tile key={i} id={i} tile={tile} />);
    }

    setLetters(grid);
  }, [tiles]);

  return (
    <div ref={ref} className="grid">
        <div className="line">
            {letters.slice(0,5)}
        </div>
        <div className="line">
            {letters.slice(6,)}
        </div>
        
    </div>
  );
};

export default Grid;
