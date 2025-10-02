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
