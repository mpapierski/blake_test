extern crate blake2;
extern crate casperlabs_contract_ffi;
#[macro_use]
extern crate failure;
use blake2::{
    digest::{Input, VariableOutput},
    VarBlake2b,
};
use casperlabs_contract_ffi::bytesrepr::{Error as BytesReprError, ToBytes};
use failure::Error;

#[derive(Debug, Fail)]
enum CommonError {
    #[fail(display = "{:?}", _0)]
    BytesRepr(BytesReprError),
}

impl From<BytesReprError> for CommonError {
    fn from(e: BytesReprError) -> CommonError {
        CommonError::BytesRepr(e)
    }
}

fn new_function_address(pubkey: [u8; 32], nonce: u64, fn_store_id: u32) -> Result<(), CommonError> {
    let mut pre_hash_bytes = Vec::with_capacity(44); //32 byte pk + 8 byte nonce + 4 byte ID
    pre_hash_bytes.extend_from_slice(&pubkey);
    pre_hash_bytes.append(&mut nonce.to_bytes()?);
    pre_hash_bytes.append(&mut fn_store_id.to_bytes()?);

    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.input(&pre_hash_bytes);
    let mut hash_bytes = [0; 32];
    hasher.variable_result(|hash| hash_bytes.clone_from_slice(hash));
    println!("hash bytes {:?}", hash_bytes);
    Ok(())
}

fn main() -> Result<(), Error> {
    new_function_address([48; 32], 3, 0)?;
    Ok(())
}
