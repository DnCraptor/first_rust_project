// + Cargo.toml
use ring::rand::SystemRandom; // ring = "*"
use ring::signature::{KeyPair, Ed25519KeyPair}; // ring = "*"
use ron::Error; // ron = "0.8"

pub fn init() -> Result<Ed25519KeyPair, std::io::Error> {
    let path = "VecU8.key";
    let f = match lib::read_file(std::path::Path::new(path)) {
        Ok(r) => { r },
        Err(_) => {
            println!("'{}' not found. New random generation...", path);
            let rng = SystemRandom::new();
            let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
            let res = lib::write_file(std::path::Path::new(path), pkcs8_bytes.as_ref()).unwrap();
            res
        }
    };
    Ok(Ed25519KeyPair::from_pkcs8(f.as_ref()).unwrap())
}

fn server() -> Result<String, Error> {
    let key_pair = init().unwrap();
    let public_key_bytes = key_pair.public_key().as_ref();
    let msg = "Hello, world!";
    let message = msg.as_bytes();
    let signature = key_pair.sign(&message);
    let signature_bytes = signature.as_ref();
    let sig = lib::SignedMessage {
        puk: public_key_bytes.to_vec(),
        msg: String::from(msg),
        sig: signature_bytes.to_vec()
    };
    sig.to_json()
}

fn client(serialized: &String) {
    let sig: lib::SignedMessage = lib::SignedMessage::from_json(&serialized).unwrap();
    sig.verification().unwrap();
    println!("verification passed for {}", serialized);
}

pub fn test() {
    let serialized = server().unwrap();
    println!("serialized = {}", serialized);
    client(&serialized);
}
