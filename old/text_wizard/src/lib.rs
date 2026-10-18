// src/lib.rs
use pyo3::prelude::*;
use std::collections::HashSet;

#[pyfunction]
fn count_unique_words(text: String) -> usize {
    let words: HashSet<&str> = text.split_whitespace().collect();
    words.len()
}

#[pymodule]
fn text_wizard(m: &Bound<'_, PyModule>) -> PyResult<()> {   // ← &PyModule নয়, &Bound<'_, PyModule>
    m.add_function(wrap_pyfunction!(count_unique_words, m)?)?;
    Ok(())
}