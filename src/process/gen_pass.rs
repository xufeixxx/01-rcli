use crate::opts::GenPassOpts;
use rand::{seq::SliceRandom, Rng};
use zxcvbn::zxcvbn;

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(opts: &GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if opts.uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty."));
    }

    if opts.lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty."));
    }

    if opts.number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty."));
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty."));
    }

    password.shuffle(&mut rng);

    for _ in 0..(opts.length as usize - password.len()) {
        let idx = rng.gen_range(0..chars.len());
        password.push(chars[idx]);
    }

    let password_string = String::from_utf8(password)?;
    println!("{}", password_string);

    let estimate = zxcvbn(&password_string, &[])?;
    eprintln!("Password strength {}", estimate.score());

    // output password strength in stderr

    Ok(())
}
