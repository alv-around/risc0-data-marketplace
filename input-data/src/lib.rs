use input_data_methods::HEART_DISEASE_DATA_ELF;
use input_data_core::{DataParsingComputation, HeartDiseaseMeasure};
use risc0_zkvm::{default_prover, ExecutorEnv, serde::from_slice};
use std::error::Error;


pub fn prove_data_parsing(data: &'static str) -> Result<DataParsingComputation, Box<dyn Error>> {
    let mut output = Vec::new();

    // Run signature verified in the zkVM guest and get the resulting receipt.
    let env = ExecutorEnv::builder()
        .write(&data)?
        .stdout(&mut output)
        .build()?;

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let proof = prover.prove(env, HEART_DISEASE_DATA_ELF)?;
    println!("Proving Successfully! receipt generated");


    let measures: Vec<HeartDiseaseMeasure> = from_slice(&output)?;
    
    Ok(DataParsingComputation{ 
        receipt: proof.receipt,
        measures,
    })
}