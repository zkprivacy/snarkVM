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

impl<E: Environment, I: IntegerType> FromField for Integer<E, I> {
    type Field = Field<E>;

    /// Casts an integer from a base field.
    fn from_field(field: Self::Field) -> Self {
        // Note: We are reconstituting the integer from the base field.
        // This is safe as the number of bits in the integer is less than the base field modulus,
        // and thus will always fit within a single base field element.
        debug_assert!(I::BITS < E::BaseField::size_in_bits() as u64);

        // Extract the integer bits from the field element, **without** a carry bit.
        let bits_le = field.to_lower_bits_le(I::BITS as usize);

        // Return the integer.
        Integer { bits_le, phantom: Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use snarkvm_circuit_environment::Circuit;

    const ITERATIONS: u64 = 128;

    fn check_from_field<I: IntegerType>(mode: Mode, rng: &mut TestRng) {
        for i in 0..ITERATIONS {
            // Sample a random integer.
            let expected = Uniform::rand(rng);
            let candidate = Integer::<Circuit, I>::new(mode, expected).to_field();

            Circuit::scope(format!("{mode} {expected} {i}"), || {
                // Perform the operation.
                let candidate = Integer::from_field(candidate);
                assert_eq!(expected, candidate.eject_value());
                match mode {
                    Mode::Constant => assert_scope!(I::BITS, 0, 0, 0),
                    _ => assert_scope!(0, 0, I::BITS, I::BITS + 1),
                }
            });
            Circuit::reset();
        }
    }

    #[test]
    fn test_u8_from_field() {
        let mut rng = TestRng::default();

        type I = u8;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_i8_from_field() {
        let mut rng = TestRng::default();

        type I = i8;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_u16_from_field() {
        let mut rng = TestRng::default();

        type I = u16;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_i16_from_field() {
        let mut rng = TestRng::default();

        type I = i16;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_u32_from_field() {
        let mut rng = TestRng::default();

        type I = u32;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_i32_from_field() {
        let mut rng = TestRng::default();

        type I = i32;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_u64_from_field() {
        let mut rng = TestRng::default();

        type I = u64;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_i64_from_field() {
        let mut rng = TestRng::default();

        type I = i64;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_u128_from_field() {
        let mut rng = TestRng::default();

        type I = u128;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }

    #[test]
    fn test_i128_from_field() {
        let mut rng = TestRng::default();

        type I = i128;
        check_from_field::<I>(Mode::Constant, &mut rng);
        check_from_field::<I>(Mode::Public, &mut rng);
        check_from_field::<I>(Mode::Private, &mut rng);
    }
}
