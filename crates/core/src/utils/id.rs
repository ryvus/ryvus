use rand::{Rng, distr::Alphanumeric};

pub fn generate_id(prefix: &str) -> String {
    let suffix: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    format!("{}_{}", prefix, suffix)
}
