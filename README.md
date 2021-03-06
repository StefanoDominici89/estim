# estim

Collection of lightweight statistical estimators implementing a common interface through the Estimator trait.

## Examples

```rust
let samples: std::Vec<f64> = vec![0.0, 1.0, 2.0, 3.0, 4.0];
let mut mse = estimators::MeanSquaredError::<f64>::new();
let mut avg = estimators::Average::<f64>::new();

for sample in &samples{
    mse.update(sample);
    avg.update(sample);
}

println!("AVG: {}",avg.evaluate());
println!("AVG and MSE: ({},{})",mse.average.evaluate(),mse.evaluate());
```

Current version: 0.1.0

License: MIT
