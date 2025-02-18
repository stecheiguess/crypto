type WebSocketMessage =
    | { type: "TRANSACTION"; data: Transaction }
    | { type: "CHAIN"; data: Block[] };

// 🔹 Defines an Output transaction
interface TransactionOutput {
    address: string; // Public key of recipient
    amount: number; // Amount sent
}

// 🔹 Defines the Input transaction
interface TransactionInput {
    address: string; // Public key of sender
    amount: number; // Total input amount
    signature: string; // Digital signature
    timestamp: number; // Unix timestamp
}

// 🔹 Defines the full Transaction structure
interface Transaction {
    id: string; // Unique transaction ID
    input: TransactionInput; // Transaction input details
    outputs: TransactionOutput[]; // List of transaction outputs
}

// 🔹 Defines a single Block in the blockchain
interface Block {
    data: string; // Block data (transactions, messages, etc.)
    difficulty: number; // Proof-of-work difficulty level
    index: number; // Position in the blockchain
    nonce: number; // Nonce used for mining
    prev: string; // Previous block hash
    timestamp: number; // Unix timestamp when block was mined
}
