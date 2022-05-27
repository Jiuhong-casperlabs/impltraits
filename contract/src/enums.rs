#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::vec::Vec;

use casper_contract::contract_api::{runtime, storage};
use casper_types::U256;
use casper_types::{
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, ContractHash, U512,
};

use core::mem;
pub const U8_SERIALIZED_LENGTH: usize = mem::size_of::<u8>();

pub enum Payment {
    CSPR {
        amount: U512,
    },
    ERC20 {
        contract_hash: ContractHash,
        amount: U256,
    },
    CEP47 {
        collection: ContractHash,
        token_id: U256,
    },
}
impl ToBytes for Payment {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        match self {
            Payment::CSPR { amount } => {
                buffer.insert(0, 0u8);
                buffer.extend(amount.to_bytes()?);
            }
            Payment::ERC20 {
                contract_hash,
                amount,
            } => {
                buffer.insert(0, 1u8);
                buffer.extend(contract_hash.to_bytes()?);
                buffer.extend(amount.to_bytes()?);
            }
            Payment::CEP47 {
                collection,
                token_id,
            } => {
                buffer.insert(0, 2u8);
                buffer.extend(collection.to_bytes()?);
                buffer.extend(token_id.to_bytes()?);
            }
        }
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        mem::size_of::<u8>()
            + match self {
                Payment::CSPR { amount } => amount.serialized_length(),
                Payment::ERC20 {
                    contract_hash,
                    amount,
                } => contract_hash.serialized_length() + amount.serialized_length(),
                Payment::CEP47 {
                    collection,
                    token_id,
                } => collection.serialized_length() + token_id.serialized_length(),
            }
    }
}

impl FromBytes for Payment {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (tag, remainder) = u8::from_bytes(bytes)?;
        match tag {
            0 => {
                let (amount, remainder) = U512::from_bytes(remainder)?;

                Ok((Payment::CSPR { amount }, remainder))
            }
            1 => {
                let (contract_hash, remainder) = FromBytes::from_bytes(remainder)?;
                let (amount, remainder) = U256::from_bytes(remainder)?;
                Ok((
                    Payment::ERC20 {
                        contract_hash,
                        amount,
                    },
                    remainder,
                ))
            }
            2 => {
                let (collection, remainder) = FromBytes::from_bytes(remainder)?;
                let (token_id, remainder) = U256::from_bytes(remainder)?;
                Ok((
                    Payment::CEP47 {
                        collection,
                        token_id,
                    },
                    remainder,
                ))
            }
            _ => Err(bytesrepr::Error::Formatting),
        }
    }
}
impl CLTyped for Payment {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let mem1 = Payment::CSPR {
        amount: U512::from(123u64),
    };
    let mem2 = Payment::ERC20 {
        contract_hash: ContractHash::from_formatted_str(
            "hash-033a6a5f47f9f247e1a3bd1307ea5d94a232ddec05aaa6b91363589e94728381",
        )
        .unwrap(),
        amount: U256::from(1u64),
    };
    let mem3 = Payment::CEP47 {
        collection: ContractHash::from_formatted_str(
            "hash-033a6a5f47f9f247e1a3bd1307ea5d94a232ddec05aaa6b91363589e94728381",
        )
        .unwrap(),
        token_id: U256::from(1u64),
    };

    runtime::put_key("mem1", storage::new_uref(mem1).into());
    runtime::put_key("mem2", storage::new_uref(mem2).into());
    runtime::put_key("mem3", storage::new_uref(mem3).into());
}
