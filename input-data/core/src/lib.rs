
use k256::{EncodedPoint, ecdsa::Signature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HeartDiseaseRecord {
    pub public_key: EncodedPoint,
    pub signature: Signature,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HeartDiseaseMeasure {
    age: f64,
    sex: f64,
    cp: f64,
    trestbps: f64,
    chol: f64,
    fbs: f64,
    restecg: f64,
    thalach: f64,
    exang: f64,
    oldpeak: f64,
    slope: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    mayor: Option<f64>,
    #[serde(deserialize_with = "csv::invalid_option")]
    pop: Option<f64>,
    target: u8,
}