# DualHash

**DualHash** is a conservative, combined hash function that strengthens cryptographic hashing by composing two independent primitives:

```
DualHash-1024(M) = SHA3-512(0x01 || M) || BLAKE3-512(0x02 || M)
DualHash-512-trunc(M) = SHA3-512(0x01 || M) XOR BLAKE3-512(0x02 || M)
```

## Features
- `dualhash1024` → 1024-bit digest (128 bytes)
- `dualhash512_trunc` → 512-bit digest (64 bytes)
- CLI enabled by default (`cargo install dualhash` will install the CLI)

## Example
```rust
use dualhash::{dualhash1024, dualhash512_trunc};

fn main() {
    let msg = b"abc";
    let d1 = dualhash1024(msg);
    let d2 = dualhash512_trunc(msg);
    println!("DualHash-1024: {}", hex::encode(d1));
    println!("DualHash-512-trunc: {}", hex::encode(d2));
}
```

## CLI
Install:
```bash
cargo install --path . --features cli
```

Usage:
```bash
dualhash "hello world"
dualhash "abc" --trunc
```

## License
DualHash is available under **MIT OR Apache-2.0**.

Repository: https://github.com/TooNsmk/dualhash
