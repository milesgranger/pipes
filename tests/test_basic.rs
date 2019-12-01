use pipes::{MinMaxScaler, Pipeline};

#[test]
fn test_pipeline() {
    let pipe = Pipeline::new(vec![Box::new(MinMaxScaler::new(0, 1))]);
}
