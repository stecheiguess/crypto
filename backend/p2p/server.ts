import WebSocket, { WebSocketServer } from "ws";
import axios from "axios";
import express, { Request, Response } from "express";
import { createHash } from "crypto";

const MAX_CACHE_SIZE = 5000; // 🔥 Limit to 5000 messages

class P2PServer {
    private P2P_PORT: number;
    private peers: string[];
    private sockets: WebSocket[] = [];
    private server: WebSocketServer;
    private app: express.Express;
    private seen: Set<string>;

    constructor() {
        this.P2P_PORT = parseInt(process.env.P2P_PORT || "5001", 10);
        this.peers = process.env.PEERS ? process.env.PEERS.split(",") : [];

        this.server = new WebSocketServer({ port: this.P2P_PORT });
        this.app = express();
        this.app.use(express.json());
        this.seen = new Set<string>();

        this.initializeServer();
        this.connectToPeers();
        this.startHTTPServer();
    }

    /** 🔹 Initialize WebSocket Server */
    private initializeServer(): void {
        this.server.on("connection", async (socket: WebSocket) => {
            console.log("New peer connected");
            await this.connectSocket(socket);
        });

        console.log(`P2P Server listening on port ${this.P2P_PORT}`);
    }

    /** 🔹 Connect to existing peers */
    private connectToPeers(): void {
        this.peers.forEach((peer) => {
            const socket = new WebSocket(peer);
            socket.on("open", () => this.connectSocket(socket));
            socket.on("error", (err) =>
                console.error(`Error connecting to peer: ${peer}`, err)
            );
        });
    }

    /** 🔹 Handle new WebSocket connection */
    private async connectSocket(socket: WebSocket): Promise<void> {
        this.sockets.push(socket);
        console.log("Socket connected");

        this.messageHandler(socket);

        try {
            const response = await axios.get(
                `http://127.0.0.1:${this.P2P_PORT - 2000}/api/chain`
            );
            socket.send(JSON.stringify(response.data));
        } catch (error) {
            console.error("Error fetching blockchain data:", error);
        }
    }

    /** 🔹 Handle incoming messages */
    private messageHandler(socket: WebSocket): void {
        socket.on("message", async (message: string) => {
            try {
                const data: WebSocketMessage = JSON.parse(message);
                console.log("Received data:", data);
                const hash = createHash("sha256").update(message).digest("hex");

                if (this.seen.has(hash)) {
                    return;
                }

                if (this.seen.size > MAX_CACHE_SIZE) {
                    this.seen.clear();
                    console.log("Cleared seen messages cache");
                }

                this.seen.add(hash);

                switch (data.type) {
                    case "CHAIN":
                        await this.replace(data.data);
                    case "TRANSACTION":
                }

                this.broadcast(message);
            } catch (error) {
                console.error("Invalid message received:", error);
            }
        });
    }

    /** 🔹 Broadcast new blockchain data */
    private async broadcast(data: string): Promise<void> {
        for (const socket of this.sockets) {
            if (socket.readyState === WebSocket.OPEN) {
                socket.send(data);
            }
        }
    }

    /** 🔹 Replace blockchain data via API */
    private async replace(data: Block[]): Promise<void> {
        try {
            await axios.post(
                `http://127.0.0.1:${this.P2P_PORT - 2000}/api/replace`,
                data
            );
        } catch (error) {
            console.error("Replace error:", error);
        }
    }

    /** 🔹 Start Express HTTP API */
    private startHTTPServer(): void {
        this.app.post("/broadcast", async (req: Request, res: Response) => {
            console.log("Received broadcast request from API, syncing...");
            await this.broadcast(JSON.stringify(req.body));
            res.json({ message: "Blockchain broadcasted to peers" });
        });

        this.app.listen(this.P2P_PORT + 1000, () => {
            console.log(
                `P2P Control API listening on port ${this.P2P_PORT + 1000}`
            );
        });
    }
}

// 🔥 Start the P2P Server
new P2PServer();
