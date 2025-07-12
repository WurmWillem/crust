use std::process::Command;

macro_rules! create_test {
    ($func_name: ident, $test_name: expr, $output: expr) => {
        #[test]
        fn $func_name() {
            let crust_file = format!("tests/{}.crs", $test_name);

            let output = Command::new("target/debug/crust")
                .arg(crust_file)
                .output()
                .expect("Failed to run Crust interpreter");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if !stderr.is_empty() {
                eprintln!("Crust stderr:\n{}", stderr);
            }

            assert_eq!(stdout.trim(), $output);
        }
    };
}

create_test!(fib, "fib", "6765");
create_test!(arr_reassignment, "arr_reassignment", "123248");
create_test!(continue_loop, "continue", "123458910");
create_test!(break_loop, "break", "0123");
create_test!(arr_print_2d, "2d_arr_print", "1234");
create_test!(fields, "fields", "3 2\n1");
create_test!(methods, "methods", "6\n4\n\"hoi\"");
create_test!(vec, "vec", "2\n3\n[1, 2]\n[1, 2, 4]\n3");
create_test!(mult_insts, "mult_insts", "0\n10\n1\n11");
