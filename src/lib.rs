use std::ops::Deref;

// Was curious to see if there was a way to utilize the wasted space that a lot of
// compacting runs into. If values don't match up with a power of two there is inherent waste.
//
// Combining values seems like a common way to get around this. Kind of see it a bit like
// arithmetic encoding or linearization of multidimensional arrays.
//
// e.g.
//
// 3d array of 18x18x18 will be 5832 elements, indexing this via each axis like:
// - needs 2^5, but that wastes 14 values per axis, 43.75% of the space allocated to it.
// indexing it via a simple linearization scheme: x + y * 18 + z * 18 * 18
// means we only need a number for `5832` which requires 13 bits savings 2 bits.
//
// https://save-buffer.github.io/ultrapack.html

pub const fn optimal_bundle_size(max_value: u64) -> BundleSize {
    let naive_bits = if max_value == 0 {
        return BundleSize(0);
    } else {
        max_value.ilog2() + 1
    };

    let mut best_size = 1u8;
    let mut best_bits_per_val = naive_bits as f64;

    // test bundle sizes until we overflow u64
    let mut bundle_size = 1;
    while bundle_size <= 40u8 {
        // max_value^k - 1
        let Some(max_bundle) = max_value.checked_pow(bundle_size as u32) else {
            break;
        };

        let bits_needed = (64 - (max_bundle - 1).leading_zeros()) as u8;
        let bits_per_val = bits_needed as f64 / bundle_size as f64;

        if bits_per_val < best_bits_per_val {
            best_bits_per_val = bits_per_val;
            best_size = bundle_size;
        }

        bundle_size += 1;
    }

    BundleSize(best_size)
}

pub const fn bits_per_bundle(max_value: u64, bundle_size: BundleSize) -> BitsPerBundle {
    let max_bundle = max_value.pow(bundle_size.0 as u32);
    BitsPerBundle((64 - (max_bundle - 1).leading_zeros()) as u8)
}

#[derive(Copy, Clone, Debug)]
pub struct BundleSize(pub u8);
impl Deref for BundleSize {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BitsPerBundle(pub u8);
impl Deref for BitsPerBundle {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// theoretical minimum bits per value
pub fn theoretical_bits(max_value: u64) -> f64 {
    (max_value as f64).log2()
}

/// traditional bitpacking bits per value
pub fn naive_bits(max_value: u64) -> u8 {
    (64 - (max_value - 1).leading_zeros()) as u8
}

pub fn encode(bundle_size: BundleSize, max_value: u64, values: &[u64]) -> u64 {
    assert_eq!(values.len(), *bundle_size as usize);

    let mut bundle: u64 = 0;
    for &val in values {
        assert!(val < max_value);
        bundle = bundle * max_value + val;
    }
    bundle
}

pub fn decode(bundle_size: BundleSize, max_value: u64, mut bundle: u64) -> Vec<u64> {
    let mut values = vec![0u64; *bundle_size as usize];

    for i in (0..*bundle_size as usize).rev() {
        values[i] = bundle % max_value;
        bundle /= max_value;
    }

    values
}
