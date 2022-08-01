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
    high_level::{load_cell_data, load_script, QueryIter},
};

use super::helper;
use crate::error::Error;

pub fn main() -> Result<(), Error> {
    // remove below examples and write your code here

    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    // return an error if args is invalid
    if args.len() != 32 {
        return Err(Error::InvalidArgs);
    }

    // check type id
    let mut type_id = [0u8; 32];
    type_id.copy_from_slice(&args[0..32]);
    let ret = helper::verify_type_id(&mut type_id);
    debug!("{:?}", ret);
    if ret.is_err() {
        return Err(Error::InvalidTypeId);
    }

    // check data
    if !QueryIter::new(load_cell_data, Source::Output).any(|data| {
        debug!("data is {:?}", data);
        data.len() >= 29 // MIN: pledge info + 1 milestone info
    }) {
        return Err(Error::InvalidData);
    }

    Ok(())
}
