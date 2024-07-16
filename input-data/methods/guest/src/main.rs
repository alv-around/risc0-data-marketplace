#![no_main]

use k256::EncodedPoint;
use k256::ecdsa::{signature::Verifier, VerifyingKey, Signature};
use csv::{ReaderBuilder, StringRecord};

use hex;

use input_data_core::HeartDiseaseMeasure;

// use risc0_zkvm::examples::merkle::{MerkleTree, Node};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


fn main() {

    let data: String = env::read();
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());

    let mut measures: Vec<HeartDiseaseMeasure> = vec![];
    for result in csv_reader.deserialize() {
        match result {
            Ok(record) => {
                let record: Vec<String> = record;

                // Parse the fields of the CSV record
                let public_key_str = &record[0];
                let signature_str = &record[1];
                let message = record[2].clone();

                // Decode the hexadecimal strings to bytes
                let public_key_bytes = hex::decode(public_key_str)
                    .expect("ECDSA pk bytes decoding failed");
                let signature_bytes = hex::decode(signature_str)
                    .expect("ECDSA signature bytes decoding failed");

                // Deserialize the bytes into k256 types
                let public_key = EncodedPoint::from_bytes(&public_key_bytes)
                    .expect("ECDSA pk encoding failed");
                let signature = Signature::from_slice(&signature_bytes)
                    .expect("ECDSA signature encoding failed");

                let verifying_key = VerifyingKey::from_encoded_point(&public_key).unwrap();
                
                
                verifying_key
                    .verify(&message.as_bytes(), &signature)
                    .expect("ECDSA signature verification failed");

                let record = StringRecord::from(message.split(',').map(|item| item.to_owned()).collect::<Vec<_>>());
                let heart_disease: HeartDiseaseMeasure = record.deserialize(None).unwrap();
                eprintln!("{:?}", heart_disease);

                measures.push(heart_disease);

            }
            Err(err) => {
                eprintln!("Error deserializing record: {:?}", err);
            }
        }
    }

    env::write(&measures);

}