// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use composition_example_methods::{COMPOSE_ELF, COMPOSE_ID};
use input_data::prove_data_parsing;
use risc0_zkvm::{default_prover, ExecutorEnv};

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    
    let raw_data = include_str!("../input-data/data/heart+disease/output/signed.cleveland.data.csv");
    let inner_proof = prove_data_parsing(raw_data)?;
    
    // Alice might then send to "Bob" the product and the receipt that proves Alice knows the
    // factorization. Bob then raises a secret number to a public exponent mod the composite number
    // chosen by Alice. This is like an RSA encryption from Bob to Alice, verified by the zkVM.
    let env = ExecutorEnv::builder()
        // add_assumption makes the receipt to be verified available to the prover.
        .add_assumption(inner_proof.receipt)
        .write(&raw_data)?
        .build()?;

    let receipt = default_prover()
        .prove(env, COMPOSE_ELF)?
        .receipt;

    // Anybody who receives the receipt for the exponentiation is assured both that:
    // A) The modulus n included in the journal has a known factorization.
    // B) The number c is the result of exponentiation of some known secret x ^ e mod n.
    //
    // These two statements are proven with a single receipt via composition.
    receipt.verify(COMPOSE_ID).unwrap();
    println!("composition done successfully!");

    Ok(())
}
