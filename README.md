<div align="center">
    <h1>SNOS</h1>
    <br>
</div>

## [Overview](https://hackmd.io/@pragma/ByP-iux1T)

Rust Library for running the Starknet OS via the [Cairo VM](https://github.com/lambdaclass/cairo-vm).

## Test Setup

**Cairo [Env](https://docs.cairo-lang.org/0.12.0/quickstart.html)**

```bash
./scripts/setup-tests.sh

cargo test
```

### Reset Tests

```bash
./scripts/teardown-tests.sh
```

### Debug Single Cairo Program

```bash
./scripts/debug-hint.sh load_deprecated_class
```
    