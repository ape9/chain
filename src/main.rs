use core::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use sha256::{digest};

fn create_hash(input: String) -> String {
    digest(input)
}

struct Proof {
    nonce: i32,
    result_hash: String,
    timestamp: u64
}

fn find_proof(prev_hash: String, difficulty: usize) -> Proof {
    let mut nonce = 0;
    let mut result_hash = String::new();
    let mut timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    loop {
        let input_str = format!("{}{}{}", prev_hash, nonce, timestamp);
        let hash = create_hash(input_str);
        
        if hash.as_str().starts_with(&"0".repeat(difficulty)) {
            // OK, solution was found.
            result_hash = hash;
            break;
        }

        if nonce == i32::MAX {
            // reset nonce and get a new timestamp when max size of i32 is reached
            // to continue searching
            nonce = 0;
            timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        nonce += 1;
    }

    Proof {
        nonce,
        result_hash,
        timestamp
    }
}


fn main() {
    let hash = digest(b"suomi");
    let proof = find_proof(hash, 5);
    println!("Successfully mined {}", proof.result_hash);
    println!("Nonce: {}", proof.nonce);
    println!("Timestamp: {}", proof.timestamp);

}
