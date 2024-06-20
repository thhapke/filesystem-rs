pub mod filesystem;

// use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;
// use pyo3::types::PySequence;

// fn print_folder_tree(py: Python, paths: &PySequence, max_level: Option<usize>) {

//     let files: Vec<&str> = paths
//         .cast_iter()
//         .map(|obj| obj.extract::<String>().map(|s| s.as_str()))
//         .filter_map(Result::ok)
//         .collect();

//     let output = FileSystem::print_file_list(files, max_level);
//     Ok(output)
// }

// #[pymodule]
// fn bdc_utils(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_wrapped(wrap_pyfunction!(print_folder_tree))?;
//     Ok(())
// }

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
