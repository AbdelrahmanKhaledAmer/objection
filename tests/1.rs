use std::fs;
use std::path::Path;
use std::process::Command;

fn compile_and_run(prog_path: &Path, exe_path: &Path) -> i32 {
    // Run our compiler to compile our objection program
    let compiler_bin = Path::new("target/debug/objection");
    let process = Command::new(compiler_bin)
        .arg(prog_path)
        .status()
        .expect("Failed to run compiler");
    assert!(process.success(), "Failed to compile");

    // Run the compiled program
    let process = Command::new(exe_path)
        .status()
        .expect("Failed to run compiled program");
    assert!(process.code().is_some(), "Program did not run successfully");
    process
        .code()
        .expect("Program did not provide an exit code")
}

#[test]
fn test_1() {
    // Path of the file to compile
    let prog_path = Path::new("ex/1.ob");
    // Path of the executable that will be created
    let exe_path = Path::new("ex/1");

    let exit_code = compile_and_run(prog_path, exe_path);
    assert_eq!(
        exit_code, 152,
        "Program did not exit with the expected code"
    );

    // Clean up
    let _ = fs::remove_file(exe_path);
}

#[test]
fn test_2() {
    // Path of the file to compile
    let prog_path = Path::new("ex/2.ob");
    // Path of the executable that will be created
    let exe_path = Path::new("ex/2");

    let exit_code = compile_and_run(prog_path, exe_path);
    assert_eq!(
        exit_code, 140,
        "Program did not exit with the expected code"
    );

    // Clean up
    let _ = fs::remove_file(exe_path);
}
