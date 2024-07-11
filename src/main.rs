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
use risc0_ecdsa::prove_ecdsa_verification;
use risc0_zkvm::{default_prover, ExecutorEnv};

use rand_core::OsRng;

fn main() {
    // Some party "Alice" picks two numbers and multiplies them, producing a receipt that attests
    // the fact that Alice knows the factorization of the product. This is similar to RSA keygen.
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"This is a message that will be signed, and verified within the zkVM";
    let signature: Signature = signing_key.sign(message);

    // Run signature verified in the zkVM guest and get the resulting receipt.
    let signature_receipt = prove_ecdsa_verification(signing_key.verifying_key(), message, &signature);

    // Alice might then send to "Bob" the product and the receipt that proves Alice knows the
    // factorization. Bob then raises a secret number to a public exponent mod the composite number
    // chosen by Alice. This is like an RSA encryption from Bob to Alice, verified by the zkVM.
    let env = ExecutorEnv::builder()
        // add_assumption makes the receipt to be verified available to the prover.
        .add_assumption(multiply_receipt)
        // add inputs of outer proof
        .write(&(message, signature)) 
        .unwrap()
        .build()
        .unwrap();

    let receipt = default_prover().prove(env, COMPOSE_ELF).unwrap();

    // Anybody who receives the receipt for the exponentiation is assured both that:
    // A) The modulus n included in the journal has a known factorization.
    // B) The number c is the result of exponentiation of some known secret x ^ e mod n.
    //
    // These two statements are proven with a single receipt via composition.
    receipt.verify(COMPOSE_ID).unwrap();

    // Decode the receipt to get (n, e, and c = x^e mod n).
    // let (n, e, c): (u64, u64, u64) = receipt.journal.decode().unwrap();

    // println!("{c} is the result of exponentiation by {e} under composite {n} with known factors");
}
