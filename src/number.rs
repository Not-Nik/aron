// aron (c) Nikolas Wipper 2022

pub trait Number {
    type Output: AsRef<[u8]>;

    fn to_bytes(&self, big_endian: bool) -> Self::Output;

    fn to_i8(&self) -> i8;
    fn to_i16(&self) -> i16;
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;

    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;

    fn to_f32(&self) -> f32;
    fn to_f64(&self) -> f64;
}

macro_rules! to_type_impl {
    ($ty:ty, $name:ident) => {
        fn $name(&self) -> $ty {
            *self as $ty
        }
    };
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

            to_type_impl!(i8, to_i8);
            to_type_impl!(i16, to_i16);
            to_type_impl!(i32, to_i32);
            to_type_impl!(i64, to_i64);

            to_type_impl!(u8, to_u8);
            to_type_impl!(u16, to_u16);
            to_type_impl!(u32, to_u32);
            to_type_impl!(u64, to_u64);

            to_type_impl!(f32, to_f32);
            to_type_impl!(f64, to_f64);
        }
    };
}

number_for_arithmetic!(u8);
number_for_arithmetic!(u16);
number_for_arithmetic!(u32);
number_for_arithmetic!(u64);

number_for_arithmetic!(i8);
number_for_arithmetic!(i16);
number_for_arithmetic!(i32);
number_for_arithmetic!(i64);

number_for_arithmetic!(f32);
number_for_arithmetic!(f64);
