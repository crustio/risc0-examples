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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {MultiRoundHash} from "../contracts/MultiRoundHash.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.

contract MultiRoundHashTest is RiscZeroCheats, Test {
    event Log(string message);
    MultiRoundHash public roundHash;

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        roundHash = new MultiRoundHash(verifier);
        assertEq(roundHash.get(), "");
    }

    function test_SetCorrectHash() public {
        string memory str = "123";
        (bytes memory journal, bytes memory seal) = prove(
            Elf.MULTI_ROUND_HASH_PATH,
            abi.encode(str)
        );
        string memory result = abi.decode(journal, (string));
        console2.log("result", result);
        roundHash.set(result, seal);
        string memory finalStr = roundHash.get();
        assertEq(finalStr, result);
    }
}
