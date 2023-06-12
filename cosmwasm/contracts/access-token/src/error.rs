use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error(transparent)]
    Base(#[from] account_base::error::ContractError),

    #[error(transparent)]
    Std(#[from] cosmwasm_std::StdError),
}

pub type ContractResult<T> = Result<T, ContractError>;