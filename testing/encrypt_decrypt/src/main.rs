use magic_crypt::{new_magic_crypt, MagicCryptTrait};


fn main() {
    
    let mc = new_magic_crypt!("magickey", 256);

    let encrypted = mc.encrypt_str_to_base64("Hi this secret message is written by ajay kumar");

    println!("{encrypted}");

    let decrypted = mc.decrypt_base64_to_string(encrypted).unwrap();

    println!("{decrypted}");

}
