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

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;
    use alloy_sol_types::SolValue;
    use risc0_zkvm::{default_executor, default_prover, ExecutorEnv};
    use sha2::{Digest, Sha256};

    #[test]
    fn proves_even_number() {
        let even_number = U256::from(1304);

        let env = ExecutorEnv::builder()
            .write_slice(&even_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor().execute(env, super::IS_EVEN_ELF).unwrap();

        let x = U256::abi_decode(&session_info.journal.bytes, true).unwrap();
        assert_eq!(x, even_number);
    }

    #[test]
    #[should_panic(expected = "number is not even")]
    fn rejects_odd_number() {
        let odd_number = U256::from(75);

        let env = ExecutorEnv::builder()
            .write_slice(&odd_number.abi_encode())
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        default_executor().execute(env, super::IS_EVEN_ELF).unwrap();
    }

    #[test]
    fn proves_plus() {
        let a = 1;
        let env = ExecutorEnv::builder().write(&a).unwrap().build().unwrap();
        let receipt = default_executor().execute(env, super::PLUS_ELF).unwrap();

        let result = U256::abi_decode(&receipt.journal.bytes, true).unwrap();
        println!("Resulting number: {}", &result);
        let expected = U256::from(a) + U256::from(1);
        assert_eq!(expected, result);
    }

    #[test]
    fn proves_hash() {
        let base_str = "test";
        let hash = multi_round_hash(&base_str.abi_encode(), 10000);
        let write_data = &base_str.abi_encode();
        println!(
            "base str: {}, Resulting hash: {} write_data_len: {}",
            &base_str,
            hash,
            write_data.len()
        );

        let env = ExecutorEnv::builder()
            .write_slice(write_data)
            .build()
            .unwrap();

        let receipt = default_executor()
            .execute(env, super::MULTI_ROUND_HASH_ELF)
            .unwrap();
        let r = String::abi_decode(&receipt.journal.bytes, true).unwrap();
        println!("Resulting string: {}", &r);
        assert_eq!(r, hash);
    }

    fn multi_round_hash(input: &[u8], rounds: u32) -> String {
        let mut hasher = Sha256::new();
        let mut data = input.to_vec();

        for _ in 0..rounds {
            hasher.update(&data);
            data = hasher.finalize_reset().to_vec();
        }

        hex::encode(data)
    }
}
