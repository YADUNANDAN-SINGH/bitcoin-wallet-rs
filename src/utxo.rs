// This module talks to mempool.space's API to fetch UTXOs for an address.
// A UTXO (Unspent Transaction Output) is a "coin" our wallet can spend.

use serde::Deserialize;

// ---- Data models ----
// These structs mirror the JSON shape mempool.space returns. Serde uses the
// field names here to match the JSON keys automatically. Fields we don't need
// can simply be omitted — serde ignores unknown JSON keys by default.

// The "status" object nested inside each UTXO tells us if/when it was confirmed.
// block_height, block_hash, block_time are Option<T> because an unconfirmed
// UTXO (still in mempool) has confirmed: false and no block info yet.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Status {
    pub confirmed: bool,
    pub block_height: Option<u64>,
    pub block_hash: Option<String>,
    pub block_time: Option<u64>,
}

// One UTXO = one spendable coin at our address.
//   txid  = the transaction that created this output (hex string)
//   vout  = which output index within that transaction (0, 1, 2, ...)
//   value = amount in satoshis (1 BTC = 100,000,000 sats)
#[derive(Deserialize, Debug)]
pub struct Utxo {
    pub txid: String,
    pub vout: u64,
    pub status: Status,
    pub value: u64,
}

// ---- The main function of this module ----
// Given an address, query mempool.space and return all UTXOs at that address.
// Returns anyhow::Result so we can use ? to propagate errors from reqwest
// (network failure) or serde (JSON parse failure) without boilerplate.
pub fn fetch_utxos(address: &str) -> anyhow::Result<Vec<Utxo>> {
    // Build the full API URL by interpolating the address into the endpoint.
    // Using the parameter instead of hardcoding means this works for any address.
    let url = format!("https://mempool.space/signet/api/address/{}/utxo", address);

    // Blocking HTTP GET → parse the response body as JSON → into Vec<Utxo>.
    // The turbofish ::<Vec<Utxo>> tells serde what type to deserialize into.
    // Each `?` unwraps an Ok value or short-circuits with the error.
    let utxos = reqwest::blocking::get(url)?.json::<Vec<Utxo>>()?;

    Ok(utxos)
}