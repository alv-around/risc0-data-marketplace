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

use k256::ecdsa::Signature;

use risc0_ecdsa_methods::SIGNATURE_ID;
use risc0_zkvm::{guest::env, serde};

fn main() {
    // change guest code to accept the outerproof inputs
    let (message, sig): (Vec<u8>, Signature) = env::read();

    // Verify that n has a known factorization.
    env::verify(SIGNATURE_ID, &serde::to_vec(&(message, sig)).unwrap()).unwrap();

    // Commit n, e, and x^e mod n.
    //env::commit(&(n, e, pow_mod(x, e, n)));
}

