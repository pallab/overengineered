// A context that holds & change the value of paintColor

import { Context, createContext, useContext, useEffect } from "react";
import { useState } from "react";


export const GridContext = createContext({
  tiles: {},
  setTiles : () => {}
});


export const GridContextProvider = ({ children }) => {

  const [tiles, setTiles] = useState(null);
  
  useEffect(() => {
    let conn = new WebSocket(
      "ws://localhost:8080/ws"
      );
    
      conn.onopen = (e) => {
        console.log(`connecting websocket`)
        conn.send(JSON.stringify( { word : "Hello World!"}))
    };
    
    conn.onmessage = (e) => {
        let data = JSON.parse(e.data);
        
        let tiles = [];

        for (var p in data) {
            tiles.push(data[p])
        }

        console.log(`received via ws :  ${typeof(tiles)} - ${JSON.stringify(tiles).substring(0, 10)}`);
        setTiles(tiles)
    }    

    console.log(`Effect : GridContext`)
  }, []);


  return (
    <GridContext.Provider value={{ tiles, setTiles }}>
      {children}
    </GridContext.Provider>
  );
};

export const useGridContext = () => {
  const { tiles, setTiles } = useContext(GridContext)
  return {
    tiles, setTiles
  }
};