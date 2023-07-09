use std::rc::Rc;

use anonymize_rs::anonymizer::flashtext_anonymizer::FlashTextAnonymizer;
use anonymize_rs::anonymizer::ner_anonymizer::NerAnonymizer;
use anonymize_rs::anonymizer::regex_anonymizer::RegexAnonymizer;
use anonymize_rs::anonymizer::Anonymizer;
use anonymize_rs::config::{AnonymizePipelineConfig, AnonymizerConfig};
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn rusty_method(a: usize) -> PyResult<usize> {
    Ok(a * 2)
}

#[pymodule]
fn rusty_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rusty_method, m)?)?;

    #[pyclass]
    pub struct RustyClass {
        ner: NerAnonymizer
    }

    #[pymethods]
    impl RustyClass {
        #[new]
        pub fn new() -> Self {
            todo!("TODO");
            //RustyClass {}
        }

        #[staticmethod]
        pub fn rusty_method(a: usize) -> PyResult<usize> {
            Ok(a * 2)
        }
    }

    m.add_class::<RustyClass>()?;
    Ok(())
}

/*
#[pyfunction]
fn run_apply(config_path: String) -> PyResult<String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(apply_delta(config_path))
        .unwrap();
    Ok("Ok".to_string())
}

#[pyfunction]
fn run(config_path: String, host: String, port: u16, log_level: String) -> PyResult<String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_delta_server(config_path, host, port, log_level))
        .unwrap();
    Ok("Ok".to_string())
}

#[pymodule]
fn anonymize_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(run_apply, m)?)?;

    Ok(())
}

*/
