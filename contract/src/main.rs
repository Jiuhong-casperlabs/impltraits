#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{vec, vec::Vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::U256;
use casper_types::{
    account::AccountHash,
    bytesrepr::{self, FromBytes, ToBytes},
    CLType, CLTyped, U512,
};

pub struct OfferItem {
    pub maker: AccountHash,
    pub price: U512,
    pub offer_time: u64,
}

pub struct Offer {
    pub id: U256,
    pub token_id: U256,
    pub offers: Vec<OfferItem>,
}

impl CLTyped for Offer {
    fn cl_type() -> CLType {
        CLType::Any
    }
}

impl FromBytes for OfferItem {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (maker, bytes) = AccountHash::from_bytes(bytes)?;
        let (price, bytes) = U512::from_bytes(bytes)?;
        let (offer_time, bytes) = u64::from_bytes(bytes)?;
        let body = OfferItem {
            maker,
            price,
            offer_time,
        };
        Ok((body, bytes))
    }
}

impl ToBytes for OfferItem {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.maker.to_bytes()?);
        buffer.extend(self.price.to_bytes()?);
        buffer.extend(self.offer_time.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.maker.serialized_length()
            + self.price.serialized_length()
            + self.offer_time.serialized_length()
    }
}

impl FromBytes for Offer {
    fn from_bytes(bytes: &[u8]) -> Result<(Self, &[u8]), bytesrepr::Error> {
        let (id, bytes) = U256::from_bytes(bytes)?;
        let (token_id, bytes) = U256::from_bytes(bytes)?;
        let (offers, bytes) = Vec::<OfferItem>::from_bytes(bytes)?;
        let body = Offer {
            id,
            token_id,
            offers,
        };
        Ok((body, bytes))
    }
}

impl ToBytes for Offer {
    fn to_bytes(&self) -> Result<Vec<u8>, bytesrepr::Error> {
        let mut buffer = bytesrepr::allocate_buffer(self)?;
        buffer.extend(self.id.to_bytes()?);
        buffer.extend(self.token_id.to_bytes()?);
        buffer.extend(self.offers.to_bytes()?);
        Ok(buffer)
    }

    fn serialized_length(&self) -> usize {
        self.id.serialized_length()
            + self.token_id.serialized_length()
            + self.offers.serialized_length()
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let offeritem1 = OfferItem {
        maker: AccountHash::from_formatted_str(
            "account-hash-ad7e091267d82c3b9ed1987cb780a005a550e6b3d1ca333b743e2dba70680877",
        )
        .unwrap(),
        price: U512::from(100u64),
        offer_time: 1234,
    };

    let offeritem2 = OfferItem {
        maker: AccountHash::from_formatted_str(
            "account-hash-ad7e091267d82c3b9ed1987cb780a005a550e6b3d1ca333b743e2dba70680877",
        )
        .unwrap(),
        price: U512::from(100u64),
        offer_time: 1234,
    };

    let offer = Offer {
        id: U256::from(1u64),
        token_id: U256::from(100u64),
        offers: vec![offeritem1, offeritem2],
    };

    runtime::put_key("test", storage::new_uref(offer).into())
}
