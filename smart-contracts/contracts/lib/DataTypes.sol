// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

library DataTypes {
    struct CreateProfileData {
        address to;
        string handle;
        string imageURI;
        string name;
    }

    struct UpgradeToPublisherData{
        uint256 tokenID;
    }
}