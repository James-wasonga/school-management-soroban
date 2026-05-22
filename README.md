# School Management Soroban Contract

A decentralized School Management System smart contract built on the Stellar Soroban platform.  
This contract enables schools to register students, manage student records, process fee payments, and track payment history securely on-chain.

---

# Features

- Student registration
- Student record management
- Fee payment processing
- Payment history tracking
- Student class updates
- Secure authentication using Stellar addresses
- Event emission for payment tracking

---

# Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain

---

# Project Structure

```text
.
├── contracts
│   └── school-management-soroban
│       ├── src
│       │   ├── lib.rs
│       │   ├── school_management.rs
│       │   ├── storage.rs
│       │   ├── events.rs
│       │   ├── error.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

---

# Smart Contract Functionalities

## Constructor

Initializes the contract with:

- School administrator address
- Token contract address used for payments

---

## Student Registration

Allows students to register with:

- Wallet address
- Name
- Class category

Each student receives a unique student ID.

---

## Make Payment

Students can pay school fees using the configured Stellar token contract.

The contract:

- Authenticates the student
- Transfers tokens to the school admin
- Stores payment records
- Emits payment events

---

## Update Student Class

Allows a registered student to update their class category.

---

## Get Student Details

Fetches student information using the student ID.

---

## Payment History

Returns all payment records associated with a student.

---

## Remove Student

Marks a student as removed from the system.

---

# Data Structures

## StudentDetails

Stores:

- Student ID
- Name
- Wallet address
- Class
- Total fees paid
- Registration status

## Payment

Stores:

- Student ID
- Payment amount
- Timestamp

## DataKey

Used for contract storage management.

## Class

Supported classes:

- Grade
- HighSchool
- College

---

# Installation

Clone the repository:

```bash
git clone https://github.com/James-wasonga/school-management-soroban
cd school-management-soroban
```

---

# Build the Contract

```bash
stellar contract build
```

---

# Run Tests

```bash
cargo test
```

---

# Important Notes

- The project uses `#![no_std]` because Soroban contracts compile to WebAssembly (WASM).
- The contract is designed for the `wasm32v1-none` target.
- Soroban SDK v25 does not use `soroban_sdk::prelude::*`.

---

# Future Improvements

- Role-based access control
- Attendance management
- Exam and grading system
- Teacher management
- NFT-based student certificates
- Advanced analytics dashboard

---

# License

This project is licensed under the MIT License.

---

# Author
Developed by James Wasonga.