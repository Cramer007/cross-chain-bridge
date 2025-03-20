use web3::types::{Address, FilterBuilder, Log, H160, H256, U256};
use web3::transports::Http;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use crate::database::{establish_connection, insert_deposit, insert_distribution}; // Ajout de insert_deposit

/// Fonction principale qui écoute les événements `Deposit` et `Distribution`
pub async fn listen_to_deposits() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let rpc_url = env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL must be set");
    let contract_address: Address = env::var("SEPOLIA_BRIDGE_ADDRESS")
        .expect("SEPOLIA_BRIDGE_ADDRESS must be set")
        .parse()?;

    println!("🎯 L’indexeur écoute le contrat à cette adresse : {:?}", contract_address);

    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);

    let deposit_event_signature: H256 = H256::from_slice(
        &web3::signing::keccak256(b"Deposit(address,address,address,uint256,uint256)")
    );

    let distribution_event_signature: H256 = H256::from_slice(
        &web3::signing::keccak256(b"Distribution(address,address,uint256,uint256)")
    );

    println!("🔹 Signature Deposit: {:?}", deposit_event_signature);
    println!("🔹 Signature Distribution: {:?}", distribution_event_signature);

    let deposit_event_signature: H256 = H256::from_slice(
        &web3::signing::keccak256(b"Deposit(address,address,address,uint256,uint256)")
    );

    let distribution_event_signature: H256 = H256::from_slice(
        &web3::signing::keccak256(b"Distribution(address,address,uint256,uint256)")
    );

    let latest_block = web3.eth().block_number().await?;
    let from_block = latest_block.as_u64().saturating_sub(1000);
    let from_block = U256::from(from_block);

    println!(
        "🔍 Récupération des logs de dépôt et distribution de {} à {}...",
        from_block, latest_block
    );

    let filter = FilterBuilder::default()
        .address(vec![contract_address])
        .from_block(web3::types::BlockNumber::Number(from_block.as_u64().into()))
        .to_block(latest_block.into())
        
        .build();

    let logs: Vec<Log> = web3.eth().logs(filter).await?;
    println!("📡 Logs récupérés : {:?}", logs);

    if logs.is_empty() {
        println!("⚠️ Aucun événement trouvé.");
    } else {
    let mut conn = establish_connection();
    // println!("🛠️ Test d'insertion distribution manuelle...");
    // insert_distribution(
    //     &mut conn,
    //     H160::from_low_u64_be(1),
    //     H160::from_low_u64_be(2),
    //     H160::from_low_u64_be(3),
    //     U256::from(1000),
    //     U256::from(99999)
    // );

    println!("✅ Test insert_distribution terminé !");



    
        for log in logs {
            println!("🔍 Log brut reçu : {:?}", log);
            println!("🔹 Signature trouvée : {:?}", log.topics.get(0));
            println!("🔹 Tous les topics du log: {:?}", log.topics);
        
            println!("📩 Événement détecté !");
            println!("Block: {:?}", log.block_number.unwrap_or_default());
            println!("Tx Hash: {:?}", log.transaction_hash.unwrap_or_default());
        
            if let Some(first_topic) = log.topics.first() {
                if *first_topic == deposit_event_signature {
                    println!("🔹 C'est un dépôt !");
                    if log.topics.len() >= 4 {
                        let token: H160 = Address::from_slice(&log.topics[1].0[12..]);
                        let from: H160 = Address::from_slice(&log.topics[2].0[12..]);
                        let to: H160 = Address::from_slice(&log.topics[3].0[12..]);
                        let amount: U256 = U256::from_big_endian(&log.data.0[0..32]);
                        let nonce: U256 = U256::from_big_endian(&log.data.0[32..64]);
        
                        println!("📩 Dépôt détecté ! Insertion en cours...");
                        insert_deposit(&mut conn, token, from, to, amount, nonce);
                        println!("✅ Après insert_deposit !");
                    } else {
                        println!("❌ Erreur: Topics insuffisants pour un dépôt !");
                    }
                } else if *first_topic == distribution_event_signature {
                    println!("🔹 C'est une distribution !");
                    if log.topics.len() >= 3 {
                        let token: H160 = Address::from_slice(&log.topics[1].0[12..]);
                        let from: H160 = Address::from_slice(&log.topics[2].0[12..]);
                        let to: H160 = if log.topics.len() > 3 {
                            Address::from_slice(&log.topics[3].0[12..]) // Normalement, ce serait ici, mais il semble que ce champ soit manquant
                        } else {
                            from // Si le recipient est absent, on met le sender
                        };
                        let amount: U256 = U256::from_big_endian(&log.data.0[0..32]);
                        let nonce: U256 = U256::from_big_endian(&log.data.0[32..64]);
        
                        println!("📩 Distribution détectée ! Insertion en cours...");
                        insert_distribution(&mut conn, token, from, to, amount, nonce);
                        println!("✅ Après insert_distribution !");
                    } else {
                        println!("❌ Erreur: Topics insuffisants pour une distribution !");
                    }
                } else {
                    println!("⚠️ Événement inconnu, signature non reconnue !");
                }
            }
        }
    
    println!("🔍 En attente de nouveaux événements...");
}

    println!("🔍 En attente de nouveaux événements...");
    Ok(())
}
