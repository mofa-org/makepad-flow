# API Gap Analysis: makepad-flow vs xyflow (React Flow)

Comprehensive comparison of features between makepad-flow and [xyflow/React Flow](https://reactflow.dev).

## Legend
- ✅ Implemented
- 🔶 Partial
- ❌ Not implemented

---

## Core Features

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| Nodes rendering | ✅ | ✅ | |
| Edges rendering | ✅ | ✅ | |
| Pan canvas | ✅ | ✅ | Shift+drag |
| Zoom canvas | ✅ | ✅ | Scroll wheel |
| Fit view | ✅ | ✅ | |
| Node dragging | ✅ | ✅ | |
| Node selection | ✅ | ✅ | |
| Multi-selection | ✅ | ✅ | Shift+click, drag box |
| Delete nodes/edges | ✅ | ✅ | Delete/Backspace |
| Keyboard shortcuts | ✅ | ✅ | Ctrl+A, Escape, Ctrl+Z/Y |

## Node Features

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| Custom node types | ✅ | ✅ | RoundedRect, DoubleRoundedRect, Rectangle, Round, Diamond |
| Node position | ✅ | ✅ | |
| Node dimensions | ✅ | ✅ | |
| Node data/label | ✅ | ✅ | |
| Node styles | ✅ | ✅ | Border width, colors |
| Node resizing | ✅ | ❌ | NodeResizer component in xyflow |
| Multiple handles | ✅ | 🔶 | Currently 1 input + 1 output per node |
| Handle positions | ✅ | 🔶 | Fixed left/right positions |
| Node toolbar | ✅ | ❌ | NodeToolbar component |
| Selectable prop | ✅ | ❌ | Per-node selectable flag |
| Draggable prop | ✅ | ❌ | Per-node draggable flag |
| Connectable prop | ✅ | ❌ | Per-node connectable flag |
| Hidden prop | ✅ | ❌ | Per-node visibility |
| Z-index | ✅ | ❌ | Layer ordering |
| Parent nodes | ✅ | ❌ | Nested/grouped nodes |
| Extent constraints | ✅ | ❌ | Limit node movement area |

## Edge Features

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| Edge types | ✅ | 🔶 | Bezier only (xyflow: bezier, step, smoothstep, straight) |
| Edge labels | ✅ | ✅ | |
| Edge markers | ✅ | ✅ | Arrow heads |
| Edge styles | ✅ | ✅ | Solid, dashed, dotted |
| Animated edges | ✅ | ✅ | Flow particles |
| Edge reconnection | ✅ | ❌ | Drag edge to new target |
| Deletable prop | ✅ | ❌ | Per-edge deletable flag |
| Selectable prop | ✅ | ❌ | Per-edge selectable flag |
| Edge label position | ✅ | ❌ | labelBgPadding, labelBgStyle |
| Custom edge paths | ✅ | ❌ | getSmoothStepPath, getBezierPath |

## Interaction

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| Connection creation | ✅ | ✅ | Drag from port |
| Connection validation | ✅ | ❌ | isValidConnection callback |
| Snap to grid | ✅ | ❌ | snapToGrid, snapGrid props |
| Selection box | ✅ | ✅ | |
| Pan on scroll | ✅ | ❌ | panOnScroll prop |
| Pan on drag | ✅ | ✅ | Shift+drag |
| Zoom on scroll | ✅ | ✅ | |
| Zoom on double-click | ✅ | ❌ | |
| Prevent scrolling | ✅ | ❌ | preventScrolling prop |
| Node drag threshold | ✅ | ❌ | nodeDragThreshold prop |
| Selection on drag | ✅ | ✅ | |

## Events/Callbacks

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| onNodesChange | ✅ | ❌ | Node add/remove/position/select |
| onEdgesChange | ✅ | ❌ | Edge add/remove/select |
| onConnect | ✅ | ❌ | New connection created |
| onNodeClick | ✅ | 🔶 | Internal only |
| onNodeDrag | ✅ | 🔶 | Internal only |
| onNodeDragStart | ✅ | ❌ | |
| onNodeDragStop | ✅ | ❌ | |
| onEdgeClick | ✅ | 🔶 | Internal only |
| onPaneClick | ✅ | 🔶 | Internal only |
| onMove | ✅ | ❌ | Viewport move |
| onMoveStart | ✅ | ❌ | |
| onMoveEnd | ✅ | ❌ | |
| onSelectionChange | ✅ | ❌ | |
| onInit | ✅ | ❌ | ReactFlow instance ready |

## UI Components

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| MiniMap | ✅ | ❌ | Overview in corner |
| Controls | ✅ | ❌ | Zoom in/out/fit buttons |
| Background | ✅ | ❌ | Dots/lines/cross pattern |
| Panel | ✅ | ❌ | Positioned panel component |
| NodeResizer | ✅ | ❌ | Resize handles |
| NodeToolbar | ✅ | ❌ | Floating toolbar |

## Viewport

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| getViewport | ✅ | 🔶 | zoom, pan_offset available |
| setViewport | ✅ | ❌ | |
| fitView | ✅ | ✅ | |
| zoomIn | ✅ | 🔶 | Scroll only |
| zoomOut | ✅ | 🔶 | Scroll only |
| zoomTo | ✅ | ❌ | Zoom to specific level |
| setCenter | ✅ | ❌ | Center on coordinates |
| fitBounds | ✅ | ❌ | Fit to specific bounds |

## Utilities

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| getOutgoers | ✅ | ❌ | Get downstream nodes |
| getIncomers | ✅ | ❌ | Get upstream nodes |
| getConnectedEdges | ✅ | ❌ | Get edges for nodes |
| isNode | ✅ | ❌ | Type guard |
| isEdge | ✅ | ❌ | Type guard |
| addEdge | ✅ | ✅ | |
| applyNodeChanges | ✅ | ❌ | Apply change set |
| applyEdgeChanges | ✅ | ❌ | Apply change set |

## State Management

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| Controlled mode | ✅ | ❌ | External state management |
| Uncontrolled mode | ✅ | ✅ | Internal state |
| useNodesState | ✅ | ❌ | Hook for nodes |
| useEdgesState | ✅ | ❌ | Hook for edges |
| useReactFlow | ✅ | ❌ | Instance access hook |
| useNodes | ✅ | ❌ | |
| useEdges | ✅ | ❌ | |
| useViewport | ✅ | ❌ | |

## Persistence

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| Export to JSON | ✅ | ❌ | toObject() |
| Import from JSON | ✅ | ❌ | |
| Save/restore viewport | ✅ | ❌ | |
| Undo/redo | 🔶 | ✅ | Ctrl+Z/Y |

## Styling

| Feature | xyflow | makepad-flow | Notes |
|---------|--------|--------------|-------|
| CSS variables | ✅ | ❌ | Theming via CSS |
| Dark mode | ✅ | 🔶 | Currently dark only |
| Custom colors | ✅ | ✅ | Via code |
| Class names | ✅ | ❌ | |

---

## Priority Implementation Order

### High Priority (Core UX)
1. **MiniMap** - Essential for large graphs
2. **Controls panel** - Zoom buttons for accessibility
3. **Background grid** - Visual reference
4. **Snap to grid** - Precision layout

### Medium Priority (Power Users)
5. **Node resizing** - Flexible layouts
6. **Multiple handles** - Complex connections
7. **Edge reconnection** - Edit existing connections
8. **Copy/paste** - Productivity

### Lower Priority (Advanced)
9. **Export/import JSON** - Persistence
10. **Event callbacks** - Integration
11. **Auto-layout** - dagre/elkjs integration
12. **Parent-child nodes** - Hierarchies

---

## Summary

| Category | xyflow | makepad-flow | Coverage |
|----------|--------|--------------|----------|
| Core | 10 | 10 | 100% |
| Node Features | 15 | 5 | 33% |
| Edge Features | 10 | 5 | 50% |
| Interaction | 11 | 5 | 45% |
| Events | 14 | 0 | 0% |
| UI Components | 6 | 0 | 0% |
| Viewport | 8 | 2 | 25% |
| Utilities | 8 | 1 | 12% |
| State | 7 | 1 | 14% |
| Persistence | 4 | 1 | 25% |

**Overall: ~30% feature parity with xyflow**

Sources:
- [React Flow Documentation](https://reactflow.dev)
- [xyflow GitHub](https://github.com/xyflow/xyflow)
