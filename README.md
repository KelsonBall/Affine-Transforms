
# Rust.Positron.Transforms

This is a linear transformations library implemented in Rust. It is intended to be the backbone of the Positron app engine, which is my dream app engine designed for deploying from fluent, efficient, flexible desktop technologies to any platform (the opposite of electron).

This project is in the very early stages and is not yet intended for public use. 

## Descriptive tests

I strive for 100% code coverage in my unit tests, and for the tests to be descriptive examples for users. 

Instead of abstracting the snot out of everything in my tests, each test represents an action a user might take followed by one or more assertions that must hold true after the action. This gives each test an `if I do this, then this is the result` structure that makes them useful for learning how the library is intended to be consumed.