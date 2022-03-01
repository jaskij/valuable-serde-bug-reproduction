use anyhow::anyhow;
use valuable::{Fields, NamedField, NamedValues, StructDef, Structable, Valuable, Value, Visit};
use valuable_serde::Serializable;

pub struct ValuableAnyhow {
    error: anyhow::Error,
}

impl From<anyhow::Error> for ValuableAnyhow {
    fn from(error: anyhow::Error) -> Self {
        ValuableAnyhow { error }
    }
}

const VALUABLE_ANYHOW_FIELDS: &[NamedField<'static>] = &[
    NamedField::new("message"),
    NamedField::new("root_cause"),
    NamedField::new("chain"),
];

impl Valuable for ValuableAnyhow {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, v: &mut dyn Visit) {
        let chain: Vec<_> = self.error.chain().skip(1).collect();

        v.visit_named_fields(&NamedValues::new(
            VALUABLE_ANYHOW_FIELDS,
            &[
                self.error.to_string().as_str().as_value(),
                self.error.root_cause().as_value(),
                chain.as_value(),
            ],
        ));
    }
}

impl Structable for ValuableAnyhow {
    fn definition(&self) -> StructDef<'_> {
        StructDef::new_static("ValuableAnyhow", Fields::Named(VALUABLE_ANYHOW_FIELDS))
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
