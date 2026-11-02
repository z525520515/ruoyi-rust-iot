use argon2::{Algorithm, Argon2, Params, Version};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;

const DEVICE_UNIQUE_ID_SALT: &[u8] = b"iot-device-unique-id-salt";

pub fn device_unique_id_hash_from_sn(sn: &str) -> Result<String, argon2::Error> {
    let params = Params::new(8192, 2, 1, Some(32))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut output = [0u8; 32];
    argon2.hash_password_into(sn.as_bytes(), DEVICE_UNIQUE_ID_SALT, &mut output)?;

    Ok(BASE64_STANDARD.encode(output))
}
