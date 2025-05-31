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

#[test]
fn continue_loop() {
    let crust_file = "tests/continue.crs";

    let output = Command::new("target/debug/crust")
        .arg(crust_file)
        .output()
        .expect("Failed to run Crust interpreter");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Crust stderr:\n{}", stderr);
    }

    assert_eq!(stdout.trim(), "123458910");
}

#[test]
fn break_loop() {
    let crust_file = "tests/break.crs";

    let output = Command::new("target/debug/crust")
        .arg(crust_file)
        .output()
        .expect("Failed to run Crust interpreter");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Crust stderr:\n{}", stderr);
    }

    assert_eq!(stdout.trim(), "0123");
}

#[test]
fn arr_print_2d() {
    let crust_file = "tests/2d_arr_print.crs";

    let output = Command::new("target/debug/crust")
        .arg(crust_file)
        .output()
        .expect("Failed to run Crust interpreter");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("Crust stderr:\n{}", stderr);
    }

    assert_eq!(stdout.trim(), "1234");
}
