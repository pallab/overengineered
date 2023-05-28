'use client';
import React, { useState, useRef, useEffect} from "react";
import Tile from "./Tile";
import { useGridContext } from "./GridContext";
import { stringify } from "postcss";

//  a grid component that fills the screen with Pixel components

const Grid = () => {
  const ref = useRef(null);

  const [letters, setLetters] = useState([]);

  const {tiles, setTiles} = useGridContext();

  useEffect(() => {

    const grid = [];

    for (let i in tiles) {  
        if(i < 23) {
            let tile = tiles[i];

            console.log(`This tile index : ${i} value : ${JSON.stringify(tile).substring(0, 10)}`);
    
            grid.push(<Tile key={i} id={i} tile={tile} />);
        }
    }

    console.log(`Effect : Grid : ${grid.length} : ${typeof(tiles)}`)
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
