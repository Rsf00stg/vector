use super::Error as E;
use crate::{state, Expression, Object, Result, TypeDef, Value};

#[derive(thiserror::Error, Clone, Debug, PartialEq)]
pub enum Error {
    #[error("undefined variable: {0}")]
    Undefined(String),
}

#[derive(Debug, Clone)]
pub struct Variable {
    ident: String,
}

impl Variable {
    pub fn new(ident: String) -> Self {
        Self { ident }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Expression for Variable {
    fn execute(&self, state: &mut state::Program, _: &mut dyn Object) -> Result<Option<Value>> {
        state
            .variable(&self.ident)
            .cloned()
            .ok_or_else(|| E::from(Error::Undefined(self.ident.to_owned())).into())
            .map(Some)
    }

    fn type_def(&self, state: &state::Compiler) -> TypeDef {
        state
            .variable_type(&self.ident)
            .cloned()
            // TODO: we can make it so this can never happen, by making it a
            // compile-time error to reference a variable before it is assigned.
            .unwrap_or(TypeDef {
                fallible: true,
                ..Default::default()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_type_def, value::Constraint::*, value::Kind::*};

    test_type_def![
        ident_match {
            expr: |state: &mut state::Compiler| {
                state.variable_types_mut().insert("foo".to_owned(), TypeDef::default());
                Variable::new("foo".to_owned())
            },
            def: TypeDef::default(),
        }

        exact_match {
            expr: |state: &mut state::Compiler| {
                state.variable_types_mut().insert("foo".to_owned(), TypeDef {
                    fallible: true,
                    optional: false,
                    constraint: Exact(String)
                });

                Variable::new("foo".to_owned())
            },
            def: TypeDef {
                fallible: true,
                optional: false,
                constraint: Exact(String),
            },
        }

        ident_mismatch {
            expr: |state: &mut state::Compiler| {
                state.variable_types_mut().insert("foo".to_owned(), TypeDef {
                    fallible: true,
                    ..Default::default()
                });

                Variable::new("bar".to_owned())
            },
            def: TypeDef {
                fallible: true,
                ..Default::default()
            },
        }

        empty_state {
            expr: |_| Variable::new("foo".to_owned()),
            def: TypeDef {
                fallible: true,
                ..Default::default()
            },
        }
    ];
}
