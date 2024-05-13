use std::io::{BufReader, Read};

use log::debug;
use ring::digest::{Context, SHA256, SHA512};

fn main() -> Result<(), std::io::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        if args.len() == 2 {
            if args[1] == "--help" || args[1] == "help" {
                display_help(&args);
                return Ok(())
            }
        }
        display_help(&args);
        return Ok(())
    }

    if args[1] == "--sha256" {
        let file_handle = std::fs::File::open(&args[2])?;
        let buf_reader = BufReader::new(file_handle);
        let hash = sha_hash(buf_reader, &SHA256)?;
        println!("{}", hash);
        return Ok(())
    } else if args[1] == "--sha512" {
        let file_handle = std::fs::File::open(&args[2])?;
        let buf_reader = BufReader::new(file_handle);
        let hash = sha_hash(buf_reader, &SHA512)?;
        println!("{}", hash);
        return Ok(())
    }

    let file = &args[1];
    let given_hash = &args[2].to_lowercase();

    let file_handle = std::fs::File::open(file)?;
    let buf_reader = BufReader::new(file_handle);

    let result_hash = match given_hash.len() {
        64 => sha_hash(buf_reader, &SHA256)?,
        128 => sha_hash(buf_reader, &SHA512)?,
        _ => {
            println!("Invalid hash length");
            return Ok(())
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
    println!("Usage:");
    println!("  {} <filename> <expected-hash>", args[0]);
    println!("  {} --sha{{256,512}} <filename>", args[0]);
}

fn sha_hash(
    mut reader: impl Read,
    algorithm: &'static ring::digest::Algorithm,
) -> Result<String, std::io::Error> {
    let mut ctx = Context::new(algorithm);
    let mut buffer = [0; 1024];
    loop {
        let amount = reader.read(&mut buffer)?;
        if amount == 0 {
            break;
        }
        ctx.update(&buffer[..amount]);
    }
    let digest = ctx.finish();
    Ok(data_encoding::HEXLOWER.encode(digest.as_ref()))
}
