- mod.rs `address()` function 
  Does not provide the bech32::Variant enum anymore.
  I used `bech32::Bech32` enum instead which follow bip-0173
  Requires bech32::Hrp instance to be passed to encode function. 

- bech32 does not provide convert_bits function anymore.
- cmp-manager?
- serde_yaml is deprecated i used it's last stable release 0.9.33
- Updated secp256k1 dependency in Cargo.toml:
  - Upgraded from version 0.24.0 to 0.29.1
  - Added "global-context" and "recovery" features
  - This update provides improved performance and additional functionality for ECDSA operations
-