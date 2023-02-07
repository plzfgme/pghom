use kzen_paillier::*;

pub fn keypair() -> (Vec<u8>, Vec<u8>) {
    let (ek, dk) = Paillier::keypair().keys();
    let mut ek_vec = Vec::new();
    let mut dk_vec = Vec::new();
    ciborium::ser::into_writer(&ek, &mut ek_vec).unwrap();
    ciborium::ser::into_writer(&dk, &mut dk_vec).unwrap();

    (ek_vec, dk_vec)
}

pub fn encrypt_u64(ek: &EncryptionKey, m: u64) -> Vec<u8> {
    let c = Paillier::encrypt(ek, m);
    let mut c_vec = Vec::new();
    ciborium::ser::into_writer(&c, &mut c_vec).unwrap();

    c_vec
}

pub fn decrypt_u64(dk: &DecryptionKey, c: &[u8]) -> u64 {
    let ctext: EncodedCiphertext<u64> = ciborium::de::from_reader(c).unwrap();

    Paillier::decrypt(dk, ctext)
}
