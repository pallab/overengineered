'use client';

import { Context, createContext, useContext, useEffect } from "react";
import { useState } from "react";


export const GridContext = createContext({
  counts: {}
});


export const GridContextProvider = ({ children }) => {

  const [counts, setCounts] = useState([]);
  
  useEffect(() => {
    let conn = new WebSocket(
      "ws://localhost:8080/ws"
      );
    
      conn.onopen = (e) => {
        console.log(`connecting websocket`)
        conn.send( "\\print Hello World!")
    };
    
    conn.onmessage = (e) => {
        let data = JSON.parse(e.data);
        setCounts(data.counts)
    }    

    console.log(`Effect : GridContext`)
  }, []);


  return (
    <GridContext.Provider value={{ counts }}>
      {children}
    </GridContext.Provider>
  );
};

export const useGridContext = () => {
  const { counts } = useContext(GridContext)
  return {
    counts
  }
};