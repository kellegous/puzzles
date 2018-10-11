### Find all divisors

The challenge here is to find all the divisors of a 32-bit integer. The results must be returned sorted in increasing order.

The `large_number` test also prints out the time it takes to find the divisors of a larger number. Sadly, rust stable doesn't support benchmark tests, so you should consider running the tests with:

```
cargo test --release -- --nocapture
```

This will run the tests without capturing and suppressing `stdout`, so you can see how long it takes to call your function on a larger number.
