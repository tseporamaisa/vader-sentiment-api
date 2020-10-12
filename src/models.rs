use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct VaderScores {
    pub text: String,
    pub neg: f64,
    pub neu: f64,
    pub pos: f64,
    pub compound: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestText {
    pub text: Vec<String>,
}
