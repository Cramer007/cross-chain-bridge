use std::env;
use dotenv::dotenv;
use tokio::time::{sleep, Duration};
mod indexeur;
use indexeur::listen_to_deposits;
mod database;
use database::{establish_connection, update_deposit_as_processed};
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rpc_url = env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL must be set");
    println!("✅ Rust fonctionne ! RPC URL: {}", rpc_url);

    println!("🔍 Indexeur en cours d'exécution...");
    
    // Exécuter l'indexeur en tâche de fond
    tokio::spawn(async {
        listen_to_deposits().await.expect("Erreur lors de l'écoute des dépôts");
    });

    // Simulation d'un traitement des dépôts
    let mut conn = establish_connection();

    loop {
        // Exemple : Marquer un dépôt avec nonce=1 comme "processed"
        update_deposit_as_processed(&mut conn, 1);
        
        // Attendre un certain temps avant de re-vérifier
        sleep(Duration::from_secs(10)).await;
    }
}

//etape pour valider un deposit 
//faire un token add 
//ensuite approuve manually avecv la commande qui suit
//cast send 0x88bd0f559465aE1029fdaBE3f45E86abfc93E40A "approve(address,uint256)" 0xE2e4eC7863Ee9828D3A1F83EDa42839aCaE61DCe 1000000000000000000 --rpc-url https://eth-sepolia.g.alchemy.com/v2/AwXNQB1sZceWq5qoYpeR7U0jsYYvoBHv --private-key 0x03771d4edf1ec9eeb581e8b2b3b6cc3b7c259473213165bf3dea7bef0ae951a1
//faire un deposit 
//fais un distribute sans oublier de faire un icrement + 1 sur le nonce
// pour les infos a mettr e
//token adress : 0x88bd0f559465ae1029fdabe3f45e86abfc93e40a
//ammount: 1000000000000000000
//recipient address: 0x750e319c15edad2c5a8260c0bf70314a178fc40b

//aller sur etherscan sepolia tesnet : 0xE2e4eC7863Ee9828D3A1F83EDa42839aCaE61DCe
//dans event logs en refreshant la page tu auras toutes les transactions passés

//cargo clean , puis cargo build , puis cargo run , pour la bdd sqlite3 indexer.db 
//bdd : SELECT * FROM deposits; SELECT * FROM distributions; ".tables" pour voir les tables

//je me suis arreter a peut etre stopper les doublons ( a nouveau) dans la table distributions , mais je peut aussi faire entre deux token differents . mais j'ai reussis mon premeir bridge! 

// au cas ou au debut le forge contract demarrer :  forge create --rpc-url https://eth-sepolia.g.alchemy.com/v2/AwXNQB1sZceWq5qoYpeR7U0jsYYvoBHv --private-key 0x03771d4edf1ec9eeb581e8b2b3b6cc3b7c259473213165bf3dea7bef0ae951a1 contracts/src/TestToken.sol:TestToken --broadcast