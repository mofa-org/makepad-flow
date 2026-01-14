# Makepad Flow Editor Roadmap

Gap analysis vs xyflow (React Flow) - Feature implementation plan.

## P0.1 - Core UX (Essential) - COMPLETE

| Feature | Description | Status |
|---------|-------------|--------|
| Multi-selection | Shift+click to add to selection, drag box to select multiple | DONE |
| Edge markers | Arrow heads at edge endpoints | DONE |
| Edge labels | Text labels on edges (via edge.label property) | DONE |
| Undo/redo | History stack for node/edge changes (Ctrl+Z/Y) | DONE |
| Keyboard shortcuts | Delete, Ctrl+A select all, Escape deselect | DONE |

## P0.2 - Navigation & Visualization

| Feature | Description | Status |
|---------|-------------|--------|
| MiniMap | Small overview of entire canvas in corner | TODO |
| Controls panel | Zoom in/out/fit buttons | TODO |
| Background grid | Dot/line pattern on canvas | TODO |
| Snap to grid | Align nodes to grid while dragging | TODO |

## P0.3 - Advanced Interactions

| Feature | Description | Status |
|---------|-------------|--------|
| Node resizing | Drag handles to resize nodes | TODO |
| Multiple handles | Multiple input/output ports per node | TODO |
| Edge reconnection | Drag existing edge to new target | TODO |
| Connection validation | Validate connections before creating | TODO |
| Copy/paste | Duplicate selected nodes/edges | TODO |

## P0.4 - Layout & Structure

| Feature | Description | Status |
|---------|-------------|--------|
| Auto-layout | Automatic node arrangement (tree, force) | TODO |
| Parent-child grouping | Nested node hierarchies | TODO |
| Node extent | Constrain node movement to area | TODO |
| Z-index control | Layer ordering for overlapping nodes | TODO |

## P0.5 - Data & Integration

| Feature | Description | Status |
|---------|-------------|--------|
| Export/import | Save/load flow as JSON | TODO |
| Event callbacks | onNodesChange, onConnect, etc. | TODO |
| Graph analysis | getIncomers, getOutgoers helpers | TODO |
| Theming | Dark/light mode, CSS variables | TODO |

---

## Current Implementation (Done)

- Canvas pan (shift+drag)
- Canvas zoom (scroll wheel)
- Node dragging
- Node selection
- Node shapes (RoundedRect, DoubleRoundedRect, Rectangle, Round, Diamond)
- Custom SDF shaders for smooth rounded corners (DrawRoundedRect, DrawRoundedTopRect, DrawRoundedBottomRect)
- Per-node border width
- Centered text in node headers
- Bezier curve edges
- Edge styles (solid, dashed, dotted)
- Per-edge properties (style, width, animated)
- Animated flow particles on edges
- Port-based edge creation
- Delete nodes/edges (Delete/Backspace key)
- Context menus (right-click on node)
- Multi-selection context menu (apply changes to all selected nodes)
- Fit view
- Clear all
