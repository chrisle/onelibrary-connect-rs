//! Source-compatible OneLibrary derivation. Never log key material.
use std::io::Read;
pub fn get_encryption_key() -> crate::Result<Vec<u8>> {
    let blob =
        b"PN_1dH8$oLJY)16j_RvM6qphWw`476>;C1cWmI#se(PG`j}~xAjlufj?`#0i{;=glh(SkW)y0>n?YEiD`l%t(";
    let xor = b"657f48f84c437cc1";
    let alphabet =
        b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";
    let mut decoded = Vec::new();
    for chunk in blob.chunks(5) {
        let mut n = 0_u64;
        for c in chunk {
            n = n * 85
                + alphabet
                    .iter()
                    .position(|x| x == c)
                    .ok_or(crate::Error::InvalidKeyDerivation)? as u64;
        }
        decoded.extend_from_slice(&(n as u32).to_be_bytes()[..chunk.len() - 1]);
    }
    for (i, b) in decoded.iter_mut().enumerate() {
        *b ^= xor[i % xor.len()];
    }
    let mut key = Vec::new();
    flate2::read::ZlibDecoder::new(decoded.as_slice())
        .take(4096)
        .read_to_end(&mut key)
        .map_err(|_| crate::Error::InvalidKeyDerivation)?;
    if !key.starts_with(b"r8gd") {
        return Err(crate::Error::InvalidKeyDerivation);
    }
    Ok(key)
}
