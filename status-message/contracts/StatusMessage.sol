// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract StatusMessage {
    event SetStatus(string indexed key, string indexed message);

    /// Status message per user. User can be an Aurora address or NEAR Account Id
    mapping(string => string) public status;

    /// Set of all users registered in the system.
    mapping(string => bool) public seen;
    /// List of all users registered in the system. Useful for enumeration.
    string[] public allKeys;
    /// Implicit address for NEAR Proxy. Only NEAR Proxy can call `setNearAccountIdStatus`
    address public nearProxy;

    constructor(address _nearProxy) {
        nearProxy = _nearProxy;
    }

    /// Internal method. Set status (value) for a user (key).
    function _setStatus(string memory key, string memory value) private {
        status[key] = value;
        if (!seen[key]) {
            allKeys.push(key);
            seen[key] = true;
        }
        emit SetStatus(key, value);
    }

    /// Set status for the signer of this transaction.
    function setAuroraAddressStatus(string memory _value) public {
        string memory _key = toString(abi.encodePacked(msg.sender));
        _setStatus(_key, _value);
    }

    /// Set status for specific user. This function can only be called by NEAR Proxy.
    function setNearAccountIdStatus(string calldata key, string memory value)
        public
    {
        require(
            msg.sender == nearProxy,
            "Only the NEAR proxy can set the status"
        );

        _setStatus(key, value);
    }

    /// Get status for current user (key)
    function getStatus(string memory key) public view returns (string memory) {
        return status[key];
    }

    function getKey(uint256 index) public view returns (string memory) {
        return allKeys[index];
    }

    function totalKeys() public view returns (uint256) {
        return allKeys.length;
    }

    // Convert a byte array to a hex string.
    // https://ethereum.stackexchange.com/a/58341/45323
    function toString(bytes memory data) public pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";

        bytes memory str = new bytes(2 + data.length * 2);
        str[0] = "0";
        str[1] = "x";
        for (uint256 i = 0; i < data.length; i++) {
            str[2 + i * 2] = alphabet[uint256(uint8(data[i] >> 4))];
            str[3 + i * 2] = alphabet[uint256(uint8(data[i] & 0x0f))];
        }
        return string(str);
    }
}
