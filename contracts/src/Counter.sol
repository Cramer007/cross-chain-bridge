// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Counter {
    uint256 public number;

    function setNumber(uint256 newNumber) public {
        number = newNumber;
    }

    function increment() public {
        number++;
    }
}


// Sepolia pas holesky Bridge Contract: 0xE2e4eC7863Ee9828D3A1F83EDa42839aCaE61DCe
// (holesky)Target Chain Bridge Contract: 0x5f1045AF1fd26F77224Ac3acCa1B9A44c6e7aF6d