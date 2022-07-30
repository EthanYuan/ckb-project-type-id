const CKB_SUCCESS: i32 = 0;

#[link(name = "ckb-lib-type-id", kind = "static")]
extern "C" {
    fn validate_type_id(type_id: *const u8) -> i32;
}

pub fn verify_type_id(type_id: &mut [u8; 32]) -> Result<(), i32> {
    let error_code = unsafe { validate_type_id(type_id.as_mut_ptr()) };

    if error_code != CKB_SUCCESS {
        return Err(error_code);
    }
    Ok(())
}
