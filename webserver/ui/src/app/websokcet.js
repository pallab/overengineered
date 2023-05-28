const { useEffect } = require("react");
const { useGridContext } = require("./GridContext");

// const { setPixels } = useGridContext()

export const connectSocket = () => {

    // useEffect( () => {
        
        let conn = new WebSocket(
        "ws://localhost:8080/ws/data"
        );

        conn.onopen = (e) => {
            console.log(`connecting websocket`)
            conn.send(JSON.stringify( { word : "Hello World!"}))
        };

        conn.onmessage = (e) => {
            let data = JSON.parse(e.data);
            console.log(`received via ws : $(data}`);
            // setPixels(['1:1'])
        }

    // })
}