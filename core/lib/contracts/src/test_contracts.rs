// Import necessary modules to use types and structures from the `ethabi` library
use ethabi::{ethereum_types::U256, Bytes, Token};
// Import the `Deserialize` trait from the Serde library
use serde::Deserialize;

// Import the `get_loadnext_contract` function from the current module
use crate::get_loadnext_contract;

// Define a structure named `LoadnextContractExecutionParams`
#[derive(Debug, Clone, Deserialize)]
pub struct LoadnextContractExecutionParams {
    // Define parameters required for an Ethereum smart contract function
    pub reads: usize,
    pub writes: usize,
    pub events: usize,
    pub hashes: usize,
    pub recursive_calls: usize,
    pub deploys: usize,
}

// Implement methods for the `LoadnextContractExecutionParams` structure
impl LoadnextContractExecutionParams {
    // Method to retrieve values from environment variables with a specific prefix
    pub fn from_env() -> Option<Self> {
        envy::prefixed("CONTRACT_EXECUTION_PARAMS_").from_env().ok()
    }

    // Method to create an empty `LoadnextContractExecutionParams` structure
    pub fn empty() -> Self {
        Self {
            reads: 0,
            writes: 0,
            events: 0,
            hashes: 0,
            recursive_calls: 0,
            deploys: 0,
        }
    }
}

// Define default values for the `LoadnextContractExecutionParams` structure
impl Default for LoadnextContractExecutionParams {
    fn default(

