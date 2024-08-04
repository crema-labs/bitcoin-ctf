// Transaction Builder

use anyhow::Result;
use bitcoin::{
    absolute::LockTime,
    hashes::Hash,
    key::{Keypair, Secp256k1},
    secp256k1::{All, Message, Scalar, SecretKey},
    sighash::{Prevouts, SighashCache},
    transaction::Version,
    Address, Amount, OutPoint, ScriptBuf, Sequence, TapSighashType, Transaction, TxIn, TxOut, Txid,
    Witness,
};
use rand::Rng;

/// A builder for constructing Bitcoin transactions.
pub struct TransactionBuilder {
    /// The transaction being built.
    pub transaction: Transaction,
}

impl TransactionBuilder {
    /// Creates a new TransactionBuilder with default values.
    ///
    /// # Returns
    /// A new `TransactionBuilder` instance with an empty transaction.
    pub fn new(amount_out: Amount) -> Self {
        Self {
            transaction: Transaction {
                version: Version::TWO,
                lock_time: LockTime::ZERO,
                input: vec![],
                output: vec![TxOut {
                    value: amount_out,
                    script_pubkey: ScriptBuf::default(),
                }],
            },
        }
    }

    /// Adds an input to the transaction.
    ///
    /// # Arguments
    /// * `txid` - The transaction ID of the input
    /// * `vout` - The output index of the input
    ///
    /// # Returns
    /// Self, allowing for method chaining
    pub fn add_input(&mut self, txid: Txid, vout: u32) {
        self.transaction.input.push(TxIn {
            previous_output: OutPoint { txid, vout },
            script_sig: ScriptBuf::new(), // Empty script signature (would remain empty since most of txs are p2tr)
            sequence: Sequence::MAX,      // Set sequence to maximum value
            witness: Witness::default(),  // Default (empty) witness
        });
    }

    /// Adds an output to the transaction.
    ///
    /// # Arguments
    /// * `address` - The recipient's Bitcoin address
    /// * `amount` - The amount of Bitcoin to send
    ///
    /// # Returns
    /// Self, allowing for method chaining
    #[allow(dead_code)]
    pub fn add_output(mut self, address: &Address, amount: Amount) -> Self {
        self.transaction.output.push(TxOut {
            value: amount,
            script_pubkey: address.script_pubkey(),
        });
        self
    }

    /// Builds the transaction.
    ///
    /// # Returns
    /// The built transaction
    pub fn build(self) -> Transaction {
        self.transaction
    }
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
