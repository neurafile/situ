# Gadget VM Mono-Repo

Mono-repository for the Gadget Language Virtual Machine implementation.

## Structure

```
gadget-vm/
├── grammar/          # ANTLR grammar definition (.g4)
├── rust-vm/          # Rust VM implementation (compiles to WASM)
└── js-bindings/      # JavaScript/TypeScript bindings (npm package)
```

## Components

### Grammar (`grammar/`)

ANTLR grammar file defining the Gadget Language DSL syntax:
- Collider definitions
- Listener definitions  
- Action definitions
- Piano-first keywords (not free code)

### Rust VM (`rust-vm/`)

Rust implementation of the VM:
- Parser generated from ANTLR grammar (isolated, not modified)
- Minimal language processing
- Compiles to WASM for web deployment

**Building:**
```bash
cd rust-vm
make parser    # Generate parser from grammar
make wasm      # Compile to WASM
```

### JavaScript Bindings (`js-bindings/`)

npm package providing:
- WASM bindings for the Rust VM
- TypeScript definitions
- BabylonJS integration
- ThreeJS integration

**Building:**
```bash
cd js-bindings
npm install
npm run build
```

**Publishing:**
```bash
npm publish
```

## Development Workflow

1. **Update Grammar**: Edit `grammar/GadgetLanguage.g4`
2. **Regenerate Parser**: Run `make parser` in `rust-vm/`
3. **Implement VM Logic**: Add processing in `rust-vm/src/`
4. **Build WASM**: Run `make wasm` in `rust-vm/`
5. **Update Bindings**: Modify `js-bindings/src/` as needed
6. **Build Package**: Run `npm run build` in `js-bindings/`
7. **Test Integration**: Import package in BabylonJS/ThreeJS projects

## Integration

### Import into Virtual World

```bash
npm install @gadget/vm
```

### BabylonJS

```typescript
import { initBabylonIntegration, loadGadget } from '@gadget/vm/babylonjs';
```

### ThreeJS

```typescript
import { initThreeJSIntegration, loadGadget } from '@gadget/vm/threejs';
```

## Requirements

- Rust toolchain with `wasm32-unknown-unknown` target
- Java (for ANTLR)
- Node.js and npm
- Make

## License

MIT
