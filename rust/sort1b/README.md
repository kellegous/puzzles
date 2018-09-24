### Sort 1 Billion Bytes

The challenge here is to sort 1 billion bytes quickly.

Note that to run the `one_billion` test, you will definitely want to run tests with a `release` build to ensure it finishes in a reasonable amount of time. To do that use

```
cargo test --release
```

I would also recommend commenting out the `one_billion` test and getting the other, simpler tests to pass first.