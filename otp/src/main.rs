use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use data_encoding::HEXLOWER;
use orion::aead;
use ring::hmac;
use std::convert::TryInto;

use std::fs::File;
use std::io::{self, Read, Write};
use std::time::SystemTime;

#[derive(Parser, Debug)]
struct Args {
    #[clap(
        short = 'g',
        long,
        help = "store a hex string of minimum 64 chars into ft_otp.key"
    )]
    secret: Option<String>,
    #[clap(
        short = 'k',
        long = "gen",
        help = "generate a totp from ft_otp.key",
        default_value = "false"
    )]
    generate: bool,
    #[clap(
        short,
        long,
        help = "specify file encryption password. will be asked to stdin otherwise"
    )]
    password: Option<String>,
}

fn get_stdin() -> Result<String> {
    print!("Enter encryption key:");
    io::stdout().flush().unwrap();
    let mut key_input = String::new();
    io::stdin()
        .read_line(&mut key_input)
        .context("Failed to read from stdin")?;
    Ok(key_input.trim().to_string())
}

fn encrypt_file(path: &str, data: &[u8], args: &Args) -> Result<()> {
    let secretinput = match args.password.clone() {
        Some(str) => str,
        None => get_stdin()?,
    };
    let mut secret = secretinput.as_bytes().to_vec();
    secret.resize(32, 0);
    let secretkey = aead::SecretKey::from_slice(&secret)?;
    let ciphertext = aead::seal(&secretkey, data).context("sealing key")?;
    let mut file = File::create(path).context(path.to_string())?;
    file.write(&ciphertext).context(path.to_string())?;
    Ok(())
}
fn decrypt_file(path: &str, args: &Args) -> Result<Vec<u8>> {
    let mut file =
        File::open(path).context("you probably need to store a new key first. see --store.")?;
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;
    let secretinput = match args.password.clone() {
        Some(str) => str,
        None => get_stdin()?,
    };
    let mut secret = secretinput.as_bytes().to_vec();
    secret.resize(32, 0);
    let secretkey = aead::SecretKey::from_slice(&secret)?;
    let text = aead::open(&secretkey, &ciphertext).context("wrong password")?;
    Ok(text)
}

/// Calculates the HMAC digest
fn calc_digest(decoded_secret: &[u8], counter: u64) -> hmac::Tag {
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, decoded_secret);
    hmac::sign(&key, &counter.to_be_bytes())
}

/// Encodes the HMAC digest into a 6-digit integer.
fn encode_digest(digest: &[u8]) -> Result<u32> {
    let offset = match digest.last() {
        Some(x) => *x & 0xf,
        None => bail!("Invalid digest: {:?}", digest),
    } as usize;
    let code_bytes: [u8; 4] = digest[offset..offset + 4].try_into()?;
    let code = u32::from_be_bytes(code_bytes);
    Ok((code & 0x7fffffff) % 1_000_000)
}

pub fn totp(secret: &[u8], time_step: u64) -> Result<u32> {
    let now = SystemTime::now();
    let time_since_epoch = now.duration_since(SystemTime::UNIX_EPOCH)?;
    let counter = time_since_epoch.as_secs() / time_step;
    let digest = calc_digest(secret, counter);
    encode_digest(digest.as_ref())
}

fn main() -> Result<()> {
    let args = Args::parse();
    if args.generate {
        let key = decrypt_file("ft_otp.key", &args)?;
        let code = totp(&key, 30)?;
        println!("code: {code:0>6}");
    } else if let Some(secret) = args.secret.clone() {
        if secret.len() < 64 {
            return Err(anyhow!("Secret must be at least 64 characters long."));
        }
        let secret = match HEXLOWER.decode(secret.as_bytes()) {
            Ok(x) => x,
            Err(e) => {
                return Err(anyhow!("Secret must be hexadecimal.\n{:?}", e));
            }
        };
        encrypt_file("ft_otp.key", &secret, &args)?;
    } else {
        println!("No actions specified. see --help");
    }
    Ok(())
}
