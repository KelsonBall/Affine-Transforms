# Affine Transforms

This package is in development and isn't yet fully documented for public use.

## Design principles

There are a couple of design goals I have with this project:

 1. Everything is immutable (for example, multiplying one vector by another always gives you a new vector)
    * I'm looking at you, [PVector](https://processing.org/reference/PVector.html)
 2. Everything is 3D. If you want 2D - use the XY plane.
    * It's super easy to go from 2D to 3D is you were secretly already in 3D!
 3. Specific. This is a 3D linear transforms library, not a linear algebra library
    * For example, matrix multiplication and inversion are only handled for the case of 4x4 affine matrices