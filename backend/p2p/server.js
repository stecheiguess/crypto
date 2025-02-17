const WebSocket = require("ws");
const axios = require("axios");
const express = require("express");


const P2P_PORT = parseInt(process.env.P2P_PORT) || 5001;
const peers = process.env.PEERS ? process.env.PEERS.split(",") : [];

const server = new WebSocket.Server({ port: P2P_PORT });
let sockets = [];

// Start Express server for P2P control
const app = express();
app.use(express.json());

// Endpoint to receive broadcast request from Rust API
app.post("/broadcast", async (req, res) => {
    console.log("Received broadcast request from API, syncing...");
    //console.log(req.body);
    await sync(JSON.stringify(req.body));
    res.json({ message: "Blockchain broadcasted to peers" });
});

app.listen(P2P_PORT + 1000, () => {
    console.log(`P2P Control API listening on port ${P2P_PORT + 1000}`);
});

server.on("connection", async (socket) => {
    
    await connectSocket(socket);


});

connectToPeers();

async function connectSocket(socket) {
    sockets.push(socket);
    console.log("Socket connected");

    await messageHandler(socket)

    //await send(socket)
    
    
}

function connectToPeers() {
    peers.forEach(peer => {
        const socket = new WebSocket(peer);
        socket.on("open", () => connectSocket(socket));
    });
}

async function messageHandler(socket) {
    socket.on('message', async message => {
        const data = JSON.parse(message);
        console.log(data)
        await replace(data)
    })
}

async function sync(data) {
    sockets.forEach(async socket => {
        await socket.send(data)
    });
}


async function replace(data) {
    try {
        await axios.post(`http://127.0.0.1:${P2P_PORT-2000}/api/replace`, data);


    } catch (error) {
        console.error("Replace error:", error);
    }
}


console.log(`P2P Listening on ${P2P_PORT}`);

/*async function mineNewBlock() {
    try {
        const response = await axios.get("http://127.0.0.1:3000");
        const newBlock = response.data;
        console.log("New block mined:", newBlock);

        broadcast({ type: "NEW_BLOCK", block: newBlock });
    } catch (error) {
        console.error("Mining error:", error);
    }
}*/
//*connectToPeers();

