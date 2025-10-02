
#[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum TokenBalanceError {
    InsufficientBalance,
    NotEnoughAllowance,
    NotAuthorized,
}

pub(crate) type AppResult<E> = Result<E, TokenBalanceError>;
