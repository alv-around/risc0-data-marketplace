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

use input_data_methods::HEART_DISEASE_DATA_ID;
use risc0_zkvm::{guest::env, serde};

fn main() {
    // adjust for proof inputs
    let data: String = env::read();

    // Verify parsing the csv file has been done correctly
    env::verify(HEART_DISEASE_DATA_ID, &serde::to_vec(&data).unwrap()).unwrap();

    // TODO: add matrix computation and data muching here

    // Commit values
    // env::commit(&(n, e));
}



