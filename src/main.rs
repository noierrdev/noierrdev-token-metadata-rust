use anyhow::{Context, Result};
use clap::Parser;

use std::{
    env,
    time::{  SystemTime, UNIX_EPOCH},
};
use tokio::time::{Duration};

use solana_sdk::{
    bs58,
    signature::{Keypair,Signature,Signer},
    pubkey::Pubkey,
    instruction::{AccountMeta, Instruction},
    message::Message,
    transaction::Transaction,
    hash::Hash,
    commitment_config::CommitmentConfig,
    system_instruction,
    compute_budget::{ComputeBudgetInstruction}
};

use solana_client::{
    rpc_client::RpcClient,
    tpu_client::{TpuClient, TpuClientConfig},
    rpc_response::RpcContactInfo
};

use spl_associated_token_account::{get_associated_token_address};
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::{
    instruction::close_account,
    instruction::transfer,
    instruction::sync_native,
    id as token_program_id, // Import the token program ID
};

use base64::{engine::general_purpose, Engine as _};

use reqwest::Client;
use serde_json::json;
use serde_json::Value;

use mpl_token_metadata::accounts::Metadata;
use mpl_token_metadata::types::TokenStandard;


#[derive(Debug, BorshDeserialize, BorshSchema)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: u8,
    pub share: u8,
}

#[derive(Debug, BorshDeserialize, BorshSchema)]
pub struct Data {
    pub name: String,   // max 32 chars
    pub symbol: String, // max 10 chars
    pub uri: String,    // max 200 chars
    pub seller_fee_basis_points: u16,
    pub has_creators: Option<Vec<Creator>>, // manually handled below
}

#[derive(Debug, BorshDeserialize, BorshSchema)]
pub struct Metadata {
    pub key: u8, // 4 = MetadataV1
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub data: Data,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,

    // The rest are optional: token_standard, collection, uses, etc.
    // You can extend with Option fields as needed
}

#[tokio::main]
async fn main() {
    
    dotenv::dotenv().ok();

    let sol_mint="So11111111111111111111111111111111111111112";
    let metaplex_program="metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
    let mint="E7t1xhf3b5QzRnJSaQ3fmBRBP63KmTNabZFqswsgpump";
    

    //Create web3 connection
    let rpc_api_str = env::var("RPC_API").unwrap();
    let rpc_url = rpc_api_str;
    let commitment = CommitmentConfig::processed();
    let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(),commitment);

    let http_client=Client::new();

    let (metaplex_pda, metaplex_bump) = Pubkey::find_program_address(&[
        b"metadata",
        &Pubkey::from_str_const(metaplex_program).as_ref(),
        &Pubkey::from_str_const(mint).as_ref(),
    ], &Pubkey::from_str_const(metaplex_program));

    println!("{}", metaplex_pda.to_string());

    let metaplex_account_info=rpc_client.get_account(&metaplex_pda).unwrap();
    
    let metaplex_account_info_data=metaplex_account_info.data;
    println!("{:?}",metaplex_account_info_data);
}

