use log::debug;
use ring::digest::{Context, SHA256, SHA512};

fn main() -> Result<(), std::io::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        display_help(&args);
        return Ok(());
    }

    let file = &args[1];
    let given_hash = &args[2].to_lowercase();

    let data = std::fs::read(file)?;

    let result_hash = match given_hash.len() {
        64 => sha256_hash(data.as_slice()),
        128 => sha512_hash(data.as_slice()),
        _ => {
            println!("Invalid hash length");
            return Ok(());
        }
    };

    debug!("Comparing\n{:x?} to\n{:x?}", given_hash, result_hash);
    if !result_hash.eq(given_hash) {
        println!("File doesn't match hash");
    } else {
        println!("File matches hash!");
    }

    Ok(())
}

fn display_help(args: &Vec<String>) {
    println!("Invalid args, expected: {} <filename> <expected-hash>", args[0]);
}

fn sha256_hash(data: &[u8]) -> String {
    let mut ctx = Context::new(&SHA256);
    ctx.update(data);
    let digest = ctx.finish();
    data_encoding::HEXLOWER.encode(digest.as_ref())
}

fn sha512_hash(data: &[u8]) -> String {
    let mut ctx = Context::new(&SHA512);
    ctx.update(data);
    let digest = ctx.finish();
    data_encoding::HEXLOWER.encode(digest.as_ref())
}
