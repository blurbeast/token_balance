use ink::primitives::AccountId;

#[ink::event]
pub(crate) struct OwnershipTransferred {
  #[ink(topic)]
  pub from: Option<AccountId>,
  #[ink(topic)]
  pub to: AccountId,
}

#[ink::event]
pub(crate) struct Transfer {
  #[ink(topic)]
  pub from: AccountId,
  #[ink(topic)]
  pub to: AccountId,
  pub value: u128,
}

#[ink::event]
pub(crate) struct Approval {
  #[ink(topic)]
  pub owner: AccountId,
  #[ink(topic)]
  pub spender: AccountId,
  pub value: u128,
}

#[ink::event]
pub(crate) struct Mint {
  #[ink(topic)]
  pub to: AccountId,
  pub value: u128,
}

#[ink::event]
pub(crate) struct Burn {
  #[ink(topic)]
  pub from: AccountId,
  pub value: u128,
}

#[ink::event]
pub(crate) struct Pause {
  #[ink(topic)]
  pub paused: bool,
}
