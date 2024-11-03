#[derive(Debug, PartialEq, Clone)]
pub enum StanModelBlockType {
    Functions,
    Data,
    TransformedData,
    Parameters,
    TransformedParameters,
    Model,
    GeneratedQuantities,
}
