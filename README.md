NEAR Evaluator: Certificate Issuer
==============

> This is a work in progress

This NFT contract is part of the [NEAR Evaluator](https://github.com/flmel/evaluator) project. 
It allows the `evaluator` to mint NFT if the student has covered all the requirements(evaluations).

This contract slightly deviates from the [NEP-171](https://github.com/near/NEPs/blob/master/neps/nep-0171.md) core implementation standard. Since the `nft_transfer` and `nft_transfer_call` are only allowed to the `evaluator` (`owner_id`) account, effectively making the certificates (NFTs) non-transferable.


#### 1. Initializing the contract
```rust
new(owner_id: AccountId, metadata: NFTContractMetadata)
```

#### 2. Minting a new NFT
```rust
nft_mint(&mut self, token_id: TokenId, token_owner_id: AccountId, token_metadata: TokenMetadata)
``````
