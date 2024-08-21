use borsh::{BorshSerialize, BorshDeserialize};
// mod system_program;
// use system_program::CreateAccountWithSeed;
use std::fmt;
// use system_program_substream::pubkey::Pubkey;
impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Pubkey").field(name, value)
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pubkey(pub [u8; 32]);

#[derive(Debug)]
pub struct Seed(String);

impl BorshDeserialize for Seed {
    fn deserialize_reader<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut length_bytes = [0u8; 8];
        reader.read_exact(&mut length_bytes)?;
        let length = u64::from_le_bytes(length_bytes) as usize;

        let mut string_bytes = vec![0u8; length];
        reader.read_exact(&mut string_bytes)?;

        let seed_string = String::from_utf8(string_bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;

        Ok(Seed(seed_string))
    }
}

impl BorshSerialize for Seed {
    fn serialize<W: std::io::prelude::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let string_bytes = self.0.as_bytes();
        let length = string_bytes.len() as u64;
        writer.write_all(&length.to_le_bytes())?;
        writer.write_all(string_bytes)?;
        Ok(())
    }
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CreateAccountWithSeed {
    /// Base public key
    pub base: Pubkey,
    /// String of ASCII chars, no longer than `Pubkey::MAX_SEED_LEN`
    pub seed: Seed,
    // /// Number of lamports to transfer to the new account
    // pub lamports: u64,
    // /// Number of bytes of memory to allocate
    // pub space: u64,
    // /// Owner program account address
    // pub owner: Pubkey,
}

fn main() {
    let slice = [
        72,247,212,230,58,228,137,100,154,248,245,94,133,131,146,126,71,88,206,202,164,237,221,122,17,33,166,212,228,209,140,193,
        32,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        104,
        68,
        56,
        99,
        56,
        50,
        113,
        122,
        86,
        86,
        84,
        115,
        53,
        88,
        114,
        110,
        101,
        119,
        52,
        57,
        82,
        56,
        66,
        109,
        116,
        50,
        82,
        68,
        56,
        109,
        70,
        50,
        // 240,29,31,0,0,0,0,0,
        // 165,0,0,0,0,0,0,0,
        // 6,221,246,225,215,101,161,147,217,203,225,70,206,235,121,172,28,180,133,237,95,91,55,145,58,140,245,133,126,255,0,169,
    ];
    let a = CreateAccountWithSeed::try_from_slice(&slice).unwrap();
    println!("{:#?}", a);
    // CreateAccountWithSeed {
    //     base: Pubkey {  }
    // }
    let pubkey = Pubkey::try_from_slice(&[72,247,212,230,58,228,137,100,154,248,245,94,133,131,146,126,71,88,206,202,164,237,221,122,17,33,166,212,228,209,140,193]).unwrap();
    let seed = String::from("hD8c82qzVVTs5Xrnew49R8Bmt2RD8mF2");
    let create_account = CreateAccountWithSeed {
        base: pubkey,
        seed: Seed(seed),
    };
    let mut writer = Vec::new();
    create_account.serialize(&mut writer).unwrap();
    println!("{:?}", writer);


    // CreateAccountWithSeed::serialize(&self, writer)
}
