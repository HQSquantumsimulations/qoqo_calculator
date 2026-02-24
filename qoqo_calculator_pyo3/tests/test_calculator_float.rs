use pyo3::prelude::*;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
#[test]
fn test_initialising_calculator_float() {
    Python::initialize();
    Python::attach(|py| {
        let python_type = py.get_type::<CalculatorFloatWrapper>();
        let new_result = python_type.call((1.0,), None).unwrap();
        let float_value = f64::extract(
            new_result
                .cast::<CalculatorFloatWrapper>()
                .unwrap()
                .call_method0("__float__")
                .unwrap()
                .as_borrowed(),
        )
        .unwrap();
        assert!((float_value - 1.0).abs() < f64::EPSILON);
    })
}
