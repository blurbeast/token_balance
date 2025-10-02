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
    use crate::events::{
        OwnershipTransferred, 
        Transfer, 
        Approval, 
        Mint, 
        Burn, 
        Pause
    };
    use crate::errors::{
        AppResult, TokenBalanceError,
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
            self.balances.get(owner).unwrap_or(0)
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.total_supply
        }

        #[ink(message)]
        fn allowance(&self, owner: AccountId, spender: AccountId) -> u128 {
            self.allowances.get((owner, spender)).unwrap_or(0)
        }
    }

    impl TokenBalanceMut for TokenBalance {
        #[ink(message)]
        fn transfer(&mut self, to: AccountId, value: u128) -> AppResult<bool> {
            let caller: AccountId = self.env().caller();
            self._check_to_not_self(caller, to)?;
            self._transfer(caller, to, value)?;
            self.env().emit_event(Transfer {
                from: caller,
                to,
                value,
            });
            Ok(true)
        }

        #[ink(message)]
        fn approve(&mut self, spender: AccountId, value: u128) -> AppResult<bool> {
            let caller: AccountId = self.env().caller();
            self._check_to_not_self(caller, spender)?;
            self._approve(caller, spender, value)?;
            self.env().emit_event(Approval {
                owner: caller,
                spender,
                value,
            });
            Ok(true)
        }

        #[ink(message)]
        fn transfer_from(&mut self, from: AccountId, to: AccountId, value: u128) -> AppResult<bool> {
            let caller: AccountId = self.env().caller();
            let allowance: u128 = self.allowance(from, caller);
            self._check_allowance(from, caller, value)?;
            self._transfer(from, to, value)?;
            self._approve(from, caller, allowance.saturating_sub(value))?;
            self.env().emit_event(Transfer {
                from,
                to,
                value,
            });
            Ok(true)
        }

        #[ink(message)]
        fn burn(&mut self, value: u128) -> AppResult<bool> {
            let caller: AccountId = self.env().caller();
            self._transfer(caller, AccountId::from([0; 32]), value)?;
            self._increase_total_supply(value, false);
            self.env().emit_event(Burn {
                from: caller,
                value,
            });
            Ok(true)
        }

        #[ink(message)]
        fn mint(&mut self, to: AccountId, value: u128) -> AppResult<bool> {
            self._assert_not_pause(self.is_paused)?;
            let caller: AccountId = self.env().caller();
            self._only_owner(caller)?;
            self._mint(to, value);
            
            self.env().emit_event(Mint {
                to,
                value,
            });
            Ok(true)
        }
        
        #[ink(message)]
        fn pause(&mut self) -> AppResult<bool> {
            self._assert_not_pause(self.is_paused)?;
            let caller: AccountId = self.env().caller();
            self._only_owner(caller)?;
            self._pause_contract(true)?;
            
            self.env().emit_event(Pause {
                paused: true,
            });
            
            Ok(true)
        }
        
        #[ink(message)]
        fn unpause(&mut self) -> AppResult<bool> {
            let caller: AccountId = self.env().caller();
            self._only_owner(caller)?;
            self._pause_contract(false)?;
            
            self.env().emit_event(Pause {
                paused: false,
            });
            Ok(true)
        }
    }

    impl TokenBalance {
        fn _mint(&mut self, to: AccountId, value: u128) {
            let balance: u128 = self.balance_of(to);
            self.balances.insert(to, &balance.saturating_add(value));
            self._increase_total_supply(value, true);
        }

        fn _increase_total_supply(&mut self, value: u128, reduce: bool) {
            match reduce {
                false => self.total_supply = self.total_supply.saturating_sub(value),
                _ => self.total_supply = self.total_supply.saturating_add(value),
            }
        }

        fn _transfer(&mut self, from: AccountId, to: AccountId, value: u128) -> AppResult<()> {
            self._assert_not_pause(self.is_paused)?;
            let balance_from: u128 = self.balance_of(from);
            if balance_from < value {
                return Err(TokenBalanceError::InsufficientBalance);
            }
            // subtract
            self.balances.insert(from, &(balance_from.saturating_sub(value)));
            // add
            let balance_to: u128 = self.balance_of(to);
            self.balances.insert(to, &(balance_to.saturating_add(value)));
            Ok(())
        }

        fn _approve(&mut self, owner: AccountId, spender: AccountId, value: u128) -> AppResult<()> {
            self._assert_not_pause(self.is_paused)?;
            self.allowances.insert((owner, spender), &value);
            Ok(())
        }

        fn _only_owner(&self, caller: AccountId) -> AppResult<()> {
            if  caller != self.owner {
                return Err(TokenBalanceError::NotAuthorized);
            }
            Ok(())
        }

        fn _check_allowance(&self, from: AccountId, caller: AccountId, value: u128) -> AppResult<()> {
            let allowance: u128 = self.allowance(from, caller);
            if allowance < value {
                return Err(TokenBalanceError::NotEnoughAllowance);
            }
            Ok(())
        }

        fn _check_to_not_self(&self, from: AccountId, to: AccountId) -> AppResult<()> {
            if from == to {
                return Err(TokenBalanceError::SenderIsSelf);
            }
            Ok(())
        }
        
        fn _assert_not_pause(&self, paused: bool) -> AppResult<()> {
            if paused {
                return Err(TokenBalanceError::Paused);
            }
            Ok(())
        }
        
        fn _pause_contract(&mut self, pause: bool) -> AppResult<()> {
            match pause {
                true => self.is_paused = true,
                false => self.is_paused = false,
            }
            Ok(())
        }
    }
}
