use pipes::Pipeline;

#[test]
fn test_pipeline() {

    let pipe = Pipeline::new(
        vec![
            MinMaxScaler::new()
        ]
    );

}