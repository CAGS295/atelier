
///
/// Experiment tracking
/// 
///

#[derive(Debug)]
struct Parameter {
    id: u32,
    name: String,
    value: (),
}

#[derive(Debug)]
struct Experiment {
    id: u32,
    name: String,
    hyper_params: Vec<T>,
}
