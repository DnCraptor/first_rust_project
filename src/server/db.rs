// + Cargo.toml
use persy::{Persy, Config}; // persy = "*"
use std::path::Path;

pub fn init() -> Persy {
    let path = Path::new("./storage.persy");
    let config = Config::new();
    Persy::open_or_create_with(path, config, |persy| {
        // this closure is only called on database creation
        let mut tx = persy.begin()?;
        tx.create_segment("seg")?;
//        tx.create_index::<u64, PersyId>("index", ValueMode::Replace)?;
        let prepared = tx.prepare()?;
        prepared.commit()?;
        Ok(())
    }).unwrap()
}
