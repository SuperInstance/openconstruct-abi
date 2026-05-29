# Dependencies — openconstruct-abi

## Ecosystem Role

openconstruct-abi is the **stable interface contract** for the OpenConstruct ecosystem. It defines type schemas, wire formats, and FFI boundaries that ensure all language implementations (Rust, Python, TypeScript, etc.) can interoperate seamlessly. This is the "lingua franca" repo — everything that speaks OpenConstruct speaks ABI.

---

## Upstream Dependencies

This is a foundational package with minimal upstream dependencies. It depends on:

| Repository | Description |
|---|---|
| [openconstruct](https://github.com/SuperInstance/openconstruct) | Design specifications and version governance |

## Downstream Dependents — Language Bindings

| Repository | Description |
|---|---|
| [openconstruct-rust](https://github.com/SuperInstance/openconstruct-rust) | Rust runtime consumes ABI for FFI |
| [cocapn-py](https://github.com/SuperInstance/cocapn-py) | Python bindings via ABI |
| [cocapn-sdk](https://github.com/SuperInstance/cocapn-sdk) | SDK implements ABI interfaces |
| [cocapn-cli](https://github.com/SuperInstance/cocapn-cli) | CLI uses ABI types |
| [cocapn-core](https://github.com/SuperInstance/cocapn-core) | Core protocol implements ABI |

## Downstream Dependents — Services

| Repository | Description |
|---|---|
| [captain](https://github.com/SuperInstance/captain) | Captain agent uses ABI for orchestration messages |
| [capitaine-agent](https://github.com/SuperInstance/capitaine-agent) | Capitaine agent uses ABI for fleet coordination |
| [plato-adapters](https://github.com/SuperInstance/plato-adapters) | Adapters implement ABI adapter interfaces |

## Documentation

- [OpenConstruct Docs](https://github.com/SuperInstance/openconstruct-docs)
- [SuperInstance Wiki](https://github.com/SuperInstance/superinstance-wiki)
