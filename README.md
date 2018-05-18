# Rust.Positron.Transforms

This is a linear transformations library implemented in Rust. It is intended to be the backbone of the Positron app engine, which is my dream app engine designed for deploying from fluent, efficient, flexible desktop technologies to any platform (the opposite of electron).

This project is in the very early stages and is not yet intended for public use. 

## Descriptive tests

I strive for 100% code coverage in my unit tests, and for the tests to be descriptive examples for users. 

Instead of abstracting the snot out of everything in my tests, each test represents an action a user might take followed by one or more assertions that must hold true after the action. This gives each test an `if I do this, then this is the result` structure that makes them useful for learning how the library is intended to be consumed.

## Design principles

There are a couple of design goals I have with this project:

 1. Everything is immutable (for example, multiplying one vector by another always gives you a new vector)
 2. Everything is 3D. If you want 2D - use the XY plane.
    * It's super easy to go from 2D to 3D is you were secretly already in 3D!
 3. Specific. This is a 3D linear transforms library, not a linear algebra library
    * For example, matrix multiplication and inversion are only handled for the case of 4x4 affine matrices

## Contributions

 I ♥ you

 If you'd like to contribute code, let me know by opening an issue. 

 If you know a ton about rust please show me all the dumb stuff I've done. I'm very new to this language and would love to have a mentor!