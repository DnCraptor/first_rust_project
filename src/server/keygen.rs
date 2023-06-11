pub fn new_cert_and_pair(cert_path: &str, ppk_path: &str, puk_path: &str) {
    extern crate rcgen; // rcgen = "*"
    use rcgen::generate_simple_self_signed;
    let subject_alt_names = vec!["self signed cert".to_string(), "any".to_string()];
    let cert = generate_simple_self_signed(subject_alt_names).unwrap();
    let cert_pem = cert.serialize_pem().unwrap();
    lib::write_file(std::path::Path::new(cert_path), cert_pem.as_bytes()).unwrap();
    println!("{}", cert_pem);
    let ppk = cert.serialize_private_key_pem();
    lib::write_file(std::path::Path::new(ppk_path), ppk.as_bytes()).unwrap();
    println!("{}", ppk);
    let puk = cert.get_key_pair().public_key_pem();
    lib::write_file(std::path::Path::new(puk_path), puk.as_bytes()).unwrap();
    println!("{}", puk);
    ()
}
