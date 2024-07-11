use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

use csv::Writer;
use rand_core::OsRng;
use k256::ecdsa::{signature::Signer, SigningKey, Signature};
use hex;


fn main() -> Result<(), Box<dyn Error>> {
    // Path to input and output CSV file
    let input_file = "data/heart+disease/processed.cleveland.data";
    let output_file = "data/heart+disease/output/signed.cleveland.data.csv";

    // Generate a new keypair for signing
    let sk = SigningKey::random(&mut OsRng);
    let public_key = sk.verifying_key().to_encoded_point(true);

    // Open input CSV file
    let input = File::open(input_file)?;
    let reader = BufReader::new(input);

    // Create output CSV file
    let output = File::create(output_file)?;
    let mut writer = Writer::from_writer(BufWriter::new(output));

    // Iterate over lines in the input CSV file
    for line in reader.lines() {
        let line = line?;

        let signature: Signature = sk.sign(line.as_bytes());

        // Write line, signature, and public key to output CSV file
        writer.serialize((
            public_key,
            hex::encode(&signature.to_vec()),
            line,
        ))?;
    }

    // Flush and close output CSV file
    writer.flush()?;

    println!("CSV processing complete.");
    Ok(())
}
