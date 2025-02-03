use ethers::providers::{Http, Middleware, Provider};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); // Загружаем переменные из .env

    let rpc_url = env::var("ETH_RPC_URL").expect("ETH_RPC_URL не найден");
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // Новый способ получения номера блока
    let block_number = provider.get_block_number().await?;
    println!("Текущий номер блока: {}", block_number);

    Ok(())
}