use super::*;

pub const MAX_EDITION_MARKER_SIZE: usize = 32;

pub const EDITION_MARKER_BIT_SIZE: u64 = 248;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct EditionMarker {
    pub key: Key,
    pub ledger: [u8; 31],
}

impl Default for EditionMarker {
    fn default() -> Self {
        Self {
            key: Key::EditionMarker,
            ledger: [0; 31],
        }
    }
}

impl TokenMetadataAccount for EditionMarker {
    fn key() -> Key {
        Key::EditionMarker
    }

    fn size() -> usize {
        MAX_EDITION_MARKER_SIZE
    }
}

impl EditionMarker {
    // fn get_edition_offset_from_starting_index(edition: u64) -> Result<usize, ProgramError> {
    //     Ok(edition
    //         .checked_rem(EDITION_MARKER_BIT_SIZE)
    //         .ok_or(MetadataError::NumericalOverflowError)? as usize)
    // }

    // fn get_index(offset_from_start: usize) -> Result<usize, ProgramError> {
    //     let index = offset_from_start
    //         .checked_div(8)
    //         .ok_or(MetadataError::NumericalOverflowError)?;

    //     // With only EDITION_MARKER_BIT_SIZE bits, or 31 bytes, we have a max constraint here.
    //     if index > 30 {
    //         return Err(MetadataError::InvalidEditionIndex.into());
    //     }

    //     Ok(index)
    // }

    // fn get_offset_from_right(offset_from_start: usize) -> Result<u32, ProgramError> {
    //     // We're saying the left hand side of a u8 is the 0th index so to get a 1 in that 0th index
    //     // you need to shift a 1 over 8 spots from the right hand side. To do that you actually
    //     // need not 00000001 but 10000000 which you can get by simply multiplying 1 by 2^7, 128 and then ORing
    //     // it with the current value.
    //     Ok(7 - offset_from_start
    //         .checked_rem(8)
    //         .ok_or(MetadataError::NumericalOverflowError)? as u32)
    // }

    // pub fn get_index_and_mask(edition: u64) -> Result<(usize, u8), ProgramError> {
    //     // How many editions off we are from edition at 0th index
    //     let offset_from_start = EditionMarker::get_edition_offset_from_starting_index(edition)?;

    //     // How many whole u8s we are from the u8 at the 0th index, which basically dividing by 8
    //     let index = EditionMarker::get_index(offset_from_start)?;

    //     // what position in the given u8 bitset are we (remainder math)
    //     let my_position_in_index_starting_from_right =
    //         EditionMarker::get_offset_from_right(offset_from_start)?;

    //     Ok((index, u8::pow(2, my_position_in_index_starting_from_right)))
    // }

    // pub fn edition_taken(&self, edition: u64) -> Result<bool, ProgramError> {
    //     let (index, mask) = EditionMarker::get_index_and_mask(edition)?;

    //     // apply mask with bitwise and with a 1 to determine if it is set or not
    //     let applied_mask = self.ledger[index] & mask;

    //     // What remains should not equal 0.
    //     Ok(applied_mask != 0)
    // }
}
