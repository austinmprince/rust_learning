# Rust Tutorial Notes

` basic syntax  `


Rust shadowing - allows us to create a variable with the same name as another variable (often used when converting variable from one type to another)

`match` match statement allows us to match the result of some function and execute code based on this - especially useful in enum comparison 

`let guess: u32 = guess.trim().parse().expect("Please enter a number!")` 
  this line of code sets converts a previously set variable name guess into u32 signed (exception handling is handled by the .expect statement). parsing / trimming are done to convert from string to u32 int