// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> Parser for ProgramID<N> {
    /// Parses a string into a program ID of the form `{name}.{network}`.
    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        // Parse the name from the string.
        let (string, name) = Identifier::parse(string)?;
        // Parse the "." and network-level domain (NLD) from the string.
        let (string, (_, network)) = pair(tag("."), Identifier::parse)(string)?;
        // Return the program ID.
        Ok((string, Self { name, network }))
    }
}

impl<N: Network> FromStr for ProgramID<N> {
    type Err = Error;

    /// Parses a string into a program ID.
    #[inline]
    fn from_str(string: &str) -> Result<Self> {
        match Self::parse(string) {
            Ok((remainder, object)) => {
                // Ensure the remainder is empty.
                ensure!(remainder.is_empty(), "Failed to parse string. Found invalid character in: \"{remainder}\"");
                // Return the object.
                Ok(object)
            }
            Err(error) => bail!("Failed to parse string. {error}"),
        }
    }
}

impl<N: Network> Debug for ProgramID<N> {
    /// Prints the program ID as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<N: Network> Display for ProgramID<N> {
    /// Prints the program ID as a string.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{name}.{network}", name = self.name, network = self.network)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_console_network::Testnet3;

    type CurrentNetwork = Testnet3;

    #[test]
    fn test_parse() -> Result<()> {
        let id = ProgramID::<CurrentNetwork>::parse("bar.aleo").unwrap().1;
        assert_eq!(id.name(), &Identifier::<CurrentNetwork>::from_str("bar")?);
        assert_eq!(id.network(), &Identifier::<CurrentNetwork>::from_str("aleo")?);

        assert!(ProgramID::<CurrentNetwork>::parse("foo").is_err());

        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let id = ProgramID::<CurrentNetwork>::from_str("bar.aleo")?;
        assert_eq!("bar.aleo", id.to_string());

        assert!(ProgramID::<CurrentNetwork>::from_str("foo").is_err());

        Ok(())
    }
}
