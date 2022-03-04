#![no_std]

type Mask = u32;

/// Numbers of bits in a `Mask`
type LogMask = u32;

fn bit_mask(num_bits: Mask) -> Mask {
    if num_bits != 0 {
        (((1 as Mask) << (num_bits - 1)) << 1).wrapping_sub(1)
    } else {
        0
    }
}

/*
pub fn from_range_low(base: Mask, max: Mask) -> LogMask {
    assert!(base <= max);
    let base_tz = base.count_zeros();
    let mask1 = bit_mask(base_tz);
}

pub fn from_range_high(base: Mask, min: Mask) -> LogMask  {
    todo!()
}

pub fn from_range(base: Mask, limit: Mask) -> LogMask {
    todo!()
}

pub fn matches(base: Mask, mask_bits: LogMask, value: V) -> bool {
    todo!()
}

pub fn max(base: Mask, mask_bits: LogMask) -> Mask {
    todo!()
}

pub fn base(base: Mask, mask_bits: LogMask) -> Mask {
    todo!()
}
*/

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_bit_mask {
        () => {};
        ($n:ident => $i:expr, $o:expr; $($tail:tt)*) => {
            #[test]
            fn $n() {
                assert_eq!(bit_mask($i), $o);
            }

            test_bit_mask!($($tail)*);
        }
    }

    test_bit_mask!{
        bit_mask_0 => 0, 0b0000;
        bit_mask_1 => 1, 0b0001;
        bit_mask_2 => 2, 0b0011;
        bit_mask_3 => 3, 0b0111;
        bit_mask_4 => 4, 0b1111;
        bit_mask_max_m1 => 31, 0b01111111_11111111_11111111_11111111;
        bit_mask_max => 32, Mask::max_value();
    }
}
