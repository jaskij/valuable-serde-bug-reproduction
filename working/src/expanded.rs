#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::anyhow;
use valuable_derive::Valuable;
use valuable_serde::Serializable;
pub struct ValuableAnyhow {
    message: String,
    root_cause: String,
    chain: Vec<String>,
}
#[allow(non_upper_case_globals)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    static ValuableAnyhow_FIELDS: &[::valuable::NamedField<'static>] = &[
        ::valuable::NamedField::new("message"),
        ::valuable::NamedField::new("root_cause"),
        ::valuable::NamedField::new("chain"),
    ];
    impl ::valuable::Structable for ValuableAnyhow {
        fn definition(&self) -> ::valuable::StructDef<'_> {
            ::valuable::StructDef::new_static(
                "ValuableAnyhow",
                ::valuable::Fields::Named(ValuableAnyhow_FIELDS),
            )
        }
    }
    impl ::valuable::Valuable for ValuableAnyhow {
        fn as_value(&self) -> ::valuable::Value<'_> {
            ::valuable::Value::Structable(self)
        }
        fn visit(&self, visitor: &mut dyn ::valuable::Visit) {
            visitor.visit_named_fields(&::valuable::NamedValues::new(
                ValuableAnyhow_FIELDS,
                &[
                    ::valuable::Valuable::as_value(&self.message),
                    ::valuable::Valuable::as_value(&self.root_cause),
                    ::valuable::Valuable::as_value(&self.chain),
                ],
            ));
        }
    }
};
impl From<&anyhow::Error> for ValuableAnyhow {
    fn from(error: &anyhow::Error) -> Self {
        ValuableAnyhow {
            message: error.to_string(),
            root_cause: {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&error.root_cause())],
                ));
                res
            },
            chain: error.chain().skip(1).map(|v| v.to_string()).collect(),
        }
    }
}
impl From<anyhow::Error> for ValuableAnyhow {
    fn from(error: anyhow::Error) -> Self {
        ValuableAnyhow {
            message: error.to_string(),
            root_cause: {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[""],
                    &[::core::fmt::ArgumentV1::new_debug(&error.root_cause())],
                ));
                res
            },
            chain: error.chain().skip(1).map(|v| v.to_string()).collect(),
        }
    }
}
fn main() {
    let err = {
        let error =
            ::anyhow::private::format_err(::core::fmt::Arguments::new_v1(&["asdf FOO"], &[]));
        error
    };
    let ser = Serializable::new(ValuableAnyhow::from(err));
    match serde_json::to_string(&ser) {
        Ok(json) => {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["JSON: ", "\n"],
                &[::core::fmt::ArgumentV1::new_display(&json)],
            ));
        }
        Err(e) => {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["FAILED: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&e)],
            ));
        }
    }
}
