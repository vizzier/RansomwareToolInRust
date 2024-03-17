use std::fs;
use std::path::PathBuf;
use std::path::Path;
use encryptor::keystuff;
use crate::encryptor;
use crate::encryptor::decryptor;
//bool true for enc and false for dec
pub fn traverse_directory(path: &Path, flag : bool, ks: &keystuff) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut result = Vec::new();

    if path.is_dir() {
        let entries = fs::read_dir(path)?; 

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // If it's a directory, recursively traverse it
                let mut subdir_files = traverse_directory(&entry_path, true,ks)?;
                result.append(&mut subdir_files);
            } else {
                // If it's a file, add it to the result vector
                if flag {
                let mut ks = encryptor::tkd(&entry_path, &ks);}
                else {
                decryptor(&entry_path, &ks);
                }
            }
        }
    }

    Ok(result)
}