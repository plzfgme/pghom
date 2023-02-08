use kzen_paillier::*;

pub const ENCRYPTION_KEY_MAX_SIZE: usize = 623;
pub const DECRYPTION_KEY_MAX_SIZE: usize = 629;
pub const CIPHERTEXT_MAX_SIZE: usize = 1263;

pub fn keypair() -> (Vec<u8>, Vec<u8>) {
    let (ek, dk) = Paillier::keypair().keys();
    let mut ek_vec = Vec::new();
    let mut dk_vec = Vec::new();
    ciborium::ser::into_writer(&ek, &mut ek_vec).unwrap();
    ciborium::ser::into_writer(&dk, &mut dk_vec).unwrap();

    (ek_vec, dk_vec)
}

pub fn encrypt_u64(ek: &[u8], m: u64) -> Vec<u8> {
    let ek: EncryptionKey = ciborium::de::from_reader(ek).unwrap();
    let c = Paillier::encrypt(&ek, m);
    let mut c_vec = Vec::new();
    ciborium::ser::into_writer(&c, &mut c_vec).unwrap();

    c_vec
}

pub fn encrypt_u64_to_mut_slice(ek: &[u8], m: u64, out: &mut [u8]) {
    let ek: EncryptionKey = ciborium::de::from_reader(ek).unwrap();
    let c = Paillier::encrypt(&ek, m);
    ciborium::ser::into_writer(&c, out).unwrap();
}

pub fn decrypt_u64(dk: &[u8], c: &[u8]) -> u64 {
    let dk = ciborium::de::from_reader(dk).unwrap();
    let ctext: EncodedCiphertext<u64> = ciborium::de::from_reader(c).unwrap();

    Paillier::decrypt(&dk, ctext)
}
