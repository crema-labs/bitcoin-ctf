use anyhow::Result;
use async_trait::async_trait;
use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    key::{Keypair, Secp256k1},
    secp256k1::{All, Message, Scalar, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::Version,
    Address, Amount, Network, ScriptBuf, TapSighashType, Transaction, TxOut,
};
use bitcoind::bitcoincore_rpc::{json::ScanTxOutRequest, RpcApi};
use rand::Rng;

use crate::{bitcoin::CtfFramework, level::Level};

pub struct LevelOne {
    target_tx: Transaction,
}

#[async_trait]
impl Level for LevelOne {
    async fn setup() -> Result<Transaction> {
        // spin up regtest
        let regtest = CtfFramework::new()?;

        // generate a keypair
        let (_, kp, address) = setup();

        // fund generated keypair with 17Btc
        regtest.client.generate_to_address(101, &address)?;

        // get balance and verify
        let utxos_result = regtest
            .client
            .scan_tx_out_set_blocking(&[ScanTxOutRequest::Single(format!(
                "tr({})",
                &kp.x_only_public_key().0
            ))])?;
        // let total_amount = utxosResult.total_amount;

        println!("result: {:?}", utxos_result);
        // Generate a new tx with 17 outputs all pointing to some random address.
        // todo

        // Sign the Tx and cache/Log the tx.
        Ok(create_dummy_transaction(Amount::from_sat(17 * 100_000_000)))
    }


    async fn run(tx: Transaction) -> Result<bool> {
        // await for 1 minute and submit tx to regtest
        // if succeeds, return [false]
        // if not succeeds,
        // and if utxos_spent/17 > 6/10, return [true] else return false
        Ok(true)
    }

    
    async fn cleanup(&self) -> Result<()> {
        // award points and clean up the regtest save the game state.
        Ok(())
    }
}

pub fn setup() -> (Secp256k1<All>, Keypair, Address) {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    let data: [u8; 32] = rng.gen();
    let keypair = SecretKey::from_slice(&data).unwrap().keypair(&secp);

    let new_payout_address: Address =
        Address::p2tr(&secp, keypair.x_only_public_key().0, None, Network::Regtest);

    (secp, keypair, new_payout_address)
}

/// Adds a signature to a transaction input.
///
/// This function creates a signature for the specified input of a transaction,
/// optionally applying a tweak to the private key before signing.
/// Note : Uses Single Byte Nonce.
pub fn add_signature(
    transaction: &mut Transaction,
    input_idx: usize,
    prevouts: &[&TxOut],
    private_key: SecretKey,
    tweak: Option<&Scalar>,
    secp: &Secp256k1<All>,
) -> Result<()> {
    // apply tweak if provided
    let keypair: Keypair = match tweak {
        Some(tweak) => private_key.keypair(secp).add_xonly_tweak(secp, tweak)?,
        None => private_key.keypair(secp),
    };

    // Compute the sighash
    let mut sighash_cache = SighashCache::new(transaction.clone());
    let sighash = sighash_cache.taproot_key_spend_signature_hash(
        input_idx,
        &Prevouts::All(prevouts),
        TapSighashType::All,
    )?;

    // Create and sign the message
    let message = Message::from_digest(sighash.as_raw_hash().to_byte_array());

    let mut aux_rand = [0u8; 32];
    aux_rand[31] = rand::thread_rng().gen();
    let signature = secp.sign_schnorr_with_aux_rand(&message, &keypair, &aux_rand);

    // Verify the signature
    secp.verify_schnorr(&signature, &message, &keypair.x_only_public_key().0)?;

    // Add the signature to the transaction input
    let mut vec_sig = signature.serialize().to_vec();
    vec_sig.push(0x01);
    transaction.input[input_idx].witness.push(vec_sig);

    Ok(())
}

fn create_dummy_transaction(amount_out: Amount) -> Transaction {
    let tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![],
        output: vec![TxOut {
            value: amount_out,
            script_pubkey: ScriptBuf::default(),
        }],
    };
    tx
}
