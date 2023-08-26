mod generated;

pub mod prelude {
    pub use super::generated::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    #[test]
    fn test_it_works() {
        let _clone = ("whatever".to_owned(),).with_cloned_0(|(whatever,)| whatever.clone());

        let append_index_to = ("whatever".to_owned(),)
            .with_cloned_1(|(whatever,), idx| format!("{whatever} - {idx}"));
        let _ = (0..2137).map(append_index_to).collect::<Vec<String>>();
    }
}
