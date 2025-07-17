# 📚 Documentation

This document provides a detailed explanation of Crust's features.  
For a simpler overview, check out the **Sneak Peek** section in the [README](../README.md).

---

## 🔤 Datatypes

Crust supports several built-in datatypes:

- **`null`**  
  The default value of uninitialized variables.

- **`bool`**  
  Represents a boolean value — either `true` or `false`.

- **`str`**  
  A heap-allocated string.

- **`int`**  
  A 64-bit signed integer.

- **`uint`**  
  A 64-bit unsigned integer.

- **`double`**  
  A 64-bit floating-point number.
  To represent a double place a dot after the number, e.g. write `3.` instead of `3`
  

## Variables 

Variables can be declared and defined with C-style syntax:
```rs
int x = 3; // define a signed integer named x which holds the value '3'.

bool y;    // define a bool named y which holds 'null'.
y = true;  // y now holds 'true'.
```

## Control flow 

If statementents work in a similiar way to most modern languages, however they only accept booleans as condition. 
Crust gives an error for any other value.
```rs
uint number = 5;
if number > 5 {
    println("Number is higher than 5.");
} else if number < 5 {
    println("Number is less than 5.");
} else {
    println("Number is equal to 5.");
}

if true println("true."); // The body doesn't need curly braces if it is only one statement.

if null {
    // throws compile-time error.  
    println("null found.");
}
```
While loops also work as you'd expect, except similiarly to if statements they also only accept booleans as condition.
```rs
uint i = 0;
while i < 10 {   // curly braces can be omitted.
    println(i);  // prints the numbers 1 up to and including 9.
    i += 1;
}
```

For loops have special syntax that may be unfamiliar to you.
```rs
for i in 0 to 10 // curly braces can be added if needed.
    println(i);  // prints the numbers 1 up to and including 9.
```
