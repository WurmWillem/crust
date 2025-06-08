<H2> Introducing Crust: One Of The Crustiest Languages Of All Time </H2>
This is Crust, a statically and strongly typed programming language written in Rust 🦀. Crust intends to be a simple yet powerful language, which should be easy to pick up without compromising features. You could think of it as a mix between Rust and C, as it takes many of the good parts of Rust, and combines them with the simplicity of C.

<H2>Why should you use Crust?</H2>

- 🚀 Blazingly fast and written in Rust
- 😃 Because you are a masochist
- 🥰 You want to support me
- 🇳🇱 Je bent een Nederlander in hart en nieren

<H3>Sneak Peek</H3>

```rs
// every program has an entry point called 'main'
fn main() {
    // print values with newline with the function 'println'
    println("Hello World!");

    // create variables with C-like syntax
    int variable = 3;

    // use functions defined later in the file, no forward declaration necessary
    factorial(5); // evaluates to 120

    // for loops (and there are while loops as well)
    for i in 0 to 10 {
        println(i); // prints the numbers 1 up to and including 9
    }

    // create arrays with the following syntax
    int[] array = [1, 2, 3];
    println(array[0]);  // prints 1

    // create instances with 'struct_name(fields)' syntax
    Vec2 vec = Vec2(3., 2.);
    // call methods with 'struct_name.method_name(arguments)'
    double product = vec.product(); // holds '6.'

    // use structs from the standard library such as Vec (dynamic array)
    Vec vec = Vec([1, 2, 3]);
    vec.push(4);
    println(vec.get(3)); // prints '4'
}

// define a function named 'factorial' that takes and returns a uint
fn factorial(uint n): uint {
    if n <= 1 return 1;
    return n * factorial(n - 1);
}

// define a struct named 'Vec2'
struct Vec2 {
    // declare the fields
    double x;
    double y;

    // define the functions
    fn product(): double {
        // use 'self.property_name' to access fields and methods
        return self.x * self.y;
    }
}
```
