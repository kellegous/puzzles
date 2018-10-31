# Intermediate Rust FFI Challenge 

Given a rust array of `i32`s, sort the array using the standard C `qsort` function.

For reference here is the function signature of `qsort`:

```c
void qsort( void *ptr, size_t count, size_t size,
            int (*comp)(const void *, const void *) );
```

More documentation [here](https://en.cppreference.com/w/c/algorithm/qsort).

This challenge requires calling an unsafe C function from Rust, handling
pointers from C in Rust, making a Rust function callable from C, and passing a Rust
array to a C function.

## Starter Code

```rust
fn main() {
    let mut data = [3i32, 2, 1, 4];
    unsafe {
        // qsort(...);
    }
    println!("sorted: {:?}", data); // sorted: [1, 2, 3, 4]
}
```

## Bonus Points

- Add a safe interface for `qsort`.
- Sort an array of strings instead of an array of integers.
- Make a generic safe qsort function (this will require specialization).
