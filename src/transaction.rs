use serde::{Serialize, Deserialize}

#[derive(Debug, Serialize, Deserialize)]
struct TxInput {
    previous: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TxOutput {
    signature: String,
    amount: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct Tx {
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>
}

impl Tx {
    fn new(inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            inputs,
            outputs
        }
    }

}