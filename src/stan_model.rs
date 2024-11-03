#[derive(Debug, PartialEq)]
pub struct StanModel {
    pub functions: Option<Vec<String>>,
    pub data: Vec<String>,
    pub transformed_data: Option<Vec<String>>,
    pub parameters: Vec<String>,
    pub transformed_parameters: Option<Vec<String>>,
    pub model: Vec<String>,
    pub generated_quantities: Option<Vec<String>>,
}

impl StanModel {
    pub fn new() -> StanModel {
        StanModel {
            functions: None,
            data: Vec::new(),
            transformed_data: None,
            parameters: Vec::new(),
            transformed_parameters: None,
            model: Vec::new(),
            generated_quantities: None,
        }
    }

    pub fn add_function(&mut self, function: &str) {
        match &mut self.functions {
            Some(functions) => functions.push(function.to_string()),
            None => self.functions = Some(vec![function.to_string()]),
        }
    }

    pub fn add_data(&mut self, data: &str) {
        self.data.push(data.to_string());
    }

    pub fn add_transformed_data(&mut self, data: &str) {
        match &mut self.transformed_data {
            Some(transformed_data) => transformed_data.push(data.to_string()),
            None => self.transformed_data = Some(vec![data.to_string()]),
        }
    }

    pub fn add_parameter(&mut self, parameter: &str) {
        self.parameters.push(parameter.to_string());
    }

    pub fn add_transformed_parameter(&mut self, parameter: &str) {
        match &mut self.transformed_parameters {
            Some(transformed_parameters) => transformed_parameters.push(parameter.to_string()),
            None => self.transformed_parameters = Some(vec![parameter.to_string()]),
        }
    }

    pub fn add_model(&mut self, model: &str) {
        self.model.push(model.to_string());
    }

    pub fn add_generated_quantities(&mut self, quantity: &str) {
        match &mut self.generated_quantities {
            Some(generated_quantities) => generated_quantities.push(quantity.to_string()),
            None => self.generated_quantities = Some(vec![quantity.to_string()]),
        }
    }

    fn get_functions_string(&self) -> Option<Vec<String>> {
        self.functions
            .as_ref()
            .map(|functions| functions.to_owned())
    }

    fn get_data_string(&self) -> Vec<String> {
        self.data.to_owned()
    }

    fn get_transformed_data_string(&self) -> Option<Vec<String>> {
        self.transformed_data
            .as_ref()
            .map(|transformed_data| transformed_data.to_owned())
    }

    fn get_parameters_string(&self) -> Vec<String> {
        self.parameters.to_owned()
    }

    fn get_transformed_parameters_string(&self) -> Option<Vec<String>> {
        self.transformed_parameters
            .as_ref()
            .map(|transformed_parameters| transformed_parameters.to_owned())
    }

    fn get_model_string(&self) -> Vec<String> {
        self.model.to_owned()
    }

    fn get_generated_quantities_string(&self) -> Option<Vec<String>> {
        self.generated_quantities
            .as_ref()
            .map(|generated_quantities| generated_quantities.to_owned())
    }

    pub fn stanmodel_strings(&self) -> Vec<String> {
        self.get_functions_string()
            .unwrap_or_default()
            .iter()
            .chain(self.get_data_string().iter())
            .chain(
                self.get_transformed_data_string()
                    .unwrap_or_default()
                    .iter(),
            )
            .chain(self.get_parameters_string().iter())
            .chain(
                self.get_transformed_parameters_string()
                    .unwrap_or_default()
                    .iter(),
            )
            .chain(self.get_model_string().iter())
            .chain(
                self.get_generated_quantities_string()
                    .unwrap_or_default()
                    .iter(),
            )
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    pub fn has_include_directive(&self) -> bool {
        self.stanmodel_strings()
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
            data: Vec::new(),
            transformed_data: None,
            parameters: Vec::new(),
            transformed_parameters: None,
            model: Vec::new(),
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

        assert_eq!(
            model.functions,
            Some(vec![
                "real foo(real x) { return x; }".to_string(),
                "real bar(real x) { return x; }".to_string()
            ])
        );
    }

    #[test]
    fn can_add_data_statement() {
        let mut model = StanModel::new();
        model.add_data("int<lower=0> N;");
        model.add_data("int y[N];");

        assert_eq!(
            model.data,
            vec!["int<lower=0> N;".to_string(), "int y[N];".to_string()]
        );
    }

    #[test]
    fn can_add_transformed_data_statement() {
        let mut model = StanModel::new();
        model.add_transformed_data("int<lower=0> N;");
        model.add_transformed_data("int y[N];");

        assert_eq!(
            model.transformed_data,
            Some(vec!["int<lower=0> N;".to_string(), "int y[N];".to_string()])
        );
    }

    #[test]
    fn can_add_parameter_statement() {
        let mut model = StanModel::new();
        model.add_parameter("real<lower=0> sigma;");

        assert_eq!(model.parameters, vec!["real<lower=0> sigma;".to_string()]);
    }

    #[test]
    fn can_add_transformed_parameter_statement() {
        let mut model = StanModel::new();
        model.add_transformed_parameter("real mu;");
        model.add_transformed_parameter("real<lower=0> sigma;");

        assert_eq!(
            model.transformed_parameters,
            Some(vec![
                "real mu;".to_string(),
                "real<lower=0> sigma;".to_string()
            ])
        );
    }

    #[test]
    fn can_add_model_statement() {
        let mut model = StanModel::new();
        model.add_model("mu ~ normal(0, 1);");

        assert_eq!(model.model, vec!["mu ~ normal(0, 1);".to_string()]);
    }

    #[test]
    fn can_add_generated_quantities_statements() {
        let mut model = StanModel::new();
        model.add_generated_quantities("real y_pred[N];");
        model.add_generated_quantities("for (n in 1:N) { y_pred[n] = normal_rng(mu, sigma); }");

        assert_eq!(
            model.generated_quantities,
            Some(vec![
                "real y_pred[N];".to_string(),
                "for (n in 1:N) { y_pred[n] = normal_rng(mu, sigma); }".to_string()
            ])
        );
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
}
