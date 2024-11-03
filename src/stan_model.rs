use crate::stan_model_block::StanModelBlock;
use crate::stan_model_block_type::StanModelBlockType;

#[derive(Debug, PartialEq)]
pub struct StanModel {
    pub functions: Option<StanModelBlock>,
    pub data: StanModelBlock,
    pub transformed_data: Option<StanModelBlock>,
    pub parameters: StanModelBlock,
    pub transformed_parameters: Option<StanModelBlock>,
    pub model: StanModelBlock,
    pub generated_quantities: Option<StanModelBlock>,
}

impl StanModel {
    pub fn new() -> StanModel {
        StanModel {
            functions: None,
            data: StanModelBlock::new(StanModelBlockType::Data),
            transformed_data: None,
            parameters: StanModelBlock::new(StanModelBlockType::Parameters),
            transformed_parameters: None,
            model: StanModelBlock::new(StanModelBlockType::Model),
            generated_quantities: None,
        }
    }

    pub fn add_function(&mut self, function: &str) {
        match &mut self.functions {
            Some(functions) => functions.add(function),
            None => {
                self.functions = Some(StanModelBlock::new(StanModelBlockType::Functions));
                self.functions.as_mut().unwrap().add(function);
            }
        }
    }

    /// Add a line of Stan code to the data block.
    pub fn add_data(&mut self, data: &str) {
        self.data.add(data);
    }

    pub fn add_transformed_data(&mut self, data: &str) {
        match &mut self.transformed_data {
            Some(transformed_data) => transformed_data.add(data),
            None => {
                self.transformed_data =
                    Some(StanModelBlock::new(StanModelBlockType::TransformedData));
                self.transformed_data.as_mut().unwrap().add(data);
            }
        }
    }

    pub fn add_parameter(&mut self, parameter: &str) {
        self.parameters.add(parameter);
    }

    pub fn add_transformed_parameter(&mut self, parameter: &str) {
        match &mut self.transformed_parameters {
            Some(transformed_parameters) => transformed_parameters.add(parameter),
            None => {
                self.transformed_parameters = Some(StanModelBlock::new(
                    StanModelBlockType::TransformedParameters,
                ));
                self.transformed_parameters.as_mut().unwrap().add(parameter);
            }
        }
    }

    pub fn add_model(&mut self, model: &str) {
        self.model.add(model);
    }

    pub fn add_generated_quantities(&mut self, quantity: &str) {
        match &mut self.generated_quantities {
            Some(generated_quantities) => generated_quantities.add(quantity),
            None => {
                self.generated_quantities =
                    Some(StanModelBlock::new(StanModelBlockType::GeneratedQuantities));
                self.generated_quantities.as_mut().unwrap().add(quantity);
            }
        }
    }

    fn get_optional_block(
        &self,
        block: &Option<StanModelBlock>,
        block_type: StanModelBlockType,
    ) -> StanModelBlock {
        block.as_ref().map(|block| block.to_owned()).unwrap_or({
            let mut block = StanModelBlock::new(block_type);
            block.add("");
            block
        })
    }

    fn get_functions(&self) -> StanModelBlock {
        self.get_optional_block(&self.functions, StanModelBlockType::Functions)
    }

    fn get_data(&self) -> StanModelBlock {
        self.data.to_owned()
    }

    fn get_transformed_data(&self) -> StanModelBlock {
        self.get_optional_block(&self.transformed_data, StanModelBlockType::TransformedData)
    }

    fn get_parameters(&self) -> StanModelBlock {
        self.parameters.to_owned()
    }

    fn get_transformed_parameters(&self) -> StanModelBlock {
        self.get_optional_block(
            &self.transformed_parameters,
            StanModelBlockType::TransformedParameters,
        )
    }

    fn get_model(&self) -> StanModelBlock {
        self.model.to_owned()
    }

    fn get_generated_quantities(&self) -> StanModelBlock {
        self.get_optional_block(
            &self.generated_quantities,
            StanModelBlockType::GeneratedQuantities,
        )
    }

    pub fn collect_stan_model_segments(&self) -> Vec<String> {
        self.get_functions()
            .get_code()
            .chain(self.get_data().get_code())
            .chain(self.get_transformed_data().get_code())
            .chain(self.get_parameters().get_code())
            .chain(self.get_transformed_parameters().get_code())
            .chain(self.get_model().get_code())
            .chain(self.get_generated_quantities().get_code())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn has_include_directive(&self) -> bool {
        self.collect_stan_model_segments()
            .iter()
            .any(|s| s.contains("#include"))
    }
}

impl Default for StanModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_model() {
        let model1 = StanModel {
            functions: None,
            data: StanModelBlock::new(StanModelBlockType::Data),
            transformed_data: None,
            parameters: StanModelBlock::new(StanModelBlockType::Parameters),
            transformed_parameters: None,
            model: StanModelBlock::new(StanModelBlockType::Model),
            generated_quantities: None,
        };

        let model2 = StanModel::new();

        assert_eq!(model1, model2);
    }

    #[test]
    fn can_add_function_statements() {
        let mut model = StanModel::new();
        model.add_function("real foo(real x) { return x; }");
        model.add_function("real bar(real x) { return x; }");

        let mut functions_block = StanModelBlock::new(StanModelBlockType::Functions);
        functions_block.add("real foo(real x) { return x; }");
        functions_block.add("real bar(real x) { return x; }");

        assert_eq!(model.functions, Some(functions_block));
    }

    #[test]
    fn can_add_data_statement() {
        let mut model = StanModel::new();
        model.add_data("int<lower=0> N;");
        model.add_data("int y[N];");

        let mut data = StanModelBlock::new(StanModelBlockType::Data);
        data.add("int<lower=0> N;");
        data.add("int y[N];");

        assert_eq!(model.data, data);
    }

    #[test]
    fn can_add_transformed_data_statement() {
        let mut model = StanModel::new();
        model.add_transformed_data("int<lower=0> N;");
        model.add_transformed_data("int y[N];");

        let mut transformed_data_block = StanModelBlock::new(StanModelBlockType::TransformedData);
        transformed_data_block.add("int<lower=0> N;");
        transformed_data_block.add("int y[N];");

        assert_eq!(model.transformed_data, Some(transformed_data_block));
    }

    #[test]
    fn can_add_parameter_statement() {
        let mut model = StanModel::new();
        model.add_parameter("real<lower=0> sigma;");

        let mut parameter_block = StanModelBlock::new(StanModelBlockType::Parameters);
        parameter_block.add("real<lower=0> sigma;");

        assert_eq!(model.parameters, parameter_block);
    }

    #[test]
    fn can_add_transformed_parameter_statement() {
        let mut model = StanModel::new();
        model.add_transformed_parameter("real mu;");
        model.add_transformed_parameter("real<lower=0> sigma;");

        let mut transformed_parameters_block =
            StanModelBlock::new(StanModelBlockType::TransformedParameters);
        transformed_parameters_block.add("real mu;");
        transformed_parameters_block.add("real<lower=0> sigma;");

        assert_eq!(
            model.transformed_parameters,
            Some(transformed_parameters_block)
        );
    }

    #[test]
    fn can_add_model_statement() {
        let mut model = StanModel::new();
        model.add_model("mu ~ normal(0, 1);");

        let mut model_block = StanModelBlock::new(StanModelBlockType::Model);
        model_block.add("mu ~ normal(0, 1);");

        assert_eq!(model.model, model_block);
    }

    #[test]
    fn can_add_generated_quantities_statements() {
        let mut model = StanModel::new();
        model.add_generated_quantities("real y_pred[N];");
        model.add_generated_quantities("for (n in 1:N) { y_pred[n] = normal_rng(mu, sigma); }");

        let mut generated_quantities_block =
            StanModelBlock::new(StanModelBlockType::GeneratedQuantities);
        generated_quantities_block.add("real y_pred[N];");
        generated_quantities_block.add("for (n in 1:N) { y_pred[n] = normal_rng(mu, sigma); }");

        assert_eq!(model.generated_quantities, Some(generated_quantities_block));
    }

    #[test]
    fn can_tell_that_model_includes_includes_directive() {
        let mut model = StanModel::new();
        model.add_function("#include 'functions.stan'");
        model.add_data("int y[N];");
        model.add_model("mu ~ normal(0, 1);");

        assert!(model.has_include_directive());

        let mut newmodel = StanModel::new();
        newmodel.add_data("int y[N];");
        newmodel.add_model("mu ~ normal(0, 1);");
        newmodel.add_generated_quantities("#include 'generated_quantities.stan'");

        assert!(newmodel.has_include_directive());

        let mut noincludes = StanModel::new();
        noincludes.add_data("int y[N];");
        noincludes.add_model("mu ~ normal(0, 1);");
        noincludes.add_generated_quantities("real y_pred[N];");

        assert!(!noincludes.has_include_directive());
    }

    #[test]
    fn default_model_is_the_same_as_new_model() {
        let default_model = StanModel::default();
        let new_model = StanModel::new();

        assert_eq!(default_model, new_model);
    }
}
