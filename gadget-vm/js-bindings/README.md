# @gadget/vm - JavaScript Bindings

JavaScript/TypeScript bindings for the Gadget VM (Rust/WASM).

## Installation

```bash
npm install @gadget/vm
```

## Usage

### Basic Usage

```typescript
import { initVM, parse, process } from '@gadget/vm';

// Initialize (required before first use)
await initVM();

// Parse Gadget Language source
const result = await parse(`
  collider box1 {
    shape box(1, 1, 1)
  }
`);

// Process input
const output = await process('some input');
```

### BabylonJS Integration

```typescript
import { initBabylonIntegration, loadGadget } from '@gadget/vm/babylonjs';
import { Scene } from '@babylonjs/core';

const scene = new Scene(engine);
const integration = await initBabylonIntegration(scene);

await loadGadget(integration, gadgetSource);
```

### ThreeJS Integration

```typescript
import { initThreeJSIntegration, loadGadget } from '@gadget/vm/threejs';
import { Scene } from 'three';

const scene = new Scene();
const integration = await initThreeJSIntegration(scene);

await loadGadget(integration, gadgetSource);
```

## Building

```bash
npm run build
```

This will:
1. Build the Rust VM to WASM
2. Compile TypeScript
3. Bundle for distribution

## Publishing

```bash
npm publish
```
