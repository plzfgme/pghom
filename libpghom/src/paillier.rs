#[no_mangle]
pub extern "C" fn paillier_keypair(_ek: *mut u8, _dk: *mut u8) {
    todo!()
}

#[no_mangle]
pub extern "C" fn paillier_encrypt_u64(_ek: *const u8, _in: u64, _out: *mut u8) {
    todo!()
}

#[no_mangle]
pub extern "C" fn paillier_decrypt_u64(_dk: *const u8, _in: *const u8, _out: *mut u64) {
    todo!()
}
