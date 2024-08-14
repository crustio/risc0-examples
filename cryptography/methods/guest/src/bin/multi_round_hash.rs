// Copyright 2023 RISC Zero, Inc.
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

use std::io::Read;

use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};

fn main() {
    // Read the input data for this application.
    let mut input_data = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_data).unwrap();
    let mut hasher = Sha256::new();
    let mut data = input_data.to_vec();
    for _ in 0..10000 {
        hasher.update(&data);
        data = hasher.finalize_reset().to_vec();
    }
    let hash = hex::encode(data);
    env::commit_slice(&hash.abi_encode().as_slice());
}
