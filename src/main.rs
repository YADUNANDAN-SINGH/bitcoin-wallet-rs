// ---- Imports ----
// Hash functions used by Bitcoin (SHA-256 twice, then RIPEMD-160) for learning/display
use bitcoin::hashes::{ripemd160, sha256, Hash};
// Core types: Address (human-readable), Network (mainnet/signet/testnet)
use bitcoin::{Address, Network};
// Keypair generation: Secp256k1 is the curve, OsRng is a secure random source
use secp256k1::{rand::rngs::OsRng, Secp256k1};
// Bitcoin's wrappers for keys — know how to format themselves as WIF / compressed
use bitcoin::PrivateKey;
use bitcoin::CompressedPublicKey;
use bitcoin::consensus::encode::serialize_hex;

use std::path::Path;

// Declare our UTXO module — tells Rust to compile src/utxo.rs as part of this crate
mod utxo;
mod tx;

// Standard library: for writing the key info to a local file
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

// Main returns anyhow::Result so we can use `?` to short-circuit on errors.
// If the program fails anywhere, main returns Err and the CLI exits with code 1.

fn top_secret_exist() -> bool {
    Path::new("Top_secret.txt").exists()
}

fn read_secret_txt() -> anyhow::Result<()> {
    let file = File::open("Top_secret.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn load_keys_from_file() -> anyhow::Result<(PrivateKey, CompressedPublicKey, Address)> {
    let file = File::open("Top_secret.txt")?;
    let reader = BufReader::new(file);

    let mut wif: Option<PrivateKey> = None;
    let mut pubkey: Option<CompressedPublicKey> = None;
    let mut address: Option<Address> = None;

    for line in reader.lines() {
        let line = line?;
        if let Some(val) = line.strip_prefix("WIF Private Key: ") {
            wif = Some(val.trim().parse::<PrivateKey>()?);
        } else if let Some(val) = line.strip_prefix("Public Key: ") {
            pubkey = Some(val.trim().parse::<CompressedPublicKey>()?);
        } else if let Some(val) = line.strip_prefix("Address: ") {
            address = Some(val.trim().parse::<Address<_>>()?.require_network(Network::Signet)?);
        }
    }

    match (wif, pubkey, address) {
        (Some(w), Some(p), Some(a)) => Ok((w, p, a)),
        _ => anyhow::bail!("Missing WIF, Public Key, or Address in Top_secret.txt"),
    }
}

fn main() -> anyhow::Result<()> {
    println!("welcome to your Bitcoin wallet");

    // Declared here so both branches can assign them and the tx block below can use them.
    let utxos: Vec<utxo::Utxo>;
    let address: Address;

    if top_secret_exist() {
        println!("Here is your existing secret:");
        read_secret_txt().expect("Failed to read Top_secret.txt");

        println!("here is your wallet status : ");

        let (btc_private_key, compressed, loaded_address) = load_keys_from_file()?;
        let _ = (btc_private_key, compressed);
        address = loaded_address;

        utxos = utxo::fetch_utxos(&address.to_string())?;
        if utxos.is_empty() {
            println!("\nNo UTXOs yet — hit the faucet!");
        } else {
            println!("\nFound {} UTXO(s):", utxos.len());
            for u in &utxos {
                println!("  {}:{} → {} sats (confirmed: {})",
                    u.txid, u.vout, u.value, u.status.confirmed);
            }
        }
    } else {

        // ---- STEP 1: Generate a keypair ----
        // Secp256k1 context — holds precomputed tables for fast curve operations.
        let secp = Secp256k1::new();

        // OsRng = cryptographically secure RNG from the operating system.
        // Never use a weak RNG here — a predictable private key = stolen coins.
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        // Debug-print to verify we got a keypair. SecretKey's Debug is redacted
        // ("#SecretKey") on purpose so we don't accidentally leak keys in logs.
        println!("\nSecret Key: {:?}\nPublic Key: {:?}", secret_key, public_key);

        // ---- STEP 2: Learning detour — show how a legacy address is built ----
        // This is what Grokking Bitcoin Ch. 3 describes. SegWit (our actual address
        // below) uses a similar hashing step but encodes with bech32 instead of Base58.

        // SHA-256 of the serialized (compressed) public key.
        let sha256_hash = sha256::Hash::hash(&public_key.serialize());
        println!("\nHashed version of public key: {sha256_hash}");

        // Same hash in hex form (just for visual confirmation — same bytes).
        let hexa = hex::encode(sha256_hash.as_byte_array());
        println!("Hex version of hashed public key: {hexa}");

        // RIPEMD-160 of that SHA-256 hash = the 20-byte "public key hash" (PKH)
        // that identifies this wallet. This is the core of most Bitcoin addresses.
        let ripmed_hash = ripemd160::Hash::hash(sha256_hash.as_ref());
        let final_hex = hex::encode(ripmed_hash.as_ref() as &[u8]);
        println!("Final 20-byte Public Key Hash: {final_hex}");

        // ---- STEP 3: Build the actual SegWit address we'll use ----
        // Wrap the secret key as a Bitcoin PrivateKey tagged for the Signet network.
        // Same bytes — just adds network context so WIF encoding uses the right prefix.
        let btc_private_key = PrivateKey::new(secret_key, Network::Signet);

        // SegWit (P2WPKH) requires a COMPRESSED public key (33 bytes, not 65).
        // Returns Result because the input key could theoretically be malformed.
        let compressed = CompressedPublicKey::from_private_key(&secp, &btc_private_key)
            .expect("Failed to compress public key");

        // Generate a native SegWit (P2WPKH) address — starts with "tb1q..." on signet.
        // This is the address we share with faucets and receive coins at.
        address = Address::p2wpkh(&compressed, Network::Signet);
        println!("Address: {address}");

        // ---- STEP 4: Persist the key so we don't lose access to our coins ----
        // Open (or create) Top_secret.txt for writing.
        //   create(true)   = make the file if it doesn't exist
        //   write(true)    = we intend to write
        //   truncate(true) = wipe existing contents so each run overwrites cleanly
        //                    (otherwise we'd end up with many keypairs stacked)
        let mut txt_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("Top_secret.txt")
        .expect("Failed to open Top_secret.txt");
         // Save the WIF private key (standard Bitcoin import format — starts with "c"
        // on signet), the public key, and the address. WIF is what we'd paste into
        // another wallet to import this key. MUST be in .gitignore — never commit.
         writeln!(
            txt_file,
            "WIF Private Key: {}\nPublic Key: {}\nAddress: {}",
            btc_private_key, public_key, address
            ).expect("Failed to write to Top_secret.txt");

        // ---- STEP 5: Check the blockchain for coins at our new address ----
        utxos = utxo::fetch_utxos(&address.to_string())?;

        if utxos.is_empty() {
            println!("\nNo UTXOs yet — hit the faucet!");
        } else {
            println!("\nFound {} UTXO(s):", utxos.len());
            for u in &utxos {
                println!("  {}:{} → {} sats (confirmed: {})",
                    u.txid, u.vout, u.value, u.status.confirmed);
            }
        }
    }

    if let Some(utxo) = utxos.first() {
        let recipient_address: Address = "tb1qqgeguwml9x9jpklhq4q8uhqr69npt9llw3h0lk"
            .parse::<Address<_>>()?
            .require_network(Network::Signet)?;

        let tx = tx::build_unsigned_transaction(utxo, &recipient_address, 50_000, &address, 500)?;

        println!("\n--- Unsigned transaction built ---");
        println!("Provisional txid: {}", tx.compute_txid());
        println!("Inputs:  {}", tx.input.len());
        println!("Outputs: {}", tx.output.len());
        println!("Raw hex: {}", serialize_hex(&tx));
    }

    // Explicit Ok(()) because main returns Result. This signals "success, no value".
    Ok(())
}
