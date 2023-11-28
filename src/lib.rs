use near_contract_standards::non_fungible_token::core::{
    NonFungibleTokenCore, NonFungibleTokenResolver,
};

use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata,
};

use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{events::NftMint, Token, TokenId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, require, AccountId, BorshStorageKey, PanicOnDefault, PromiseOrValue,
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        require!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                None::<StorageKey>,
                None::<StorageKey>,
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
        }
    }

    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Unauthorized"
        );

        let token = self.tokens.internal_mint(
            token_id.clone(),
            token_owner_id.clone(),
            Some(token_metadata),
        );

        NftMint::emit(NftMint {
            token_ids: &[&token_id],
            owner_id: &token_owner_id,
            memo: None,
        });

        return token;
    }
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Unauthorized"
        );
        self.tokens
            .nft_transfer(receiver_id, token_id, approval_id, memo);
    }

    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        assert_eq!(
            env::predecessor_account_id(),
            self.tokens.owner_id,
            "Unauthorized"
        );
        self.tokens
            .nft_transfer_call(receiver_id, token_id, approval_id, memo, msg)
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }
}

#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        previous_owner_id: AccountId,
        receiver_id: AccountId,
        token_id: TokenId,
        approved_account_ids: Option<HashMap<AccountId, u64>>,
    ) -> bool {
        self.tokens.nft_resolve_transfer(
            previous_owner_id,
            receiver_id,
            token_id,
            approved_account_ids,
        )
    }
}

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}

// #[cfg(all(test, not(target_arch = "wasm32")))]
// mod tests {
//     use near_sdk::test_utils::{accounts, VMContextBuilder};
//     use near_sdk::{testing_env, NearToken};
//     use std::collections::HashMap;

//     use super::*;

//     const MINT_STORAGE_COST: NearToken = NearToken::from_yoctonear(5870000000000000000000u128);

//     fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
//         let mut builder = VMContextBuilder::new();
//         builder
//             .current_account_id(accounts(0))
//             .signer_account_id(predecessor_account_id.clone())
//             .predecessor_account_id(predecessor_account_id);
//         builder
//     }

//     fn sample_token_metadata() -> TokenMetadata {
//         TokenMetadata {
//             title: Some("Olympus Mons".into()),
//             description: Some("The tallest mountain in the charted solar system".into()),
//             media: None,
//             media_hash: None,
//             copies: Some(1u64),
//             issued_at: None,
//             expires_at: None,
//             starts_at: None,
//             updated_at: None,
//             extra: None,
//             reference: None,
//             reference_hash: None,
//         }
//     }

//     #[test]
//     fn test_new() {
//         let mut context = get_context(accounts(1));
//         testing_env!(context.build());
//         let contract = Contract::new_default_meta(accounts(1).into());
//         testing_env!(context.is_view(true).build());
//         assert_eq!(contract.nft_token("1".to_string()), None);
//     }

//     #[test]
//     #[should_panic(expected = "The contract is not initialized")]
//     fn test_default() {
//         let context = get_context(accounts(1));
//         testing_env!(context.build());
//         let _contract = Contract::default();
//     }

//     #[test]
//     fn test_mint() {
//         let mut context = get_context(accounts(0));
//         testing_env!(context.build());
//         let mut contract = Contract::new_default_meta(accounts(0).into());

//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .predecessor_account_id(accounts(0))
//             .build());

//         let token_id = "0".to_string();
//         let token = contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());
//         assert_eq!(token.token_id, token_id);
//         assert_eq!(token.owner_id, accounts(0));
//         assert_eq!(token.metadata.unwrap(), sample_token_metadata());
//         assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
//     }

//     #[test]
//     fn test_transfer() {
//         let mut context = get_context(accounts(0));
//         testing_env!(context.build());
//         let mut contract = Contract::new_default_meta(accounts(0).into());

//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(MINT_STORAGE_COST)
//             .predecessor_account_id(accounts(0))
//             .build());
//         let token_id = "0".to_string();
//         contract.nft_mint(token_id.clone(), accounts(0), sample_token_metadata());

//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .attached_deposit(NearToken::from_yoctonear(1))
//             .predecessor_account_id(accounts(0))
//             .build());
//         contract.nft_transfer(accounts(1), token_id.clone(), None, None);

//         testing_env!(context
//             .storage_usage(env::storage_usage())
//             .account_balance(env::account_balance())
//             .is_view(true)
//             .attached_deposit(NearToken::from_near(0))
//             .build());
//         if let Some(token) = contract.nft_token(token_id.clone()) {
//             assert_eq!(token.token_id, token_id);
//             assert_eq!(token.owner_id, accounts(1));
//             assert_eq!(token.metadata.unwrap(), sample_token_metadata());
//             assert_eq!(token.approved_account_ids.unwrap(), HashMap::new());
//         } else {
//             panic!("token not correctly created, or not found by nft_token");
//         }
//     }
// }
