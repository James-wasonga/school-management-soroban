use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub enum ContractError {
    InsufficientFunds = 1,
}


