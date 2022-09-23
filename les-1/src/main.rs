use std::error::Error;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{system_transaction, signature::{Keypair, Signature}, signer::Signer};

const LAMPORTS_PER_SOL: f64 = 1000000000.0;
const URL: &str = "https://api.devnet.solana.com";

pub fn create_keypair() -> Keypair {
    Keypair::new()
}

pub fn check_balance(rpc_client: &RpcClient, public_key: &Pubkey) -> Result<f64, Box<dyn Error>> {
    Ok(rpc_client.get_balance(&public_key)? as f64 / LAMPORTS_PER_SOL)
}

pub fn request_air_drop(rpc_client: &RpcClient, pub_key: &Pubkey, amount_sol: f64) -> Result<Signature, Box<dyn Error>> {
    let sig = rpc_client.request_airdrop(&pub_key, (amount_sol * LAMPORTS_PER_SOL) as u64)?;
    loop {
        let confirmed = rpc_client.confirm_transaction(&sig)?;
        if confirmed {
            break;
        }
    }
    Ok(sig)
}

pub fn transfer_funds(rpc_client: &RpcClient, sender_keypair: &Keypair, receiver_pub_key: &Pubkey, amount_sol: f64) 
        -> core::result::Result<Signature, Box<dyn Error>> {
    let amount_lamports = (amount_sol * LAMPORTS_PER_SOL) as u64;
    
    Ok(rpc_client.send_and_confirm_transaction(
        &system_transaction::transfer(
            &sender_keypair, &receiver_pub_key, 
            amount_lamports, 
            rpc_client.get_latest_blockhash()?))?)
}


fn main() {
    let rpc_client = RpcClient::new(URL);

    let sender = create_keypair();
    let receiver = create_keypair();

    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &sender.pubkey(), 1.0) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }

        let transfer_amount = 0.5;

        match transfer_funds(&rpc_client, &sender, &receiver.pubkey(), transfer_amount) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Airdrop failed");
    }
}