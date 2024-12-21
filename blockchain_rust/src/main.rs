use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::fs;
use std::sync::{Arc, Mutex};

// Define a transaction structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Transaction {
    sender: String,
    #[serde(rename = "receiver")]
    reciever: String,
    amount: u64,
}

// Define a block structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    timestamp: i64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: "".to_string(),
        };
        block.hash = block.calculate_hash(); // Calculate initial hash
        block
    }

    // Calculate the hash of the block
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{:?}{:?}{}{}",
            self.timestamp, self.transactions, self.previous_hash, self.nonce
        ));
        format!("{:x}", hasher.finalize()) // Use lowercase hex
    }

    // Mine the block (find a valid hash)
    fn mine_block(&mut self, difficulty: usize) {
        while &self.hash[0..difficulty] != "0".repeat(difficulty) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

// Define the blockchain structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(vec![], String::from("0")); // Genesis block
        Blockchain {
            chain: vec![genesis_block],
            difficulty: 2, // Mining difficulty
        }
    }

    fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(transactions, previous_hash);
        new_block.mine_block(self.difficulty); // Mine the block
        self.chain.push(new_block); // Add to chain
    }

    fn is_chain_valid(&self) -> bool {
        for (i, block) in self.chain.iter().enumerate() {
            if i == 0 {
                continue; // Skip genesis block
            }
            let previous_block = &self.chain[i - 1];
            if block.hash != block.calculate_hash() {
                return false; // Invalid hash
            }
            if block.previous_hash != previous_block.hash {
                return false; // Invalid chain link
            }
        }
        true
    }

    // Save blockchain to a file
    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(filename, data)?;
        Ok(())
    }

    // Load blockchain from a file
    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let data = fs::read_to_string(filename)?;
        let blockchain: Blockchain = serde_json::from_str(&data)?;
        Ok(blockchain)
    }
}

// Shared application state
struct AppState {
    blockchain: Mutex<Blockchain>,
}

// Route to get the blockchain
async fn get_chain(data: web::Data<Arc<AppState>>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap().clone();
    HttpResponse::Ok().json(&blockchain)
}

// Route to add a block
async fn add_block(
    data: web::Data<Arc<AppState>>,
    transactions: web::Json<Vec<Transaction>>,
) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_block(transactions.into_inner());
    HttpResponse::Ok().json(serde_json::json!({"message": "Block Added Successfully"}))
}

// Main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let filename = "blockchain.json";

    // Load or create blockchain
    let blockchain = if let Ok(blockchain) = Blockchain::load_from_file(filename) {
        blockchain
    } else {
        Blockchain::new()
    };

    let app_state = Arc::new(AppState {
        blockchain: Mutex::new(blockchain),
    });

    // Clone the Arc for use in the closure
    let app_state_clone = Arc::clone(&app_state);

    println!("Starting blockchain application on port 8080");

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&app_state))) // Clone here
            .route("/chain", web::get().to(get_chain))
            .route("/add_block", web::post().to(add_block))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // Use the cloned app_state to save the blockchain
    let blockchain = app_state_clone.blockchain.lock().unwrap();
    if blockchain.is_chain_valid() {
        println!("Blockchain is valid");
    } else {
        println!("Blockchain is invalid");
    }
    blockchain.save_to_file(filename)?;

    Ok(())
}
