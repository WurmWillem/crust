<H2> Introducing Crust: One of the crustiest languages of all time </H2>
This is Crust, a statically and strongly typed programming language written in Rust 🦀. Crust intends to be a simple yet powerful language, which should be easy to pick up without compromising features. You could think of it as a mix between Rust and C, as it takes many of the good parts of Rust, and combines them with the simplicity of C.

<H2>Why should you use Crust?</H2>

- 🚀 Blazingly fast and written in Rust
- 😃 Because you are a masochist
- 🥰 You want to support me
- 🇳🇱 Je bent een Nederlander in hart en nieren

<H3>Sneak Peek</H3>

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
