extern crate rusty_machine;

use rusty_machine::learning::lin_reg::LinRegressor;
use rusty_machine::learning::SupModel;
use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::Vector;

pub fn testing() {
    let inputs = Matrix::new(4, 1, vec![1.0, 3.0, 5.0, 7.0]);
    let targets = Vector::new(vec![1., 5., 9., 13.]);

    let mut lin_mod = LinRegressor::default();

// Train the model
    lin_mod.train(&inputs, &targets).unwrap();

// Now we'll predict a new point
    let new_point = Matrix::new(1, 1, vec![10.]);
    let output = lin_mod.predict(&new_point).unwrap();

    println!("{:?}", output);

// Hopefully we classified our new point correctly!
    assert!(output[0] > 17f64, "Our regressor isn't very good!");
}
