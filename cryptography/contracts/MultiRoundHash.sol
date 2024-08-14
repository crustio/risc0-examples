// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be even.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest running on Bonsai.
contract MultiRoundHash {
    event Log(string message);

    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    ///         The image ID is similar to the address of a smart contract.
    ///         It uniquely represents the logic of that guest program,
    ///         ensuring that only proofs generated from a pre-defined guest program
    ///         (in this case, checking if a number is even) are considered valid.
    bytes32 public constant imageId = ImageID.MULTI_ROUND_HASH_ID;

    string public storeString;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        storeString = "";
    }

    /// @notice Set the even number stored on the contract. Requires a RISC Zero proof that the number is even.
    function set(bytes calldata x, bytes calldata seal) public {
        // Construct the expected journal data. Verify will fail if journal does not match.
        verifier.verify(seal, imageId, sha256(x));
        storeString = "123";
    }

    /// @notice Returns the number stored.
    function get() public view returns (string memory) {
        return storeString;
    }
}
