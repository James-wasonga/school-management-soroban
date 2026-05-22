use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct PaymentEvent {
    #[topic]
    pub wallet_address: Address,
    pub student_id: u64,
    pub amount: u32,
}