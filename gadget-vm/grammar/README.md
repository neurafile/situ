# Gadget Language Grammar

ANTLR grammar definition for the Gadget Language DSL.

## File

- `GadgetLanguage.g4` - Main grammar file

## Usage

Generate parser code using ANTLR:

```bash
# Download ANTLR (if not already present)
curl -L -o antlr-4.13.2-complete.jar https://www.antlr.org/download/antlr-4.13.2-complete.jar

# Generate Rust parser
java -jar antlr-4.13.2-complete.jar -Dlanguage=Rust -visitor -o ../rust-vm/src/parser GadgetLanguage.g4
```

Or use the Makefile in `../rust-vm/`:

```bash
cd ../rust-vm
make parser
```

## Grammar Overview

The Gadget Language is a "piano-first" DSL with keywords for:
- **Collider definitions**: Define collision shapes (box, sphere, cylinder, capsule) or reference geometry
- **Listener definitions**: Define event handlers (collision, trigger, custom) with target IDs and actions
- **Action definitions**: Define actions (runAnim, stopAnim, playSound, stopSound, httpCall, emitEvent)

## Example

```
collider box1 {
    shape box(1, 1, 1)
}

listener collision1 {
    on collision
    target "player"
    action moveAction
}

action moveAction {
    runAnim
    name = "walk"
}
```
