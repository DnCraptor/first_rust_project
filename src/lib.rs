use std::io::{Read, Write};
use std::path::Path;
use std::fs::File;

pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &Path, bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut file = File::create(path)?;
    file.write(bytes)?;
    Ok(bytes.to_vec())
}

use serde::{Serialize, Deserialize}; // serde = { version = "1.0", features = ["derive"] }
use serde_with::{serde_as, Bytes}; // serde_with = "1.0"

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct SignedMessage {
    #[serde_as(as = "Bytes")]
    pub puk: Vec<u8>,
    pub msg: String,
    #[serde_as(as = "Bytes")]
    pub sig: Vec<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct  SignedMessageDto {
    pub puk: String,
    pub msg: String,
    pub sig: String
}

use ron::{Error, from_str, to_string}; // ron = "0.8"
use ring::signature::UnparsedPublicKey; // ring = "*"
extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, FromBase64, STANDARD};

impl SignedMessage {
    pub fn from_json(str: &String) -> Result<SignedMessage, Error> {
        let sig: SignedMessage = from_str(str)?;
        Ok(sig)
    }
    pub fn to_json(self: &SignedMessage) -> Result<String, Error> {
        to_string(&self)
    }
    pub fn verification(self: &SignedMessage) -> Result<(), Error> {
        let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, &self.puk );
        public_key.verify(&self.msg.as_bytes(), self.sig.as_ref()).unwrap();
        Ok(())
    }
    pub fn to_dto(self: &SignedMessage) -> SignedMessageDto {
        SignedMessageDto {
            puk: self.puk.to_base64(STANDARD),
            msg: self.msg.clone(),
            sig: self.sig.to_base64(STANDARD)
        }
    }
}

impl SignedMessageDto {
    pub fn to_native(self: &SignedMessageDto) -> SignedMessage {
        SignedMessage {
            puk: self.puk.from_base64().unwrap(),
            msg: self.msg.clone(),
            sig: self.sig.from_base64().unwrap()
        }
    }
}