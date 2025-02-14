use core::time;
use std::{io::Write, thread};

use anyhow::Result;
use async_trait::async_trait;
use bitcoin::{
    key::{Keypair, Secp256k1},
    secp256k1::{All, SecretKey},
    taproot::{TaprootBuilder, TaprootSpendInfo},
    Address, Amount, Network, Transaction, TxOut,
};
use bitcoind::bitcoincore_rpc::{json::ScanTxOutRequest, RawTx, RpcApi};
use colored::Colorize;
use rand::Rng;


use crate::{
    bitcoin::{add_signature, CtfFramework, TransactionBuilder},
    constrants::PROJECTED_FEE,
    level::Level,
    utils::{print_failure_messege, print_success_messege},
};


// Contants
const TX_WAIT_TIME: u64 = 60;

pub struct LevelOne {
    target_tx: Transaction,
    ctf_framework: CtfFramework,
}

#[async_trait]
impl Level for LevelOne {
    async fn setup() -> Result<Self> {
        // spin up regtest
        let ctf_framework = CtfFramework::new()?;
        let client = &ctf_framework.bitcoind.client;

        // generate a keypair
        let (secp, kp, address, tr_spend_info) = level_setup();

        // fund generated keypair with 17Btc
        client.generate_to_address(118, &address)?;

        // compute recieve address
        let reciever_address = client
            .get_new_address(None, None)
            .unwrap()
            .require_network(bitcoind::bitcoincore_rpc::bitcoin::Network::Regtest)
            .unwrap();

        // get balance and verify
        let mut utxos_result = client.scan_tx_out_set_blocking(&[ScanTxOutRequest::Single(
            format!("tr({})", &kp.x_only_public_key().0),
        )])?;
        utxos_result.unspents.sort_by_key(|utxo| utxo.height);


        // Generate a new tx with 17 outputs all pointing to some random address.
        let mut tx_builder = TransactionBuilder::new(utxos_result.total_amount - PROJECTED_FEE);

        let mut prevouts = Vec::new();
        let mut amount = Amount::from_sat(0);

        for (added_inputs, utxo) in utxos_result.unspents.into_iter().enumerate() {
            if added_inputs >= 17 {
                break;
            }
            tx_builder.add_input(utxo.txid, utxo.vout);
            prevouts.push(TxOut {
                script_pubkey: utxo.script_pub_key,
                value: utxo.amount,
            });

            amount += utxo.amount;
        }

        let inputs = tx_builder.transaction.input.clone();
        let mut tx = tx_builder.build();
        let prevout_refs: Vec<&TxOut> = prevouts.iter().collect();

        tx.output[0].value = amount - Amount::from_sat((tx.vsize() * 2).try_into().unwrap());
        tx.output[0].script_pubkey = reciever_address.script_pubkey();

        for (input_idx, _) in inputs.iter().enumerate() {
            add_signature(
                &mut tx,
                input_idx,
                &prevout_refs,
                kp.secret_key(),
                Some(&tr_spend_info.tap_tweak().to_scalar()),
                &secp,
            )?;
        }

        println!("\n{}", "Transaction Hex:".cyan().bold());
        println!("{}", tx.raw_hex().bright_magenta());

        // Sign the Tx and cache/Log the tx.
        Ok(Self {
            target_tx: tx,
            ctf_framework,
        })
    }

    async fn run(&self) -> Result<bool> {
        // await for 1 minute and submit tx to regtest
        // if succeeds, return [false]
        // if not succeeds,
        // and if utxos_spent/17 > 6/10, return [true] else return false

        println!("\n{}", "Time remaining:".green().bold());

        // Countdown timer
        for i in (1..=TX_WAIT_TIME).rev() {
            print!("\r{i:02} seconds");
            std::io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }

        let result = self
            .ctf_framework
            .bitcoind
            .client
            .send_raw_transaction(&self.target_tx);

        println!("\n");
        if result.is_ok() {
            print_failure_messege();
            Ok(false)
        } else {
            print_success_messege();
            Ok(true)
        }

        // Ok(true)
    }

    async fn cleanup(&self) -> Result<()> {
        // award points and clean up the regtest save the game state.
        CtfFramework::clean()?;
        Ok(())
    }

    fn print_problem_statement() {
        println!("\n{}", level_one_title().bright_green().bold());
        println!("{}", "===============".bright_green());

        println!("\n{}", "Problem Statement:".yellow().bold());
        println!("{}", "Alice has a wallet, but a phishy wallet uses 1 bit security (nonce) to sign transactions.".bright_white());
        println!(
            "{}",
            "Note: All transactions signed by Alice's wallet are Taproot addresses.".bright_white()
        );
        println!(
            "{}",
            "Her transaction has entered the mempool and will be confirmed in one minute."
                .bright_white()
        );

        println!("\n{}", "Your Mission:".red().bold());
        println!("{}", "You have one minute to decode the transaction and drain Alice's funds from her wallet.".bright_white());
    }
}

pub fn level_setup() -> (Secp256k1<All>, Keypair, Address, TaprootSpendInfo) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    let data: [u8; 32] = rng.gen();
    let keypair = SecretKey::from_slice(&data).unwrap().keypair(&secp);

    let taproot_spend_info: TaprootSpendInfo = TaprootBuilder::new()
        .finalize(&secp, keypair.x_only_public_key().0)
        .expect("Failed to finalize script");

    let new_payout_address: Address =
        Address::p2tr_tweaked(taproot_spend_info.output_key(), Network::Regtest);

    (secp, keypair, new_payout_address, taproot_spend_info)
}

fn level_one_title() -> String {
    r"
  ┓       ┓  ┓
  ┃ ┏┓┓┏┏┓┃  ┃
  ┗┛┗ ┗┛┗ ┗  ┻"
        .to_string()
}
