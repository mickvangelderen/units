Demonstration of how one can use newtypes to prevent mixing up different quantities. In other words,
a [unit-of-measurement](https://en.wikipedia.org/wiki/Unit_of_measurement) library.

This is not a production ready library. If you are looking for that, take a look at
[`uom`](https://docs.rs/uom/latest/uom/). 

In my experience `uom` does take a while to compile, which is partially why I wanted to investigate
what it would take to implement the functionality it provides. `uom` also takes care of printing and
implementing traits from math interoperability crates. I also wanted to try and use `const N: isize`
but Rust is not ready for that (can't use them in constant expressions as of Jan 2024). I ended up
implementing [compile-time counting](./src/const_int.rs) as well to keep track of the different
dimensions of the [unit system](https://en.wikipedia.org/wiki/System_of_units_of_measurement).
