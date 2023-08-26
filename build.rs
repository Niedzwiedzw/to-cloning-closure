use itertools::Itertools;
use std::{
    error::Error,
    fmt::Display,
    iter::{empty, once},
    num::NonZeroUsize,
    path::Path,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("IO error occurred: {source}")]
    IO {
        #[from]
        source: std::io::Error,
    },
}

type Result<T> = std::result::Result<T, BuildError>;

#[derive(Clone, Copy)]
pub struct Trait {
    closure: Closure,
}

impl Trait {
    pub fn trait_name(&self) -> String {
        let Self {
            closure: Closure { args },
        } = self;
        format!("ToClonedClosure{args}Args")
    }
    pub fn method_name(&self) -> String {
        let Self {
            closure: Closure { args },
        } = self;
        format!("with_cloned_{args}")
    }
}

impl Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { closure } = self;
        let trait_name = self.trait_name();
        let method_name = self.method_name();
        let parameters = closure.as_generic_parameters().join(", ");
        write!(
            f,
            r#"
    pub trait {trait_name}: Sized {{
        fn {method_name}<R, B, {parameters}>(&self, builder: B) -> Box<dyn (Fn({parameters}) -> R)>
        where
            B: (Fn(Self, {parameters}) -> R) + Copy + 'static;
    }}
"#
        )
    }
}

#[derive(Clone, Copy)]
pub struct Tuple {
    element_count: usize,
}

impl Tuple {
    pub fn as_generic_parameters(&self) -> impl Iterator<Item = String> {
        (0..self.element_count).map(|idx| format!("A{idx}"))
    }
}

#[derive(Clone, Copy)]
pub struct Closure {
    args: usize,
}

impl Closure {
    pub fn as_generic_parameters(&self) -> impl Iterator<Item = String> {
        (0..self.args).map(|idx| format!("C{idx}"))
    }
}

impl Trait {
    fn impl_for(&self, tuple: &Tuple) -> String {
        let closure = &self.closure;
        let tuple_generic_parameters = tuple.as_generic_parameters().join(", ");
        let closure_generic_parameters = closure.as_generic_parameters().join(", ");
        // let impl_generic_parameters = empty()
        //     .chain(tuple.as_generic_parameters())
        //     .chain(closure.as_generic_parameters())
        //     .join(",");
        let trait_name = self.trait_name();
        let method_name = self.method_name();
        let tuple_where_clause = tuple
            .as_generic_parameters()
            .map(|arg| format!("{arg}: Clone + 'static"))
            .join(",\n");

        let cloned_tuple_generic_parameters = tuple
            .as_generic_parameters()
            .map(|arg| format!("{arg}.clone()"))
            .join(", ");
        format!(
            r#"
    #[allow(non_snake_case, unused_parens)]
    impl<{tuple_generic_parameters}> {trait_name} for ({tuple_generic_parameters},)
    where
        {tuple_where_clause}
    {{
        fn {method_name}<R, B, {closure_generic_parameters}>(&self, builder: B) -> Box<dyn (Fn({closure_generic_parameters}) -> R)>
        where
            B: (Fn(Self, {closure_generic_parameters}) -> R) + Copy + 'static,
        {{
            let ({tuple_generic_parameters},) = self;
            let ({tuple_generic_parameters}, )= ({cloned_tuple_generic_parameters},);
            Box::new(move |{closure_generic_parameters}| {{
                let tuple = ({cloned_tuple_generic_parameters},);
                builder(tuple, {closure_generic_parameters})
            }})
        }}
    }}
"#
        )
    }
}

fn generate_impls(max_tuple_size: usize, max_closure_parameters: usize) -> String {
    (0..max_closure_parameters)
        .flat_map(|closure_size| {
            let trait_for = Trait {
                closure: Closure { args: closure_size },
            };

            once(trait_for.to_string()).chain((1..max_tuple_size).map(move |tuple_element_count| {
                trait_for.impl_for(&Tuple {
                    element_count: tuple_element_count,
                })
            }))
        })
        .join("\n\n")
}

fn main() -> Result<()> {
    let generated = Path::new("./src/generated.rs");
    std::fs::write(generated, generate_impls(10, 10)).map_err(Into::into)
}
