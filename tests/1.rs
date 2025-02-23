use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_1() {
    // Path of the file to compile
    let prog_path = Path::new("ex/1.ob");
    // Path of the executable that will be created
    let exe_path = Path::new("ex/1");

    // Run our compiler to compile our objection program
    let compiler_bin = Path::new("target/debug/objection");
    let process = Command::new(compiler_bin)
        .arg(prog_path)
        .status()
        .expect("Failed to run compiler");
    assert!(process.success(), "Failed to compile");

    // Run the compiled program
    // The program should exit with code 152
    let process = Command::new(exe_path)
        .status()
        .expect("Failed to run compiled program");
    assert!(process.code().is_some(), "Program did not run successfully");
    let exit_code = process
        .code()
        .expect("Program did not provide an exit code");
    assert_eq!(
        exit_code, 152,
        "Program did not exit with the expected code"
    );

    // Clean up
    let _ = fs::remove_file(exe_path);
}
