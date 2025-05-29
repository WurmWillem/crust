use std::process::Command;

#[test]
fn fib() {
    let crust_file = "tests/fib.crs";

    let output = Command::new("target/debug/crust")
        .arg(crust_file)
        .output()
        .expect("Failed to run Crust interpreter");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Crust stderr:\n{}", stderr);
    }

    assert_eq!(stdout.trim(), "6765");
}

#[test]
fn arr_reassignment() {
    let crust_file = "tests/arr_reassignment.crs";

    let output = Command::new("target/debug/crust")
        .arg(crust_file)
        .output()
        .expect("Failed to run Crust interpreter");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Crust stderr:\n{}", stderr);
    }

    assert_eq!(stdout.trim(), "123248");
}
