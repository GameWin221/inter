# Program structure
```
global {
    // Imports
    // Global variables
}

// Your function declarations:
/// EXAMPLE:
func say_hello() {
    print("Hello World!);
}

// The app's entry point:
int main() {
    /// EXAMPLE:
    say_hello();
}
```

# Variables
A variable must always be inside a scope.

### Available types:
- real - Real number (in mathematical terms, natively it's a c++ float)
- int - Integer number (in mathematic terms, natively it's a c++ int)
- bool - Boolean
- string - String (Natively it's c++ std::string)
- func - Function (Natively it's c++ void)

# Comments
The hash `#` operator starts and ends a comment:

```
# This is a comment #
int a = 123; # Set a to 123 #
```
# Operators
### Available operators:

### Operator combinations:


# Compilation

# Importing
You can import files **only** in the `global` region.
```
global {
    import std_io;        // You can now use print
    import std_graphics;  // You can now use graphical features
}
```