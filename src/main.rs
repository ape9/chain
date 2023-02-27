use core::fmt;

use sha256::{digest};

fn create_hash(input: String) -> String {
    digest(input)
}

struct Proof {
    nonce: i32,
    result_hash: String
}

fn find_proof(prev_hash: String, difficulty: usize) -> Proof {
    let mut found_proof: bool = false;
    let mut nonce = 0;
    let mut result_hash = String::new();
    
    loop {
        let input_str = format!("{}{}", prev_hash, nonce);
        let hash = create_hash(input_str);
        println!("Hash: {}", hash);
        if hash.as_str().starts_with(&"0".repeat(difficulty)) {
            result_hash = hash;
            break;
        }

        nonce += 1;
    }

    Proof {
        nonce,
        result_hash
    }
}

fn main() {
    let hash = digest(b"suomi");
    let proof = find_proof(hash, 4);
    println!("Successfully mined {}", proof.result_hash);
    println!("Nonce: {}", proof.nonce);
}
