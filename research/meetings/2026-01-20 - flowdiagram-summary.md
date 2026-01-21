# FlatHTMLAdapter Plan - Summary

## Summary

This document outlines a plan to implement a **FlatHTMLAdapter** that converts GLB 3D models into flat HTML output using Rust/WASM with software rendering (no WebGL). The adapter will be integrated into the Gadget VM as one of several runtime-chosen rendering strategies.

### Key Components

1. **Gadget VM Integration**: The VM defines gadgets and events, with a `RenderAdapter` trait that allows runtime selection of adapters (FlatHTMLAdapter, Babylon, Three.js, Unity, Unreal, etc.)

2. **FlatHTMLAdapter (Rust/WASM)**:
   - **GLTF Loader**: Parses GLB files into meshes, nodes, materials, and textures
   - **Camera System**: Orbit controls (yaw/pitch/distance) with target point
   - **CPU Renderer**: Software rendering pipeline with transform, rasterization, and z-buffer
   - **Picking System**: ID-buffer pass for object identification
   - **WASM Bridge**: Exports to JavaScript, returns RGBA or PNG bytes

3. **Software Rendering Pipeline**:
   - Scene graph resolution (node TRS → world matrices)
   - View and projection matrix application
   - Transform pipeline (world → view → clip → NDC → screen)
   - Rasterization (triangle setup + barycentric coordinates)
   - Depth testing with z-buffer
   - Shading (Phase 1: unlit flat colors, Phase 3: textures + simple lighting)
   - Frame buffer output (RGBA)

4. **Browser Integration**:
   - Canvas path (recommended): ImageData → putImageData
   - Image path: PNG bytes → base64 data URL → img.src
   - User input handling: drag rotate, wheel zoom, click pick
   - Event system: click → pick(x,y) → VM event `on_click(ObjectId)`

### Implementation Phases

- **Phase 0 (Spike)**: Render 1 frame with hardcoded camera. Exit: recognizable output
- **Phase 1 (MVP Viewer)**: Orbit + zoom + z-buffer with unlit colors. Exit: stable rotation
- **Phase 2 (Picking)**: ID buffer pass. Exit: correct object ID returned
- **Phase 3 (Visuals)**: Textures + basic lighting, optional supersampling. Exit: decent panel quality
- **Phase 4 (VM Integration)**: Adapter plugged into Gadget VM. Exit: gadget receives click events

## Open Questions

1. **Output Format Preference**: Canvas ImageData vs PNG base64? Or hybrid approach (canvas while dragging, PNG on idle)?



2. **No WebGL Constraint Details**: Is Canvas2D allowed? Any specific browser targets or sandbox constraints?

3. **Primary Use Case**: Force graphs/point clouds vs arbitrary GLB models - which should be prioritized first?

4. **Picking Granularity**: Should picking return NodeId, MeshId, or PrimitiveId? Is multi-select or hover needed?

5. **Camera Model Standard**: Orbit only vs pan + orbit? Do we need saved viewpoints?

6. **Performance Targets**: Minimum FPS while dragging? Typical model size (triangles/nodes)? Is it acceptable to render at 0.5x resolution while dragging?

7. **Materials Scope**: Is unlit rendering acceptable initially? Do we need textures, transparency, or line rendering?

8. **Integration Boundary**: Should the adapter be a separate crate? What should the event API shape be from VM → adapter and adapter → VM for pick events?

9. **Asset Pipeline**: Where does GLB come from? Local file, URL fetch, or bundled gadget package?

10. **Graph Primitives Plan**: Should FlatHTMLAdapter also support native primitives (dots/lines) for graphs instead of forcing everything into GLB format?

## Next Steps

1. **Alignment Meeting**: Bring the 10 open questions to Jordan and Steve to get alignment on:
   - Technical constraints and preferences
   - Use case priorities
   - Integration boundaries
   - Performance requirements

2. **Phase 0 - Spike Implementation**:
   - Set up Rust/WASM project structure for FlatHTMLAdapter
   - Implement basic GLTF loader to parse GLB files
   - Create minimal software renderer to output one frame
   - Hardcode camera position and render to RGBA buffer
   - Export to JavaScript and display in browser
   - **Exit Criteria**: Verify recognizable output appears in browser

3. **Technical Research**:
   - Evaluate existing Rust GLTF parsing libraries
   - Research CPU-based rasterization approaches
   - Investigate WASM performance characteristics for rendering workloads
   - Review Canvas2D vs ImageData performance trade-offs

4. **Architecture Design**:
   - Design RenderAdapter trait interface
   - Define event API contract between VM and adapter
   - Plan crate structure (separate crate vs integrated)
   - Design camera system API

5. **Begin Phase 1 After Alignment**:
   - Implement orbit camera controls
   - Add zoom functionality
   - Complete z-buffer implementation
   - Integrate user input handling (drag, wheel)
   - **Exit Criteria**: Stable rotation with proper depth sorting
