# esent #
Contains function definitions for the Windows API library esent (the "Extensible Storage Engine", also known as ESEDB). See winapi for types and constants.

```toml
[dependencies]
esent-sys = "0.1.0"
```

```rust
extern crate esent;
```

[Documentation](https://retep998.github.io/doc/esent/)

Notes:
- This code is based on the MSDN documentation and `esent.h` in Windows 10. Consult the documentation if you are using this on an older system, as not all functions are available on all OS versions.
- Only the Unicode version of functions have been provided. (You shouldn't be using the ANSI versions anyway.)
- TODO: provide definitions for all the `JET_bit...` constants.
- TODO: implement the remaining functions and their structures.
