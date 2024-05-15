## Overview

Executor for mouse output events.

## Events

| Event               | Type     | Description                                                                                     |
|---------------------|----------|-------------------------------------------------------------------------------------------------|
| MoveMouseRelatively | Incoming | Moves the mouse cursor a relative distance (specified in pixels) on the screen.                 |
| MoveMouseToPosition | Incoming | Moves the mouse cursor to an absolute position (specified in screen coordinates) on the screen. |
| Scroll              | Incoming | Simulates scrolling up, down, left, or right on the screen.                                     |
| MouseClick          | Incoming | Simulates a mouse click (left, right, middle) at a specific position on the screen.             |
| DragAndDrop         | Incoming | Simulates dragging an object from one location to another on the screen.                        |
