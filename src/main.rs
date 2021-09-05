use anyhow::Result;
use std::env;
use std::fs;
use std::io::Cursor;
use std::io::Read;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        anyhow::bail!("Plese speficy filepath")
    }

    let filename = &args[1];
    let bytes = fs::read(filename)?;

    Dump::from_bytes(&bytes)?;

    return Ok(());
}

// NOTE Magic String "REDIS"
const MAGIC_STRING: &[u8] = &[0x52, 0x45, 0x44, 0x49, 0x53];

struct Dump {
    version: u8,
}

impl Dump {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut buff = Cursor::new(bytes);

        let mut magic_number_buf = [0; MAGIC_STRING.len()];

        buff.read_exact(&mut magic_number_buf)?;
        if !Dump::check_magic_number(&magic_number_buf) {
            anyhow::bail!("validate error")
        }

        // NOTE versionは4bytes
        let mut version = [0; 4];
        buff.read_exact(&mut version)?;
        println!("{}", String::from_utf8(version.to_vec())?);
        let version = String::from_utf8(version.to_vec())?;
        let version = version.parse()?;

        Ok(Self { version })
    }

    fn check_magic_number(bytes: &[u8]) -> bool {
        let len = MAGIC_STRING.len();

        if MAGIC_STRING != &bytes[0..len] {
            return false;
        }
        return true;
    }
}
