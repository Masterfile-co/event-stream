use crate::{
    abi::GnosisSafeL2::events,
    pb::masterfile::{
        common::v1::TransactionMetadata,
        safe::v1::{
            safe_event::{Event as EventType, SafeMultiSigTransaction},
            SafeEvent,
        },
    },
    utils::pretty_hex,
};

use ethabi::{Address, Token, Uint};
use hex_literal::hex;
use substreams_ethereum::{block_view::LogView, Event};

use super::keccak256;

const SAFE_TX_TYPEHASH: [u8; 32] =
    hex!("bb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8");
pub const DOMAIN_SEPARATOR_TYPEHASH: [u8; 32] =
    hex!("47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a79469218");

pub const ERC191_BYTE: &'static str = "19";
pub const ERC191_VERSION: &'static str = "01";

pub fn translate(metadata: &Option<TransactionMetadata>, log: &LogView) -> Option<SafeEvent> {
    match events::SafeMultiSigTransaction::match_and_decode(log) {
        Some(event) => {
            let mut tx_event = map_transactions(&event);

            let (safe_tx_hash, nonce) = calculate_safe_tx_hash(
                event,
                &log.address(),
                137, // TODO! add as input parameter
            );

            tx_event.safe_tx_hash = pretty_hex(&safe_tx_hash);
            tx_event.nonce = nonce;

            Some(SafeEvent {
                r#event: Some(EventType::SafeMultisigTransaction(tx_event)),
                metadata: metadata.clone(),
                address: pretty_hex(&log.address()),
                ordinal: log.block_index() as u64,
            })
        }
        None => None,
    }
}

fn map_transactions(event: &events::SafeMultiSigTransaction) -> SafeMultiSigTransaction {
    SafeMultiSigTransaction {
        to: pretty_hex(&event.to),
        value: event.value.to_string(),
        data: pretty_hex(&event.data),
        operation: event.operation.to_u64(),
        safe_tx_gas: event.safe_tx_gas.to_string(),
        base_gas: event.base_gas.to_string(),
        gas_price: event.gas_price.to_string(),
        gas_token: pretty_hex(&event.gas_token),
        refund_receiver: pretty_hex(&event.refund_receiver),
        signatures: pretty_hex(&event.signatures),
        additional_info: pretty_hex(&event.additional_info),
        safe_tx_hash: "".to_string(),
        nonce: 0,
        result: None,
    }
}

/// https://github.com/safe-global/safe-client-gateway/blob/0957d8d66cc40b1c4c6556b33735e613caf50bce/src/utils/transactions.rs
fn calculate_safe_tx_hash(
    event: events::SafeMultiSigTransaction,
    address: &[u8],
    chain_id: u64,
) -> ([u8; 32], u64) {
    let safe_type_hash: Uint = Uint::from(SAFE_TX_TYPEHASH);
    let to: Address = Address::from_slice(&event.to);
    let value: Uint = Uint::from_big_endian(&event.value.to_signed_bytes_be());
    let data: Uint = Uint::from(keccak256(&event.data.to_vec()));
    let operation: Uint = Uint::from(event.operation.to_u64());
    let safe_tx_gas: Uint = Uint::from_big_endian(&event.safe_tx_gas.to_signed_bytes_be());
    let base_gas: Uint = Uint::from_big_endian(&event.base_gas.to_signed_bytes_be());
    let gas_price: Uint = Uint::from_big_endian(&event.gas_price.to_signed_bytes_be());
    let gas_token: Address = Address::from_slice(&event.gas_token);
    let refund_receiver: Address = Address::from_slice(&event.refund_receiver);
    let nonce: Uint = Uint::from_big_endian(&event.additional_info[0..32]);

    let hash = keccak256(ethabi::encode(&[
        Token::Uint(safe_type_hash),
        Token::Address(to),              // to
        Token::Uint(value),              // value
        Token::Uint(data),               // data
        Token::Uint(operation),          // operation
        Token::Uint(safe_tx_gas),        // safe_tx_gas
        Token::Uint(base_gas),           // base_gas
        Token::Uint(gas_price),          // gas_price
        Token::Address(gas_token),       // gas_token
        Token::Address(refund_receiver), // refund_receiver
        Token::Uint(nonce),              // base_gas                // nonce
    ]));

    let domain_hash = keccak256(ethabi::encode(&[
        Token::Uint(Uint::from(DOMAIN_SEPARATOR_TYPEHASH)),
        Token::Uint(Uint::from(chain_id)),
        Token::Address(Address::from_slice(address)),
    ]));

    let mut encoded = ethabi::encode(&[
        ethabi::Token::Uint(Uint::from(domain_hash)),
        ethabi::Token::Uint(Uint::from(hash)),
    ]);
    let erc_191_byte = u8::from_str_radix(ERC191_BYTE, 16).unwrap();
    let erc_191_version = u8::from_str_radix(ERC191_VERSION, 16).unwrap();

    encoded.insert(0, erc_191_version);
    encoded.insert(0, erc_191_byte);
    (keccak256(encoded), nonce.as_u64())
}
