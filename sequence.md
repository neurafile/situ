```mermaid
flowchart TB
  classDef q fill:#fff3cd,stroke:#b08a00,stroke-width:1px,color:#1a1a1a;

  %% ---------- Authoring ----------
  subgraph Authoring["Authoring (v0)"]
    DSL["Gadget Language (DSL)<br/>piano-first keywords (not free code)"]
    QDSL["Q: v0 keyword set?<br/>collider defs • listener defs • allowed actions"]:::q
    DSL -.-> QDSL

    ANTLR["ANTLR Grammar (.g4)"]
    RustParser["Rust Parser"]
    IR["IR v0 (Protobuf)<br/>- nodes/geometry refs<br/>- collider shapes<br/>- listener table<br/>- action templates"]
    QIR["Q: exact IR schema?<br/>What is the minimal listener+action model?"]:::q
    IR -.-> QIR

    DSL --> ANTLR --> RustParser --> IR
  end

  %% ---------- Package ----------
  subgraph Package["Gadget Package (self-contained archive)"]
    PKG[".gadgetpkg (zip-like)<br/>manifest + model.glb + sounds/ + ir.pb"]
    QName["Q: naming contract?<br/>GLB nodeIds/animIds mapping + stable instance IDs"]:::q
    PKG -.-> QName
  end

  IR --> PKG

  %% ---------- Host World ----------
  subgraph Host["Host World (Babylon / Three / Unity / …)"]
    Loader["Gadget Loader"]
    Inspect["Inspect/Validate (v0)<br/>IR-only inspection + quotas"]
    QInspect["Q: v0 quotas?<br/>max colliders • max listeners • max actions/event"]:::q
    Inspect -.-> QInspect

    Player["Gadget Player (Rust → WASM)<br/>single-thread event queue<br/>async tasks + cancel"]
    QLoop["Q: task API surface?<br/>what can yield/await, and how is cancel addressed?"]:::q
    Player -.-> QLoop

    Physics["Host Physics/Collision"]
    CollideEvt["Collision Events → Player<br/>discrete (no continuous)"]
    QCollide["Q: collision payload?<br/>targetId • enter/exit • contact info (if any)"]:::q
    CollideEvt -.-> QCollide

    EventBus["Host Event Bus<br/>FIFO (priority reserved)"]
    QEventOrder["Q: ordering guarantees?<br/>FIFO only, or priority field in v0?"]:::q
    EventBus -.-> QEventOrder

    Actions["Host Executes Actions (v0)<br/>Run GLB animation (named)<br/>Play/Stop sound (relative asset)<br/>HTTP request"]
    QActionSet["Q: final v0 action list?<br/>RunAnim/StopAnim • PlaySound/StopSound • HttpCall • EmitEvent?"]:::q
    Actions -.-> QActionSet
  end

  PKG --> Loader --> Inspect --> Player
  Physics --> CollideEvt --> EventBus --> Player
  Player --> Actions

  %% ---------- External Services (BYO compute) ----------
  subgraph Services["Owner-Provided Services (BYO keys/compute)"]
    HTTP["HTTP Call (Protobuf)"]
    SSE["SSE Stream (Protobuf events)"]
    QAuth["Q: auth model v0?<br/>how does the owner key/token get attached + scoped?"]:::q
    HTTP -.-> QAuth
    Backend["Owner Service<br/>(LLM/TTS later; v0 can be piano/logic stream)"]
  end

  Actions --> HTTP --> Backend --> SSE --> EventBus

  %% ---------- Iteration 2 ----------
  subgraph Iter2["Iteration #2 (multiplayer + push)"]
    WS["WebSocket (Protobuf)<br/>bi-directional + broadcast"]
    QMP["Q: multiplayer authority?<br/>host broadcasts vs owner service broadcasts"]:::q
    WS -.-> QMP
  end

  Backend -. v1+ .-> WS
  WS -. v1+ .-> EventBus
  ```
