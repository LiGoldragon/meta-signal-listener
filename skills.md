# skills - meta-signal-listener

Before editing this repo, read the contract-repo, Rust, Nix, and testing
discipline named by the primary workspace. This repo owns only Listener's
owner/meta configuration wire vocabulary.

Changes to `schema/lib.schema` require regenerating `src/schema/lib.rs`, then
running `nix flake check`.
