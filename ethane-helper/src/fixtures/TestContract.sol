// SPDX-License-Identifier: MIT
pragma solidity ^0.8;

contract TestContract {
    uint pos0;

    event Solution(uint indexed _num);

    constructor() {
        pos0 = 11;
    }

    function solution() public view returns (uint) {
        return 42;
    }

    function set_pos0() public {
        pos0 = 2;
        emit Solution(2);
    }
}
