```c
fn factorial(int n): int {
    if n <= 1 return 1;
    return n * factorial(n - 1);
}

fn greet_user(): str {
    str greeting = "Welcome to Crust!";
    return greeting;
}

fn fib(int n): int {
  if n < 2 return n;
  return fib(n - 1) + fib(n - 2);
}

fn main() {
    int num = 5;
    int fact = factorial(num);

    str msg = greet_user();

    println(msg);

    print("Number: ");
    println(num);

    print("Factorial: ");
    println(fact);
}

main();
```
