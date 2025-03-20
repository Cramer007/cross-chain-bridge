use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use web3::types::{H160, U256};
use std::env;
use crate::schema::deposits;
use crate::schema::distributions;
use serde::Serialize;
use serde::Deserialize;

// Fonction pour établir la connexion à la base de données SQLite
pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Erreur de connexion à {}", database_url))
}

// Marquer un dépôt comme traité
pub fn update_deposit_as_processed(conn: &mut SqliteConnection, nonce_value: i32) {
    use crate::schema::deposits::dsl::*;

    diesel::update(deposits.filter(nonce.eq(nonce_value)))
        .set(processed.eq(true))
        .execute(conn)
        .expect("Erreur lors de la mise à jour du dépôt comme traité");
}

// Structure pour insérer un dépôt
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = deposits)]
pub struct NewDeposit {
    pub token: String,
    pub sender: String,
    pub recipient: String,
    pub amount: String,
    pub nonce: i32,
    pub processed: bool,
}


// Fonction pour insérer un dépôt
pub fn insert_deposit(
    conn: &mut SqliteConnection,
    token: H160,
    sender: H160,
    recipient: H160,
    amount: U256,
    nonce: U256,
) {
    let new_deposit = NewDeposit {
        token: format!("{:?}", token),
        sender: format!("{:?}", sender),
        recipient: format!("{:?}", recipient),
        amount: format!("{}", amount),
        nonce: nonce.low_u32() as i32,
        processed: false,
    };

    match diesel::insert_into(deposits::table)
        .values(&new_deposit)
        .execute(conn)
    {
        Ok(_) => println!("✅ Dépôt enregistré dans la base de données"),
        Err(e) => eprintln!("❌ Erreur lors de l'insertion du dépôt : {:?}", e),
    }
}

// Structure pour insérer une distribution
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = distributions)]
pub struct NewDistribution {
    pub token: String,
    pub sender: String,
    pub recipient: String,
    pub amount: String,
    pub nonce: i32,
    pub processed: bool,
}
// Fonction pour insérer une distribution
pub fn insert_distribution(
    conn: &mut SqliteConnection,
    token: H160,
    sender: H160,
    recipient: H160,
    amount: U256,
    nonce: U256,
) {
    println!("🚀 insert_distribution appelé !");
    println!("🔎 Paramètres reçus - Token: {:?}, Sender: {:?}, Recipient: {:?}, Amount: {}, Nonce: {}",
        token, sender, recipient, amount, nonce
    );

    let nonce_value = nonce.low_u32() as i32;
    println!("🔹 Nonce converti en i32: {}", nonce_value);

    let amount_str = format!("{}", amount);
    println!("🔹 Amount converti en String: {}", amount_str);

    let new_distribution = NewDistribution {
        token: format!("{:?}", token),
        sender: format!("{:?}", sender),
        recipient: format!("{:?}", recipient),
        amount: amount_str,
        nonce: nonce_value,
        processed: false,
    };

    println!("🔹 Tentative d'insertion Distribution: ");

    match diesel::insert_into(distributions::table)
        .values(&new_distribution)
        .execute(conn)
    {
        Ok(_) => println!("✅ Distribution enregistrée dans la base de données"),
        Err(diesel::result::Error::DatabaseError(kind, info)) => {
            eprintln!("❌ Erreur lors de l'insertion de la distribution : {:?} - {:?}", kind, info);
        }
        Err(e) => eprintln!("❌ Erreur inattendue lors de l'insertion de la distribution : {:?}", e),
    }
}
