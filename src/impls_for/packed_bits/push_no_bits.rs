use crate::{Push, NoBits, PackedBits};

impl Push<NoBits> for PackedBits {
    fn push(&mut self, _val: NoBits) {

    }
}
