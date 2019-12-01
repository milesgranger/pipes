use pipes::{MinMaxScaler, Pipeline, Transformer};

#[test]
fn test_pipeline_with_minmax() {
    let mut pipe = Pipeline::new(vec![Box::new(MinMaxScaler::new())]);

    // Fit to some data
    let x = vec![1, 2, 3, 4];
    pipe.fit(x);

    // Ensure transforming min & maxes
    let x = vec![0, 1, 5];
    let out = pipe.transform(x);
    assert_eq!(out[0], 1);
    assert_eq!(out[1], 1);
    assert_eq!(out[2], 4);
}

#[test]
fn test_pipeline_in_pipeline() {
    let mut pipe = Pipeline::new(vec![
        Box::new(Pipeline::new(vec![
            Box::new(MinMaxScaler::new())
        ]))
    ]);

    // Fit to some data
    let x = vec![1, 2, 3, 4];
    pipe.fit(x);

    // Ensure transforming min & maxes
    let x = vec![0, 1, 5];
    let out = pipe.transform(x);
    assert_eq!(out[0], 1);
    assert_eq!(out[1], 1);
    assert_eq!(out[2], 4);
}
