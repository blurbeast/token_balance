#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub(crate) mod events;
pub(crate) mod interfaces;
pub(crate) mod errors;

#[ink::contract]
mod token_balance {

    use ink::storage::Mapping;
    use crate::interfaces::{
        TokenBalanceView,
        TokenBalanceMut
    };
    use crate::events::OwnershipTransferred;
    use crate::errors::{
        AppResult, TokenBalanceError
    };


    #[ink(storage)]
    pub struct TokenBalance {
        owner: AccountId,
        balances: Mapping<AccountId, u128>,
        allowances: Mapping<(AccountId, AccountId), u128>,
        is_paused: bool,
        total_supply: u128,
        black_listed: Mapping<AccountId, bool>,
    }

    impl Default for TokenBalance {
        fn default() -> Self {
           Self::new()
        }
    }

    impl TokenBalance {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::env().emit_event(
                OwnershipTransferred {
                    from: None,
                    to: Self::env().caller(),
                }
            );
            Self {
                owner: Self::env().caller(),
                balances: Mapping::default(),
                allowances: Mapping::default(),
                is_paused: false,
                total_supply: 0,
                black_listed: Mapping::default(),
            }
        }
    }

    impl TokenBalanceView for TokenBalance {
        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u128 {
            self.balances.get(&owner).unwrap_or(0)
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.total_supply
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.allowances.get(&(owner, spender)).unwrap_or(0)
        }
    }

    impl TokenBalanceMut for TokenBalance {
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: u128) -> AppResult<bool> {
            unimplemented!()
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> AppResult<bool> {
            unimplemented!()
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128) -> AppResult<bool> {
            unimplemented!()
        }

        #[ink(message)]
        fn burn(&mut self, value: u128) -> AppResult<bool> {
            unimplemented!()
        }

        #[ink(message)]
        fn mint(&mut self, to: AccountId, value: u128) -> AppResult<bool> {
            unimplemented!()
        }
    }
    
    impl TokenBalance {
        fn _transfer(&mut self, from: AccountId, to: AccountId, value: u128) -> AppResult<bool> {
            // get the balance_of_from
            let balance_from = self.balance_of(from);
            // check if the balance is sufficient
            if balance_from < value {
                return Err(TokenBalanceError::InsufficientBalance);
            }
            // update the balance of the sender
            self.balances.insert(from, &balance_from.saturating_sub(value));
            // update the balance of the receiver
            let balance_to = self.balance_of(to);
            self.balances.insert(to, &balance_to.saturating_add(value));
            Ok(true)
        }
        
        fn _approve(&mut self, owner: AccountId, spender: AccountId, value: u128) -> AppResult<bool> {
            unimplemented!()
        }
        
        fn _contract_owner(&self, account_id: AccountId) -> AppResult<bool> {
            unimplemented!()
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[ink::test]
        fn default_works() {
            let _ = TokenBalance::default();
        }

        #[ink::test]
        fn it_works() {
            let mut token_balance = TokenBalance::new();
            assert_eq!(token_balance.get(), false);
        }
    }
}
