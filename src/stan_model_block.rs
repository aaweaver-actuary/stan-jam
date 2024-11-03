use std::slice::Iter;

use crate::stan_model_block_type::StanModelBlockType;

#[derive(Debug, PartialEq, Clone)]
pub struct StanModelBlock {
    code: Vec<String>,
    block_type: StanModelBlockType,
}

impl StanModelBlock {
    pub fn new(block_type: StanModelBlockType) -> StanModelBlock {
        StanModelBlock {
            code: Vec::new(),
            block_type,
        }
    }

    pub fn add(&mut self, line: &str) {
        self.code.push(line.to_string());
    }

    pub fn get_code(&self) -> Iter<String> {
        self.code.iter()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let model1 = StanModelBlock::new(StanModelBlockType::Data);
        let model2 = StanModelBlock {
            code: Vec::new(),
            block_type: StanModelBlockType::Data,
        };

        assert_eq!(model1, model2);
    }

    #[test]
    fn test_add_line() {
        let mut model = StanModelBlock::new(StanModelBlockType::Parameters);
        model.add("real y;");
        model.add("real x;");
        assert_eq!(
            model.code,
            ["real y;", "real x;"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn get_code_returns_an_iterator_over_the_lines_of_code() {
        let mut model = StanModelBlock::new(StanModelBlockType::Model);
        model.add("  y ~ normal(0, 1);");
        model.add("  x ~ normal(0, 1);");

        let code = model.get_code();
        let code_vec = model.code.iter();

        assert_eq!(
            code.collect::<Vec<&String>>(),
            code_vec.collect::<Vec<&String>>()
        );
    }
}
