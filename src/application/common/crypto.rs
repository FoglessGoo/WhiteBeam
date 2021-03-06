#[cfg(target_os = "windows")]
use crate::platforms::windows as platform;
#[cfg(target_os = "linux")]
use crate::platforms::linux as platform;
#[cfg(target_os = "macos")]
use crate::platforms::macos as platform;
use crate::common::db;
use crate::common::time;
use serde::{Serialize, Deserialize};
#[allow(unused_imports)]
use serde_json::json;
#[allow(unused_imports)]
use std::{error::Error,
          io::prelude::*,
          io::Read,
          io::Write as IoWrite,
          path::Path,
          fmt::Write as FmtWrite,
          num::ParseIntError};
use sodiumoxide::crypto::{box_,
                          box_::curve25519xsalsa20poly1305::*};

// TODO: Log errors

/*
Keys
*/

fn key_array_from_slice(bytes: &[u8]) -> [u8; SECRETKEYBYTES] {
    let mut array = [0; SECRETKEYBYTES];
    let bytes = &bytes[..array.len()]; // Panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn generate_client_private_key(save_path: &Path) -> Result<(), std::io::Error> {
    let (_public_key, private_key) = box_::gen_keypair();
    let private_key_bytes: &[u8] = private_key.as_ref();
    let mut key_file = platform::path_open_secure(save_path);
    Ok(key_file.write_all(private_key_bytes)?)
}

fn get_server_public_key() -> Result<PublicKey, Box<dyn Error>> {
    let conn: rusqlite::Connection = db::db_open();
    let public_key_string: String = db::get_config(&conn, String::from("server_key"));
    let public_key_bytes: &[u8] = &hex::decode(&public_key_string)?;
    match PublicKey::from_slice(public_key_bytes) {
        Some(public_key) => Ok(public_key),
        None => Err("Invalid server public key".into())
    }
}

fn get_client_public_private_key() -> Result<(PublicKey, SecretKey), Box<dyn Error>> {
    let key_file_path: &Path = &platform::get_data_file_path("client.key");
    let gen_key: bool = !key_file_path.exists();
    if gen_key {
        generate_client_private_key(key_file_path)?;
    }
    let mut key_file = std::fs::File::open(key_file_path)?;
    let mut private_key_bytes: Vec<u8> = Vec::new();
    key_file.read_to_end(&mut private_key_bytes)?;
    let private_key_array: [u8; SECRETKEYBYTES] = key_array_from_slice(&private_key_bytes);
    let private_key = SecretKey(private_key_array);
    let public_key = private_key.public_key();
    Ok((public_key, private_key))
}

/*
Encoding
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub expires: u32,
    pub version: String,
    pub action: String,
    pub parameters: Vec<String>
}

fn json_encode_message(action: String, parameters: Vec<String>) -> Result<String, serde_json::Error> {
    let expires = time::get_timestamp()+3600;
    let version = env!("CARGO_PKG_VERSION").to_string();
    let message_object = Message {
        expires: expires,
        version: version,
        action: action,
        parameters: parameters
    };
    Ok(serde_json::to_string(&message_object)?)
}

fn json_decode_message(json: String) -> Result<Message, serde_json::Error> {
    let message_object: Message = serde_json::from_str(&json)?;
    Ok(message_object)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CryptoBox {
    pub pubkey: String,
    pub nonce: String,
    pub ciphertext: String
}

fn json_encode_crypto_box(pubkey: String, nonce: String, ciphertext: String) -> Result<String, serde_json::Error> {
    let crypto_box_object = CryptoBox {
        pubkey: pubkey,
        nonce: nonce,
        ciphertext: ciphertext
    };
    Ok(serde_json::to_string(&crypto_box_object)?)
}

#[allow(dead_code)]
fn json_decode_crypto_box(json: String) -> Result<CryptoBox, serde_json::Error> {
    let crypto_box_object: CryptoBox = serde_json::from_str(&json)?;
    Ok(crypto_box_object)
}

fn nonce_array_from_slice(bytes: &[u8]) -> Result<[u8; NONCEBYTES], String> {
    if bytes.len() != NONCEBYTES {
        return Err("Invalid nonce".into());
    }
    let mut array = [0; NONCEBYTES];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    Ok(array)
}

/*
Encryption
*/

fn generate_ciphertext(plaintext: &[u8], nonce: Nonce) -> Result<Vec<u8>, Box<dyn Error>> {
    let (_client_public_key, client_private_key) = get_client_public_private_key()?;
    let server_public_key = get_server_public_key()?;
    Ok(box_::seal(plaintext, &nonce, &server_public_key, &client_private_key))
}

fn decrypt_server_ciphertext(ciphertext: &[u8], nonce: Nonce) -> Result<Vec<u8>, Box<dyn Error>> {
    let (_client_public_key, client_private_key) = get_client_public_private_key()?;
    let server_public_key = get_server_public_key()?;
    // Verification and decryption
    match box_::open(ciphertext, &nonce, &server_public_key, &client_private_key) {
        Ok(plaintext) => Ok(plaintext),
        Err(_e) => return Err("Invalid ciphertext".into())
    }
}

/*
Handlers
*/

pub fn get_client_public_key() -> Result<PublicKey, Box<dyn Error>> {
    let (client_public_key, _client_private_key) = get_client_public_private_key()?;
    Ok(client_public_key)
}

pub fn generate_crypto_box_message(action: String, parameters: Vec<String>) -> Result<String, Box<dyn Error>> {
    let (client_public_key, _client_private_key) = get_client_public_private_key()?;
    let message = json_encode_message(action, parameters)?;
    let nonce = box_::gen_nonce();
    let ciphertext: Vec<u8> = generate_ciphertext(message.as_bytes(), nonce)?;
    Ok(json_encode_crypto_box(hex::encode(client_public_key), hex::encode(nonce), hex::encode(ciphertext))?)
}

pub fn process_request(crypto_box_object: CryptoBox) -> Result<Message, Box<dyn Error>> {
    let conn: rusqlite::Connection = db::db_open();
    // Ignore replayed messages
    if db::get_seen_nonce(&conn, &crypto_box_object.nonce) {
        return Err("Invalid message".into());
    }
    let nonce_array: [u8; NONCEBYTES] = nonce_array_from_slice(&hex::decode(crypto_box_object.nonce)?)?;
    let nonce = Nonce(nonce_array);
    let plaintext: String = String::from(
        std::str::from_utf8(
            &decrypt_server_ciphertext(
                &hex::decode(&crypto_box_object.ciphertext)?,
                nonce
            )?
        )?
    );
    let server_message = json_decode_message(plaintext)?;
    let expiry = time::get_timestamp();
    if server_message.expires < expiry {
        // Message has expired
        return Err("Invalid message".into());
    }
    Ok(server_message)
}
