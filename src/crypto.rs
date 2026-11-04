use getrandom::{SysRng, rand_core::TryRng};
use sha2::{Digest, Sha512};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::debug;

pub fn keyphrase_gen() -> Result<String, String> {
    let mut depth = 0;
    let mut success = false;
    let mut ints: [u64; 4] = [0, 0, 0, 0];

    while !success {
        success = true;
        ints = four_ints(random_512());

        for num in ints {
            if num > 2047 {
                success = false;
            }
        }

        depth += 1;

        if depth > 4 {
            return Err(String::from("Invalid Integers Generated"));
        }
    }

    let mut failed = false;
    let mut keyphrase = String::from("");

    for num in ints {
        // is fine to truncate as ints[i] is 11 bits
        let n = num as usize;
        debug!("n is: {}", num);

        let file = match File::open("./bip39/english.txt") {
            Ok(fl) => fl,
            Err(_e) => return Err(String::from("Bip39 language file not found")),
        };
        let reader = BufReader::new(&file);

        match reader.lines().nth(n) {
            Some(line) => match line {
                Ok(line) => {
                    debug!("word is: {}", line);

                    if !keyphrase.is_empty() {
                        keyphrase = keyphrase + "-" + &line;
                    } else {
                        keyphrase = keyphrase + &line;
                    }
                }
                Err(_e) => failed = true,
            },
            None => failed = true,
        }

        if failed {
            return Err(String::from("Line not found in file"));
        }
    }

    Ok(keyphrase)
}

// Generates a 512 bit random hash.
//
// Does this by first getting a SysRng 12 bit random hash and then hashing it
// to 512 bits using SHA512.
//
// # Returns
//
// A string containing the hex encoded hash.
fn random_512() -> String {
    let mut key = [0u8; 16];
    SysRng.try_fill_bytes(&mut key).unwrap();
    debug!("Random Key is: {}", hex::encode(key));

    let hash512 = hex::encode(Sha512::digest(key));
    debug!("Hash is {}", hash512);

    hash512
}

// Takes a 512 bit hex string and creates 4 ints from the first 11 characters
//
// # Returns
//
// [u64; 4]
fn four_ints(hash512: String) -> [u64; 4] {
    let hash44 = hash512.get(0..11).unwrap();
    debug!("44 Hex is {}", hash44);

    let big_int = u64::from_str_radix(hash44, 16).expect("Invalid hex char");

    let mask = 0x7ff;

    let int_0 = big_int & mask;
    let int_1 = (big_int >> 11) & mask;
    let int_2 = (big_int >> 22) & mask;
    let int_3 = (big_int >> 33) & mask;
    debug!("Ints are: {} {} {} {}", int_0, int_1, int_2, int_3);

    [int_0, int_1, int_2, int_3]
}
