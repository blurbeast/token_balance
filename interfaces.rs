use ink::primitives::AccountId;
use crate::errors::AppResult;

#[ink::trait_definition]
pub trait TokenBalanceView {

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u128;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, spender: AccountId) -> u128;
}

#[ink::trait_definition]
pub trait TokenBalanceMut {

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, value: u128) -> AppResult<bool>;

    #[ink(message)]
    fn approve(&mut self, spender: AccountId, value: u128) -> AppResult<bool>;

    #[ink(message)]
    fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128) -> AppResult<bool>;

    #[ink(message)]
    fn burn(&mut self, value: u128) -> AppResult<bool>;

    #[ink(message)]
    fn mint(&mut self, to: AccountId, value: u128) -> AppResult<bool>;
}
