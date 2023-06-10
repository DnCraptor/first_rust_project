// + Cargo.toml
use ring::rand::{SystemRandom}; // ring = "*"
use ring::signature::{KeyPair, Ed25519KeyPair, UnparsedPublicKey}; // ring = "*"
use serde::{Serialize, Deserialize}; // serde = { version = "1.0", features = ["derive"] }
use serde_with::{serde_as, Bytes}; // serde_with = "1.0"

// TODO: use as shared mod?
pub mod files {
    use std::io::{Read, Write};

    pub fn read_file(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
        let mut file = std::fs::File::open(path)?;
        let mut contents: Vec<u8> = Vec::new();
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }
    
    pub fn write_file(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut file = std::fs::File::create(path)?;
        file.write(bytes)?;
        Ok(bytes.to_vec())
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct SignedMessage {
    #[serde_as(as = "Bytes")]
    puk: Vec<u8>,
    msg: String,
    #[serde_as(as = "Bytes")]
    sig: Vec<u8>
}

pub fn init() -> Result<Ed25519KeyPair, std::io::Error> {
    let path = "VecU8.key";
    let f = match files::read_file(std::path::Path::new(path)) {
        Ok(r) => { r },
        Err(_) => {
            println!("'{}' not found. New random generation...", path);
            let rng = SystemRandom::new();
            let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
            let res = files::write_file(std::path::Path::new(path), pkcs8_bytes.as_ref()).unwrap();
            res
        }
    };
    Ok(Ed25519KeyPair::from_pkcs8(f.as_ref()).unwrap())
}

fn server() -> Result<String, ron::Error> {
    let key_pair = init().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();
    let msg = "Hello, world!";
    let message = msg.as_bytes();
    let signature = key_pair.sign(&message);
    let signature_bytes = signature.as_ref();
    let sig = SignedMessage {
        puk: public_key_bytes.to_vec(),
        msg: String::from(msg),
        sig: signature_bytes.to_vec()
    };
    ron::to_string(&sig)
}

fn client(serialized: &String) {
    let sig: SignedMessage = ron::from_str(&serialized).unwrap();
    let public_key = UnparsedPublicKey::new(&ring::signature::ED25519, sig.puk );
    public_key.verify(&sig.msg.as_bytes(), sig.sig.as_ref()).unwrap();
    println!("verification passed for {}", serialized);
}

pub fn test() {
    let serialized = server().unwrap();
    println!("serialized = {}", serialized);
    client(&serialized);
}
