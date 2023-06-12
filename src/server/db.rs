// + Cargo.toml
use persy::{Persy, Config, ValueMode, ByteVec, PersyId};
use std::{path::Path};

#[derive(Clone)]
pub struct KVDB {
    store: Persy
}

fn init() -> Persy {
    let path = Path::new("./storage.persy");
    let config = Config::new();
    Persy::open_or_create_with(path, config, |persy| {
        // this closure is only called on database creation
        let mut tx = persy.begin()?;
        tx.create_segment("seg")?;
        tx.create_index::<ByteVec, PersyId>("pk", ValueMode::Replace)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        Ok(())
    }).unwrap()
}

impl KVDB {
    pub fn new() -> KVDB {
       KVDB { store: init() }
    }
    pub fn upsert(self: &Self, item: &lib::SignedMessage) {
        let pk = item.puk.clone();
        let json = item.to_json().unwrap();
        let json_bytes = json.into_bytes();
        let mut tx = self.store.begin().unwrap();
        let id = tx.insert("seg", &json_bytes).unwrap();
        tx.put("pk", ByteVec::new(pk), id).unwrap();
        let prepared = tx.prepare().unwrap();
        prepared.commit().unwrap();
        dbg!(id);
    }
    pub fn get(self: &Self, puk: &Vec<u8>) -> Option<lib::SignedMessage> {
        let pk = ByteVec::new(puk.clone());
        match self.store.one::<ByteVec, PersyId>("pk", &pk) {
            Err(_) => return None,
            Ok(by_idx) => {
                if let Some(id) = by_idx {
                    match self.store.read("seg", &id) {
                        Err(_) => return None,
                        Ok(val) => {
                            if let Some(vec_u8) = val {
                                let str = String::from_utf8_lossy(&vec_u8).to_string();
                                let sig = lib::SignedMessage::from_json(&str).unwrap();
                                return Some(sig);
                            }
                        }
                    }
                }
            }
        }
        None
    }
    pub fn get_all(self: &Self) -> Vec<lib::SignedMessage> {
        let mut r = Vec::new();
        let scan_res = self.store.scan("seg");
        for (_id, content) in scan_res.unwrap() {
            dbg!("{}! -> {}!", _id, &content);
            let str = String::from_utf8_lossy(&content).to_string();
            dbg!("{}! -> {}!", _id, &str);
            let sig = lib::SignedMessage::from_json(&str).unwrap();
            r.push(sig);
        }
        r
    }
}
