use std::slice;

use pghom::paillier::{
    decrypt_u64, encrypt_u64, keypair, CIPHERTEXT_MAX_SIZE, DECRYPTION_KEY_MAX_SIZE,
    ENCRYPTION_KEY_MAX_SIZE,
};

#[no_mangle]
pub extern "C" fn paillier_encryption_key_max_size() -> usize {
    return ENCRYPTION_KEY_MAX_SIZE;
}

#[no_mangle]
pub extern "C" fn paillier_decryption_key_max_size() -> usize {
    return DECRYPTION_KEY_MAX_SIZE;
}

#[no_mangle]
pub extern "C" fn paillier_keypair(
    ek: *mut u8,
    dk: *mut u8,
    ek_size: *mut usize,
    dk_size: *mut usize,
) {
    let (ek_vec, dk_vec) = keypair();

    unsafe {
        let ek = slice::from_raw_parts_mut(ek, paillier_encryption_key_max_size());
        let dk = slice::from_raw_parts_mut(dk, paillier_decryption_key_max_size());

        ek.copy_from_slice(&ek_vec);
        dk.copy_from_slice(&dk_vec);

        *ek_size = ek_vec.len();
        *dk_size = dk_vec.len();
    }
}

#[no_mangle]
pub extern "C" fn paillier_u64_ciphertext_max_size() -> usize {
    return CIPHERTEXT_MAX_SIZE;
}

#[no_mangle]
pub unsafe extern "C" fn paillier_encrypt_u64(
    ek: *const u8,
    ek_size: usize,
    input: u64,
    output: *mut u8,
    output_size: *mut usize,
) {
    let ek = slice::from_raw_parts(ek, ek_size as usize);
    let output = slice::from_raw_parts_mut(output, paillier_u64_ciphertext_max_size());

    let output_vec = encrypt_u64(ek, input);
    output.copy_from_slice(&output_vec);

    *output_size = output_vec.len();
}

#[no_mangle]
pub unsafe extern "C" fn paillier_decrypt_u64(
    dk: *const u8,
    dk_size: usize,
    input: *const u8,
    output: *mut u64,
) {
    let dk = slice::from_raw_parts(dk, dk_size);
    let input = slice::from_raw_parts(input, paillier_u64_ciphertext_max_size());

    *output = decrypt_u64(dk, input);
}
