// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    high_level::{
        load_cell_data, load_cell_data_hash, load_cell_type_hash, load_script,
        QueryIter,
    },
};

use super::helper;
use crate::error::Error;

pub fn main() -> Result<(), Error> {
    // remove below examples and write your code here

    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    // return an error if args is invalid
    if args.is_empty() {
        return Err(Error::MyError);
    }

    // check args
    if !QueryIter::new(load_cell_data_hash, Source::Output).any(|data_hash| {
        debug!("data_hash is {:?}", data_hash);
        data_hash == args[0..32]
    }) {
        return Err(Error::InvalidArgs);
    }

    // check type id
    if !QueryIter::new(load_cell_type_hash, Source::Output).all(|script_hash| {
        debug!("script_hash is {:?}", script_hash);
        if let Some(mut script_hash) = script_hash {
            helper::verify_type_id(&mut script_hash).is_ok()
        } else {
            true
        }
    }) {
        return Err(Error::InvalidTypeId);
    }

    // check data
    if !QueryIter::new(load_cell_data, Source::Output).any(|_data| {
        debug!("data is {:?}", _data);
        true
    }) {
        return Err(Error::InvalidData);
    }

    Ok(())
}
