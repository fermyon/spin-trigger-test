# Spin Host Test Runner

## Build and install the test trigger

```bash
cargo build --release
spin pluginify --install
```

## Build and run the test app

### Prerequisites

- [`componentize-py`](https://github.com/bytecodealliance/componentize-py)

```bash
pip3 install componentize-py==0.11.2
```

```
cd test-app
spin build --up
```

That should discover all the functions prefixed with `test_` in all the included modules and run them.
