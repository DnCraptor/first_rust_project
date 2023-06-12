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

use ron::{Error, from_str, to_string}; // ron = "0.8"
use ring::signature::UnparsedPublicKey; // ring = "*"

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
}
