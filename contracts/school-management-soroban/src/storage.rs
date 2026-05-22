use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone)]
pub struct StudentDetails {
    pub student_id: u64,
    pub name: String,
    pub wallet_address: Address,
    pub class_name: Class,
    pub total_paid: i128,
    pub is_registered: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct Payment {
    pub student_id: u64,
    pub amount: i128,
    pub timestamp: u64,
} 

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Token,
    Student(u64),
    StudentPayments(u64),
    StudentCount,
}

#[contracttype]
#[derive(Clone)]
pub enum Class {
    Grade,
    HighSchool,
    College,
}