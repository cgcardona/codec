# Code

Library for encoding and decoding [EARTH](https://www.earth.engineering) addresses.

```rust
// encode base58check address
use Address::*;

```

## Documentation

To generate docs run the following command from within the base directory:

```
 cargo doc --no-deps --open
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Opening /Users/username/codec/target/doc/codec/index.html
```

## Contributing

First, clone [the repo](https://github.com/cgcardona/codec) and change directories to the freshly cloned code.

```
git clone https://github.com/cgcardona/codec.git
cd codec/
```

Next install deps and build app

```
cargo build
```

Create a feature branch off of [stage](https://github.com/cgcardona/codec/tree/stage) and then reate a [Pull Request](https://github.com/cgcardona/codec/pulls) to merge your code back in to `stage`.

## Credit

Originally forked from the wonderful [bitcoincash-addr](https://crates.io/crates/bitcoincash-addr)
