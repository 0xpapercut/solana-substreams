use super::state::{Key, TokenMetadataAccount};
use super::error::ProgramError;

pub fn try_from_slice_checked<T: TokenMetadataAccount>(
    data: &[u8],
    data_type: Key,
    data_size: usize,
) -> Result<T, ProgramError> {
    if !T::is_correct_account_type(data, data_type, data_size) {
        panic!();
    }

    let mut data_mut = data;
    let result = T::deserialize(&mut data_mut).unwrap();

    Ok(result)
}
