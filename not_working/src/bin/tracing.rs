#[path = "../valuable_anyhow.rs"]
mod valuable_anyhow;

use anyhow::anyhow;
use tracing::info;
use valuable::Valuable;
use valuable_anyhow::ValuableAnyhow;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stdout)
        .json()
        .init();

    let err = anyhow!("asdf FOO");
    let va = ValuableAnyhow::from(err);
    info!(test=va.as_value());
}
