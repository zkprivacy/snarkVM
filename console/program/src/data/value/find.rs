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

impl<N: Network> Value<N> {
    /// Returns the value from the given path.
    pub fn find(&self, path: &[Identifier<N>]) -> Result<Self> {
        match self {
            Self::Plaintext(plaintext) => Ok(Self::Plaintext(plaintext.find(path)?)),
            Self::Record(record) => {
                // Find the entry.
                let entry = record.find(path)?;
                // Extract the plaintext from the entry.
                match entry {
                    Entry::Constant(plaintext) => Ok(Self::Plaintext(plaintext)),
                    Entry::Public(plaintext) => Ok(Self::Plaintext(plaintext)),
                    Entry::Private(plaintext) => Ok(Self::Plaintext(plaintext)),
                }
            }
        }
    }
}
