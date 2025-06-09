<H1> Introducing Crust: One Of The Crustiest Languages Of All Time </H1>
This is Crust, a statically and strongly typed programming language written in Rust 🦀. Crust intends to be a simple yet powerful language, which should be easy to pick up without compromising features. You could think of it as a mix between Rust and C, as it takes many of the good parts of Rust, and combines them with the simplicity of C.

<H2>Why should you use Crust?</H2>

- 🚀 Blazingly fast and written in Rust
- 😃 You are looking for a language that balances simplicity and features
- 🥰 You want to support me

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
    Vector2D vec = Vector2D(3., 2.);
    println(vec.x); // prints '3.'
    // call methods with the syntax 'struct_name.method_name(arguments)'
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

// define a struct named 'Vector2D'
struct Vector2D {
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

<H2>Roadmap</H2>

- Expand standard library
- Add modules
- Add pattern matching (match/switch)
- Add enums
- Add encapsulation
- ✅ Add some syntactic sugar for mutating variables (+=, -=, *=, /=)
- ✅ Add casting operations with the 'as' keyword
- ✅ Add more integer types such as i64 and u64
- ✅ Add dynamic arrays to the standard library (Vec)
- ✅ Add methods
- ✅ Add structs
- ✅ Add garbage collection
- ✅ Add arrays
- ✅ Add functions
- ✅ Add loops
- ✅ Add if statements
- ✅ Add variables
