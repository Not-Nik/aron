// aron (c) Nikolas Wipper 2022

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub trait Number {
    type Output: AsRef<[u8]>;

    fn to_bytes(&self, big_endian: bool) -> Self::Output;
}

macro_rules! number_for_arithmetic {
    ($ty:ty) => {
        impl Number for $ty {
            type Output = [u8; std::mem::size_of::<$ty>()];

            fn to_bytes(&self, big_endian: bool) -> Self::Output {
                if big_endian {
                    self.to_be_bytes()
                } else {
                    self.to_le_bytes()
                }
            }
        }
    };
}

number_for_arithmetic!(i8);
number_for_arithmetic!(i16);
number_for_arithmetic!(i32);
number_for_arithmetic!(i64);

