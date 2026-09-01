// This file is part of Rundler.
//
// Rundler is free software: you can redistribute it and/or modify it under the
// terms of the GNU Lesser General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later version.
//
// Rundler is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Rundler.
// If not, see https://www.gnu.org/licenses/.

use alloy_primitives::{Address, B256, keccak256};

/// Seed used to derive unpredictable simulation-only addresses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationAddressSeed(B256);

impl SimulationAddressSeed {
    /// Creates a random seed for production use.
    pub fn random() -> Self {
        Self(B256::random())
    }

    /// Creates a seed with deterministic input.
    pub const fn new(seed: B256) -> Self {
        Self(seed)
    }

    /// Creates a deterministic seed without exposing Rundler's Alloy version.
    pub const fn from_bytes(seed: [u8; 32]) -> Self {
        Self(B256::new(seed))
    }

    /// Derives a stable address for one simulation purpose.
    pub fn address(self, domain: &[u8]) -> Address {
        let mut input = Vec::with_capacity(self.0.len() + domain.len());
        input.extend_from_slice(self.0.as_slice());
        input.extend_from_slice(domain);
        Address::from_slice(&keccak256(input)[12..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_addresses_are_stable_and_domain_separated() {
        let seed = SimulationAddressSeed::new(B256::repeat_byte(0x42));

        assert_eq!(seed.address(b"gas"), seed.address(b"gas"));
        assert_ne!(seed.address(b"gas"), seed.address(b"balances"));
    }
}
