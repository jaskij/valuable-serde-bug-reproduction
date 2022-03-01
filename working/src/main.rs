use anyhow::anyhow;
use valuable_derive::Valuable;
use valuable_serde::Serializable;

#[derive(Valuable)]
pub struct ValuableAnyhow {
    message: String,
    root_cause: String,
    chain: Vec<String>,
}

impl From<&anyhow::Error> for ValuableAnyhow {
    fn from(error: &anyhow::Error) -> Self {
        ValuableAnyhow {
            message: error.to_string(),
            root_cause: format!("{:?}", error.root_cause()),
            chain: error.chain().skip(1).map(|v| v.to_string()).collect(),
        }
    }
}

impl From<anyhow::Error> for ValuableAnyhow {
    fn from(error: anyhow::Error) -> Self {
        ValuableAnyhow {
            message: error.to_string(),
            root_cause: format!("{:?}", error.root_cause()),
            chain: error.chain().skip(1).map(|v| v.to_string()).collect(),
        }
    }
}

fn main() {
    let err = anyhow!("asdf FOO");
    let ser = Serializable::new(ValuableAnyhow::from(err));
    match serde_json::to_string(&ser) {
        Ok(json) => println!("JSON: {}", json),
        Err(e) => println!("FAILED: {:?}", e),
    }
}
