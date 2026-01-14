# Dataflow Visualization Gaps

Analysis of gaps for visualizing `examples/dataflow/voice-chat.yml` in makepad-flow.

## Dataflow Structure

### Nodes (15 total)

| Category | Nodes | Description |
|----------|-------|-------------|
| MaaS Clients | student1, student2, tutor | LLM chat participants |
| Text Segmenter | multi-text-segmenter | Splits text into segments |
| TTS Engines | primespeech-student1/2/tutor | Text-to-speech |
| Bridges | bridge-to-student1/2/tutor | Message routing |
| Controller | conference-controller | Orchestration logic |
| MoFA Dynamic | mofa-audio-player, mofa-prompt-input, mofa-system-log | UI widgets |

### Node Properties

```yaml
- id: student1                    # Unique ID
  build: cargo build ...          # Build command (optional)
  path: .../dora-maas-client      # Executable or "dynamic"
  inputs:                         # Multiple input ports
    text: bridge-to-student1/text
    control: conference-controller/llm_control
  outputs:                        # Multiple output ports
    - text
    - status
    - log
  env:                            # Environment variables
    MAAS_CONFIG_PATH: ...
```

### Connection Format

```yaml
inputs:
  port_name: source_node/output_port     # Simple
  port_name:                             # With config
    source: source_node/output_port
    queue_size: 1000
```

---

## Critical Gaps

### 1. Multiple Ports Per Node ❌ BLOCKING

**Current:** 1 input + 1 output per node

**Required:** Variable inputs and outputs

| Node | Inputs | Outputs |
|------|--------|---------|
| student1 | 2 | 3 |
| multi-text-segmenter | 6 | 6 |
| conference-controller | 6 | 7 |
| mofa-system-log | 27 | 0 |

**Implementation:**
```rust
struct FlowNode {
    // Replace single ports with vectors
    input_ports: Vec<Port>,   // Was: has single input_pos()
    output_ports: Vec<Port>,  // Was: has single output_pos()
}

struct Port {
    id: String,
    label: String,
    position: DVec2,  // Calculated based on index
}
```

### 2. Port Labels ❌ HIGH

**Current:** No port labels

**Required:** Show port names on hover or always

```
┌─────────────────────┐
│      student1       │
├─────────────────────┤
│ ○ text              │
│ ○ control           │
├─────────────────────┤
│           text ○    │
│         status ○    │
│            log ○    │
└─────────────────────┘
```

### 3. YAML Import ❌ BLOCKING

**Current:** Hardcoded test nodes

**Required:** Parse YAML dataflow

```rust
// New module: src/dataflow_parser.rs
fn parse_dataflow(yaml: &str) -> (Vec<FlowNode>, Vec<EdgeConnection>) {
    let doc: serde_yaml::Value = serde_yaml::from_str(yaml)?;

    // Parse nodes
    for node in doc["nodes"].as_sequence()? {
        let id = node["id"].as_str()?;
        let inputs = parse_inputs(&node["inputs"]);
        let outputs = parse_outputs(&node["outputs"]);
        // ...
    }

    // Create edges from input sources
    for (node_id, input_port, source) in inputs {
        // source = "node_id/output_port"
        let (src_node, src_port) = source.split_once('/')?;
        edges.push(Edge { src_node, src_port, dst_node: node_id, dst_port: input_port });
    }
}
```

### 4. Auto-Layout ❌ HIGH

**Current:** Manual positioning only

**Required:** Automatic graph layout for 15+ nodes

Options:
- Dagre (hierarchical)
- Force-directed
- Layered (by category)

### 5. Node Categories/Colors ❌ MEDIUM

**Current:** Single color scheme

**Required:** Color by node type

| Category | Color |
|----------|-------|
| MaaS | Blue |
| TTS | Green |
| Bridge | Orange |
| Controller | Purple |
| MoFA | Cyan |

### 6. Edge Labels ✅ DONE

Already implemented for showing port names on edges.

### 7. Dynamic vs Static Nodes ❌ LOW

**Current:** No distinction

**Required:** Visual indicator for `path: dynamic`

---

## Node Size Requirements

With multiple ports, nodes need dynamic sizing:

```rust
fn calculate_node_size(node: &FlowNode) -> DVec2 {
    let port_count = node.input_ports.len().max(node.output_ports.len());
    let port_height = 20.0;
    let header_height = 32.0;
    let padding = 16.0;

    let height = header_height + (port_count as f64 * port_height) + padding;
    let width = 180.0; // Or calculate based on longest label

    DVec2 { x: width, y: height }
}
```

---

## Edge Routing

With multiple ports, edge routing becomes complex:

```
Current: Simple bezier from node center-left to node center-right

Required:
- Port-to-port connections
- Avoid crossing nodes
- Handle many edges to same node (mofa-system-log has 27 inputs!)
```

---

## Implementation Priority

### Phase 1: Core Structure
1. [ ] Multiple ports per node (struct changes)
2. [ ] Port positioning (vertical stack on left/right)
3. [ ] Port-to-port edge connections
4. [ ] YAML parser (basic)

### Phase 2: Usability
5. [ ] Port labels (on hover)
6. [ ] Node categories/colors
7. [ ] Auto-layout (basic hierarchical)
8. [ ] Node resizing for port count

### Phase 3: Polish
9. [ ] Queue size on edges
10. [ ] Environment variables panel
11. [ ] Build status indicator
12. [ ] Edge routing optimization

---

## Example: Parsing voice-chat.yml

```
Nodes to create: 15
Edges to create: ~50 (from all input->source mappings)

Layout suggestion:
Row 1: mofa-prompt-input
Row 2: student1, student2, tutor
Row 3: bridge-to-student1, bridge-to-student2, bridge-to-tutor
Row 4: conference-controller
Row 5: multi-text-segmenter
Row 6: primespeech-student1, primespeech-student2, primespeech-tutor
Row 7: mofa-audio-player, mofa-system-log
```

---

## Summary

| Gap | Priority | Effort | Status |
|-----|----------|--------|--------|
| Multiple ports | BLOCKING | High | ❌ |
| YAML import | BLOCKING | Medium | ❌ |
| Port labels | High | Low | ❌ |
| Auto-layout | High | High | ❌ |
| Node categories | Medium | Low | ❌ |
| Dynamic sizing | Medium | Medium | ❌ |
| Edge routing | Medium | High | ❌ |
| Queue sizes | Low | Low | ❌ |
| Env vars panel | Low | Medium | ❌ |

**Minimum viable dataflow visualization requires:**
1. Multiple ports per node
2. YAML import
3. Basic auto-layout
