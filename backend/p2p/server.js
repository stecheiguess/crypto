const WebSocket = require("ws");

const P2P_PORT = process.env.P2P_PORT || 5001;
const peers = process.env.PEERS ? process.env.PEERS.split(",") : [];

const server = new WebSocket.Server({ port: P2P_PORT });
let sockets = [];

server.on("connection", (socket) => {
    
    connectSocket(socket);

});


connectToPeers();

function connectSocket(socket) {
    sockets.push(socket);
    console.log("Socket connected");
}

function connectToPeers() {
    peers.forEach(peer => {
        const socket = new WebSocket(peer);
        socket.on("open", () => connectSocket(socket));
    });
}

function broadcast(message) {
    sockets.forEach(socket => socket.send(JSON.stringify(message)));
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

