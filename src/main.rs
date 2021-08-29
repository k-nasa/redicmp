use anyhow::Result;
use std::env;
use std::fs;

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

struct Dump {}

impl Dump {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if !Dump::validate(bytes) {
            anyhow::bail!("validate error")
        }

        Ok(Self {})
    }

    fn validate(bytes: &[u8]) -> bool {
        let len = MAGIC_STRING.len();

        if MAGIC_STRING != &bytes[0..len] {
            return false;
        }
        return true;
    }
}
