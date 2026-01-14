# Quick Start Guide

## Overview

This mono-repo contains everything needed to build and use the Gadget Language VM:
1. **Grammar** - ANTLR grammar definition
2. **Rust VM** - Core VM implementation (compiles to WASM)
3. **JS Bindings** - npm package for JavaScript/TypeScript integration

## Step-by-Step Setup

### 1. Grammar Definition ✅

The grammar file is already created at `grammar/GadgetLanguage.g4`. This defines the DSL syntax.

### 2. Build Rust VM

```bash
cd rust-vm

# Install Rust WASM target (first time only)
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli (first time only)
cargo install wasm-bindgen-cli

# Generate parser from grammar
make parser

# Compile to WASM
make wasm
```

This generates:
- Parser code in `rust-vm/src/parser/` (from ANTLR)
- WASM binary and bindings in `js-bindings/wasm/`

### 3. Build JavaScript Package

```bash
cd js-bindings

# Install dependencies
npm install

# Build TypeScript and prepare package
npm run build
```

### 4. Publish to npm (when ready)

```bash
cd js-bindings

# Set package name/scope if needed
# Edit package.json

# Publish
npm publish
```

### 5. Use in Your Project

#### Install

```bash
npm install @gadget/vm
```

#### Basic Usage

```typescript
import { initVM, parse } from '@gadget/vm';

await initVM();
const result = await parse(`
  collider box1 {
    shape box(1, 1, 1)
  }
`);
```

#### BabylonJS Integration

```typescript
import { initBabylonIntegration, loadGadget } from '@gadget/vm/babylonjs';
import { Scene } from '@babylonjs/core';

const scene = new Scene(engine);
const integration = await initBabylonIntegration(scene);
await loadGadget(integration, gadgetSource);
```

#### ThreeJS Integration

```typescript
import { initThreeJSIntegration, loadGadget } from '@gadget/vm/threejs';
import { Scene } from 'three';

const scene = new Scene();
const integration = await initThreeJSIntegration(scene);
await loadGadget(integration, gadgetSource);
```

## Development Workflow

1. **Edit Grammar**: Modify `grammar/GadgetLanguage.g4`
2. **Regenerate Parser**: `cd rust-vm && make parser`
3. **Update VM Logic**: Edit `rust-vm/src/lib.rs` or add new modules
4. **Rebuild WASM**: `cd rust-vm && make wasm`
5. **Update Bindings**: Edit `js-bindings/src/` as needed
6. **Rebuild Package**: `cd js-bindings && npm run build`
7. **Test**: Import in your BabylonJS/ThreeJS project

## File Structure

```
gadget-vm/
├── grammar/
│   ├── GadgetLanguage.g4    # ANTLR grammar
│   └── README.md
├── rust-vm/
│   ├── Cargo.toml           # Rust project config
│   ├── Makefile             # Build automation
│   ├── build.sh             # WASM build script
│   ├── src/
│   │   ├── lib.rs           # Main WASM exports
│   │   └── parser/          # Generated parser (from ANTLR)
│   └── README.md
├── js-bindings/
│   ├── package.json         # npm package config
│   ├── tsconfig.json        # TypeScript config
│   ├── src/
│   │   ├── index.ts         # Main exports
│   │   ├── babylonjs.ts     # BabylonJS integration
│   │   └── threejs.ts       # ThreeJS integration
│   ├── wasm/                # Generated WASM files
│   └── README.md
├── README.md                # Main documentation
└── QUICKSTART.md            # This file
```

## Next Steps

- [ ] Implement full parser integration
- [ ] Add IR (Protobuf) generation
- [ ] Implement event queue system
- [ ] Add collision event handling
- [ ] Implement action execution
- [ ] Add BabylonJS mesh/collider integration
- [ ] Add ThreeJS object/collider integration
