#![allow(clippy::suspicious_else_formatting)]

mod impls_for;
mod new_types;
mod structs;
mod traits;
mod unions;
mod utils;

#[cfg(test)]
mod tests;

pub use new_types::packed_range::PackedRange;
pub use structs::{no_bits::NoBits, packed_bits::PackedBits, range_definition::RangeDefinition};
pub use traits::{bit_packer::BitPacker, push::Push, quantise_float::QuantiseFloat,
                 range_def::RangeDef, shift::Shift};
use traits::{box_bit_packer::BoxBitPacker, num_byte_converter::NumByteConverter};
use unions::UnsignedSigned;
