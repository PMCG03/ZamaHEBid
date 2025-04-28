// src/parameters_builder.rs

// Import the types we need from TFHE.
use tfhe::shortint::parameters::{PBSParameters, ClassicPBSParameters, PARAM_MESSAGE_2_CARRY_2};
use tfhe::shortint::{MessageModulus, CarryModulus};

/// A minimal builder for PBSParameters.
///
/// This builder assumes that a default parameter constant (PARAM_MESSAGE_2_CARRY_2)
/// is available, and then overrides the fields for the message and carry moduli.
/// For values up to 10,000 you need at least 15 message bits (i.e. 2^15 = 32768).
pub struct PBSParametersBuilder {
    message_bits: u32,
    carry_bits: u32,
    // We omit variance because the ClassicPBSParameters struct does not expose a `variance` field.
}

impl PBSParametersBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        PBSParametersBuilder {
            message_bits: 15, // default to 15 message bits
            carry_bits: 3,    // default to 3 carry bits
        }
    }

    /// Specifies the number of message bits.
    pub fn message_bits(mut self, bits: u32) -> Self {
        self.message_bits = bits;
        self
    }

    /// Specifies the number of carry bits.
    pub fn carry_bits(mut self, bits: u32) -> Self {
        self.carry_bits = bits;
        self
    }

    /// Builds and returns the PBSParameters.
    pub fn build(self) -> PBSParameters {
        // Start with a default parameter constant and modify it.
        let mut params: ClassicPBSParameters = PARAM_MESSAGE_2_CARRY_2.clone();
        // Set the message modulus and carry modulus using the new bit-widths.
        params.message_modulus = MessageModulus(1 << self.message_bits); // e.g., 15 bits gives 32768.
        params.carry_modulus = CarryModulus(1 << self.carry_bits);         // e.g., 3 bits gives 8.
        // Return it wrapped as the PBS variant.
        PBSParameters::PBS(params)
    }
}
