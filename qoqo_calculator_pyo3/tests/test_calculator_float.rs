use pyo3::prelude::*;
use qoqo_calculator_pyo3::CalculatorFloatWrapper;
#[test]
fn test_initialising_calculator_float() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let python_type = py.get_type::<CalculatorFloatWrapper>();
        let new_result = python_type.call((1.0,), None).unwrap();
        let float_value = f64::extract_bound(
            &new_result
                .downcast::<CalculatorFloatWrapper>()
                .unwrap()
                .call_method0("__float__")
                .unwrap(),
        )
        .unwrap();
        assert!((float_value - 1.0).abs() < f64::EPSILON);
    })
}
