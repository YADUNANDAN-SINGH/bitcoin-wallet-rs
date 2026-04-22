use bitcoin::absolute::LockTime;
use bitcoin::{Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness};
use bitcoin::transaction::Version;
use std::str::FromStr;


pub fn build_unsigned_transaction(
    utxo: &crate::utxo::Utxo,
    recipient_address: &Address,
    amount_to_send: u64,
    change_address: &Address,
    fee: u64,
) -> anyhow::Result<Transaction>{
    if utxo.value < amount_to_send + fee {
        anyhow::bail!("You do not heve enough funds!");
    }
    let change = utxo.value - amount_to_send - fee;
    let txid = Txid::from_str(&utxo.txid)?;

    let outpoint = OutPoint {
        txid,
        vout: utxo.vout as u32
    };

    let tx_in = TxIn {
        previous_output: outpoint,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };

    let recipient_output = TxOut {
        value: Amount::from_sat(amount_to_send),
        script_pubkey: recipient_address.script_pubkey(),
    };

    let change_output = TxOut {
    value: Amount::from_sat(change),
    script_pubkey: change_address.script_pubkey(),
    };

    let tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![tx_in],
        output: vec![recipient_output, change_output],
    };

    Ok(tx)
}