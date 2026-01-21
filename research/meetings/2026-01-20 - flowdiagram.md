```mermaid
flowchart TD

  %% =========================
  %% GLB -> Flat HTML (Rust/WASM) Plan
  %% =========================

  A["Goal: FlatHTMLAdapter\nGLB -> software render -> flat HTML\nRotate + Zoom + Depth-aware Picking"] --> B{Runtime chooses adapter strategy}

  B -->|Flat HTML| C["FlatHTMLAdapter (Rust -> WASM)"]
  B -->|Babylon/Three/Unity/Unreal| C2["Other Adapters (out of scope here)"]

  %% -------------------------
  %% VM + Adapter Interface
  %% -------------------------

  subgraph VM[ Gadget VM in-browser ]
    V1[Defines gadget + events] --> V2[RenderAdapter trait]
    V2 --> V3["Adapter chosen at runtime\n(no mixing)"]
  end

  V3 --> C

  subgraph ADAPTER[ FlatHTMLAdapter Rust/WASM ]
    C --> I1["Adapter API\nload_glb()\nset_camera()\nrender_rgba()/render_png()\npick(x,y)"]
    I1 --> L1["gltf_loader\nParse GLB -> meshes/nodes/materials/textures"]
    I1 --> CAM["camera\nOrbit yaw/pitch/dist + target"]
    I1 --> R1["renderer_cpu\nTransform -> Rasterize -> Z-buffer"]
    I1 --> P1["picking\nID-buffer pass -> ObjectId"]
    I1 --> OUT["wasm_bridge\nExports to JS\nreturns RGBA or PNG bytes"]
  end

  %% -------------------------
  %% Render Pipeline (No WebGL)
  %% -------------------------

  subgraph PIPE[ Software Rendering Pipeline ]
    L1 --> T1["Scene graph resolve\nNode TRS -> world matrices"]
    CAM --> M1[View + Projection matrices]
    T1 --> M2["Apply transforms\nworld->view->clip->ndc->screen"]
    M1 --> M2
    M2 --> RS[Raster stage\nTriangle setup + barycentric]
    RS --> ZB[Depth test\nz-buffer]
    RS --> SH["Shading\nPhase 1: unlit flat color\nPhase 3: textures + simple lighting"]
    ZB --> FB[Frame buffer RGBA]
    SH --> FB
  end

  R1 --> PIPE
  FB --> OUT

  %% -------------------------
  %% Picking
  %% -------------------------

  P1 --> IDP["ID render pass\nwrite object_id per pixel\nwith depth test"]
  IDP --> PICK["pick(x,y)\nread ID buffer -> ObjectId or 0"]

  %% -------------------------
  %% JS / HTML Output Paths
  %% -------------------------

  subgraph WEB[ Browser UI Flat HTML ]
    OUT -->|RGBA buffer| CANVAS["Canvas path (recommended)\nImageData -> putImageData"]
    OUT -->|PNG bytes| IMG["Image path\nbase64 data URL -> img.src"]
    CANVAS --> UI["User input\nDrag rotate\nWheel zoom\nClick pick"]
    IMG --> UI
    UI -->|camera updates| CAM
    UI -->|"click -> pick(x,y)"| PICK
    PICK --> EVT["VM Event\non_click(ObjectId)"]
  end

  %% -------------------------
  %% Milestones
  %% -------------------------

  subgraph MILE[ Milestones / Exit Criteria ]
    M0["Phase 0: Spike\nRender 1 frame\nhardcoded camera\nExit: recognizable output"]
    M1B["Phase 1: MVP viewer\nOrbit + zoom + z-buffer\nunlit colors\nExit: stable rotation"]
    M2B["Phase 2: Picking\nID buffer pass\nExit: correct object id"]
    M3B["Phase 3: Visuals\ntextures + basic light\noptional supersample\nExit: decent panel quality"]
    M4B["Phase 4: VM integration\nAdapter plugged into Gadget VM\nExit: gadget gets click events"]
  end

  A --> M0 --> M1B --> M2B --> M3B --> M4B

  %% =========================
  %% Questions for Jordan + Steve (Today)
  %% =========================

  subgraph Q[ Questions to Align On - Bring to Meeting ]
    Q1["Q1: Output format preference?\nCanvas ImageData vs PNG base64\n(or hybrid: canvas while dragging, PNG on idle)"]
    Q2["Q2: No WebGL constraint details?\nIs Canvas2D allowed?\nAny browser targets or sandbox constraints?"]
    Q3["Q3: Primary use case first?\nForce graphs/point clouds vs arbitrary GLB models"]
    Q4["Q4: Picking granularity?\nReturn NodeId? MeshId? PrimitiveId?\nNeed multi-select / hover?"]
    Q5["Q5: Camera model standard?\nOrbit only vs pan + orbit\nNeed saved viewpoints?"]
    Q6["Q6: Performance target?\nMinimum FPS while dragging\nTypical model size triangles / nodes\nOk to render at 0.5x while dragging?"]
    Q7["Q7: Materials scope?\nUnlit ok initially?\nNeed textures? transparency? lines?"]
    Q8["Q8: Integration boundary?\nAdapter as separate crate?\nEvent API shape from VM -> adapter\nand adapter -> VM pick events"]
    Q9["Q9: Asset pipeline?\nWhere does GLB come from?\nLocal file, URL fetch, bundled gadget package?"]
    Q10["Q10: Graph primitives plan?\nShould FlatHTMLAdapter also support\nnative primitives dots/lines for graphs\ninstead of forcing everything into GLB?"]
  end

  A -.-> Q
  C -.-> Q
  ```