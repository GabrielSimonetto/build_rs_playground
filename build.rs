// https://blog.cyplo.dev/posts/2018/12/generate-rust-tests-from-data/

use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// build script's entry point
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=tests/data");

    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("generated_tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    // write test file header, put `use`, `const` etc there
    write_header(&mut test_file);

    let test_data_directories = read_dir("./tests/data/").unwrap();

    for directory in test_data_directories {
        write_test(&mut test_file, &directory.unwrap());
    }
}

fn write_test(test_file: &mut File, directory: &DirEntry) {
    let directory = directory.path().canonicalize().unwrap();
    let path = directory.display();
    let test_name = format!("test_{}", directory.file_name().unwrap().to_string_lossy());

    write!(
        test_file,
        include_str!("./tests/test_template"),
        name = test_name,
        path = path
    )
    .unwrap();
}

fn write_header(test_file: &mut File) {
    write!(
        test_file,
        r#"
        use build_rs_playground::functionality_under_test_change_a_to_o;
        "#
    )
    .unwrap();
}
