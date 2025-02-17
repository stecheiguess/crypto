const WebSocket = require("ws");

const ws = new WebSocket("ws://127.0.0.1:5001");

ws.on("open", () => {
    console.log("Connected to P2P server");
    ws.send("Hello from client!");
});

ws.on("message", (data) => {
    console.log("Received:", data.toString());
});


ws.on("close", () => {
    console.log("Disconnected from server");
});