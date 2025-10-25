//! Define Auth Data module
//!
//! - Item: [AuthData]
//! - ApiDoc: <https://apidoc.whatsminer.com/#api-Token-generate_token>
use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit};
use base64_light::base64_encode_bytes;
#[allow(deprecated)]
use cipher::generic_array::GenericArray;
use serde::Serialize;
use sha256::digest;
use std::fmt::{Debug, Display};
use std::time::{SystemTime, UNIX_EPOCH};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::account::Account;
use crate::actor::Actor;
use crate::command::Command;
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Zeroize, ZeroizeOnDrop)]
/// Data for protected endpoints
///
/// It is a unique peer command
pub struct AuthData {
    /// time when pushed
    pub ts: u64,
    /// Auth miner token
    ///
    /// It generated from 'command'+'account's password'+'salt'+'ts'
    ///
    /// - ApiDoc: <https://apidoc.whatsminer.com/#api-Token-generate_token>
    token: String,
    /// Account username
    #[zeroize(skip)]
    #[serde(rename = "account")]
    username: Account,
    // sha256 hex data for encrypting params (AES key)
    #[serde(skip)]
    aes_key: Vec<u8>,
}

impl Display for AuthData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthData")
            .field("ts", &self.ts)
            .field("username", &self.username)
            .field("token", &"<hidden>")
            .field("aes_key", &"<hidden>")
            .finish()
    }
}

impl<'a> AuthData {
    /// Generate auth data
    pub fn new<C: Command>(
        username: Account,
        password: impl AsRef<str>,
        salt: &'a str,
    ) -> Result<Self> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let ts = since_the_epoch.as_secs();

        // generate sha256 hex from data and convert it to base64
        //
        // - ApiDoc:
        // https://apidoc.whatsminer.com/#api-Token-generate_token
        let input_to_hash = format!("{}{}{}{}", C::CMD_NAME, password.as_ref(), salt, ts);
        let sha256_hex_digest = digest(input_to_hash);

        let sha256_raw_bytes = hex::decode(&sha256_hex_digest)?;
        let token_base64_full = base64_encode_bytes(&sha256_raw_bytes);
        let token: String = token_base64_full.chars().take(8).collect();

        Ok(Self {
            ts,
            token,
            username,
            aes_key: sha256_raw_bytes,
        })
    }

    pub fn from_actor<T: Command>(actor: &Actor) -> Result<Self> {
        Self::new::<T>(actor.username, actor.password.as_ref(), &actor.salt)
    }

    /// Encrypt data using AES-256-ECB
    ///
    /// According to WhatsMiner API documentation, params are encrypted using:
    /// - AES-256 in ECB mode (no IV needed)
    /// - PKCS7 padding
    /// - The SHA256 hash as the encryption key
    pub fn encrypt(&self, data: impl AsRef<[u8]>) -> Result<String> {
        let plaintext = data.as_ref();

        // Create AES-256 cipher instance
        let cipher = Aes256::new_from_slice(&self.aes_key).map_err(|_| Error::EncryptionFailed)?;

        // Add PKCS7 padding manually
        let block_size = 16;
        let padding_len = block_size - (plaintext.len() % block_size);
        let total_len = plaintext.len() + padding_len;

        let mut buffer = vec![0u8; total_len];
        buffer[..plaintext.len()].copy_from_slice(plaintext);

        // Fill padding bytes with padding length value (PKCS7)
        for i in buffer.iter_mut().take(total_len).skip(plaintext.len()) {
            *i = padding_len as u8;
        }

        // Encrypt each 16-byte block independently (ECB mode)
        for chunk in buffer.chunks_mut(block_size) {
            if chunk.len() == block_size {
                #[allow(deprecated)]
                let mut block = *GenericArray::from_slice(chunk);
                cipher.encrypt_block(&mut block);
                chunk.copy_from_slice(&block);
            }
        }

        // Encode to base64
        Ok(base64_encode_bytes(&buffer))
    }
}
