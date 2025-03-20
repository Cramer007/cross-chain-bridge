use ethers::prelude::*;
use ethers::contract::Contract;
use dotenv::dotenv;
use std::env;
use std::fs;
use std::sync::Arc;
use ethers::abi::Abi;
use serde_json;
use futures::stream::StreamExt; //  Assure-toi que futures est bien installé !

// utiliser alloy ou web3 pas ethers car elle est deprecated 

pub async fn listen_to_deposits() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    println!("🔍 Indexeur démarré, écoute des événements Deposit...");

    let rpc_url = env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL must be set");
    let provider = Arc::new(Provider::<Http>::try_from(rpc_url)?);

    let contract_address: Address = env::var("SEPOLIA_BRIDGE_ADDRESS")
        .expect("SEPOLIA_BRIDGE_ADDRESS must be set")
        .parse()?;
    println!("🎯 L’indexeur écoute le contrat à cette adresse : {:?}", contract_address);

    // Charger correctement l'ABI
    let abi: Abi = serde_json::from_str(
        &fs::read_to_string("/Users/portablegaucher/Desktop/dev/blockchain programming /cross-chain-bridge/indexer/bridge_abi.json").expect("Erreur de lecture de l'ABI")
    ).expect("Erreur de parsing de l'ABI");

    //abi c'est pas grave mais c mieux de la hardcoder 



    let client = Arc::new(provider.clone());
    let contract = Contract::new(contract_address, abi, client);

    // ✅ DEBUG : Vérifier si des logs bruts existent
    println!("🔍 Tentative de récupération des logs bruts...");

    let logs = provider.get_logs(&Filter::new()
        .address(contract_address)
        .from_block(BlockNumber::Latest))
        .await?;
    
    println!("📩 Logs bruts reçus : {:?}", logs);
    
    //ici pour les block number c'est mieux d'avoir un range 

    //match web3 

    #[derive(Debug, Clone, EthEvent)]
    struct DepositEvent {
        #[ethevent(name = "token", indexed)]
        token: Address,
        #[ethevent(name = "from", indexed)]
        from: Address,
        #[ethevent(name = "to", indexed)]
        to: Address,
        #[ethevent(name = "amount", indexed)]
        amount: U256,
        #[ethevent(name = "nonce", indexed)]
        nonce: U256,
    }

    //  CORRECTION : stocke l'événement AVANT `.stream()`
    let event_filter = contract
        .event::<DepositEvent>()
        .from_block(BlockNumber::Latest);

    let event_stream = event_filter.stream().await?;

    let mut event_stream = Box::pin(event_stream);

    println!("🔍 En attente d'événements...");

    while let Some(event) = event_stream.next().await {
        println!("📩 Événement capté brut : {:?}", event);
        match event {
            Ok(deposit) => {
                println!(
                    " Dépôt détecté : Token: {}, From: {}, To: {}, Amount: {}, Nonce: {}",
                    deposit.token, deposit.from, deposit.to, deposit.amount, deposit.nonce
                );
            }
            Err(e) => {
                eprintln!(" Erreur lors de la récupération de l'événement : {:?}", e);
            }
        }
    }

    Ok(())
}
