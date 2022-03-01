pub mod valuable_anyhow;

use anyhow::anyhow;
use valuable_anyhow::ValuableAnyhow;
use valuable_serde::Serializable;

fn main() {
    let err = anyhow!("asdf FOO");
    let ser = Serializable::new(ValuableAnyhow::from(err));
    match serde_json::to_string(&ser) {
        Ok(json) => println!("JSON: {}", json),
        Err(e) => println!("FAILED: {:?}", e),
    }
}
