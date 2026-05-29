# openconstruct-abi — The C ABI Keystone for Polyglot OpenConstruct

A C-compatible ABI (`extern "C"`) that makes OpenConstruct accessible from any language that can call C functions — C, C++, Python, Ruby, Go, Java, Zig, Swift, and more. This is the foundation every binding builds on.

**Part of [SuperInstance OpenConstruct](https://github.com/SuperInstance/OpenConstruct).**

## What This Gives You

- **Opaque session handles** — create, step through phases, generate config
- **Module registry** — list available modules with domain filtering
- **Phase-ordered onboarding** — 5-phase lifecycle enforced at the ABI level
- **Zero-copy C strings** — proper ownership transfer with `CString` allocation/free
- **JSON result payloads** — structured data returned as JSON for any language to parse

## Quick Start

```c
#include "openconstruct.h"

// Create registry and session
OcRegistry* reg = oc_registry_create();
OcSession* sess = oc_session_create(reg);

// Phase 1: Self-declaration
oc_session_declare(sess, "my-agent", "glm-5.1");

// Phase 2: Module selection
oc_session_select_modules(sess, "spectral-graph-core,plato-room");

// Phase 3: Interface selection
oc_session_choose_interface(sess, "cli");

// Phase 4: Connection setup
oc_session_connect(sess, "localhost:9142");

// Phase 5: Generate config
OcResult* result = oc_session_generate_config(sess);
printf("Config: %s\n", result->data_json);

// Cleanup
oc_result_free(result);
oc_session_free(sess);
oc_registry_free(reg);
```

## API Reference

### Lifecycle

| Function | Phase | Description |
|----------|-------|-------------|
| `oc_registry_create()` | — | Create module registry with default modules |
| `oc_registry_free()` | — | Free registry and all module strings |
| `oc_session_create(reg)` | — | Create new onboarding session |
| `oc_session_free(sess)` | — | Free session |
| `oc_session_declare(sess, name, model)` | 1 | Register agent identity |
| `oc_session_select_modules(sess, csv)` | 2 | Select modules by comma-separated IDs |
| `oc_session_choose_interface(sess, type)` | 3 | Choose interface (cli, rest-api, etc.) |
| `oc_session_connect(sess, addr)` | 4 | Connect to coordinator |
| `oc_session_generate_config(sess)` | 5 | Generate final config as JSON |

### Types

| Type | Description |
|------|-------------|
| `OcSession` | Opaque session handle with phase tracking |
| `OcRegistry` | Opaque module registry |
| `OcModule` | Module descriptor: id, domain, name, one-line description |
| `OcResult` | Result with success flag, phase, message, and JSON data |

### Phase Constants

```
OC_PHASE_SELF_DECLARATION    = 1
OC_PHASE_MODULE_SELECTION    = 2
OC_PHASE_INTERFACE_SELECTION = 3
OC_PHASE_CONNECTION_SETUP    = 4
OC_PHASE_ENVIRONMENT_GEN     = 5
```

## How It Fits
- [OpenConstruct Documentation](https://github.com/SuperInstance/openconstruct-docs) — ecosystem-wide docs and guides

This is the keystone. Every language binding in the OpenConstruct ecosystem — [C](https://github.com/SuperInstance/openconstruct-c), [Python](https://github.com/SuperInstance/openconstruct-python), [Go](https://github.com/SuperInstance/openconstruct-go), [TypeScript](https://github.com/SuperInstance/openconstruct-ts), [Ruby](https://github.com/SuperInstance/openconstruct-ruby), [Swift](https://github.com/SuperInstance/openconstruct-swift), [Zig](https://github.com/SuperInstance/openconstruct-zig), [Java](https://github.com/SuperInstance/openconstruct-java), [C#](https://github.com/SuperInstance/openconstruct-cs), [ESP32](https://github.com/SuperInstance/openconstruct-esp32), [Jetson](https://github.com/SuperInstance/openconstruct-jetson) — either calls this ABI directly or reimplements the same protocol.

## Installation

Build the static library:

```bash
cargo build --release
# Produces libopenconstruct_abi.a
```

## Testing

```bash
cargo test
```

## License

MIT
