## Overview

Simulate mouse actions using keyboard

## Config (keyboard_to_mouse.toml)

| Property name                               | Type                       | Default value | Description                                              |
|---------------------------------------------|----------------------------|---------------|----------------------------------------------------------|
| mouse_speed                                 | i32                        | 10            | Sets the speed of the mouse cursor movement.             |
| scroll_speed                                | i64                        | 1             | Sets the speed of scrolling (e.g., lines per scroll).    |
| key_bindings                                | KeyboardToMouseKeyBindings | -             | A container for key bindings to perform mouse actions.   |
| key_bindings.activate                       | [Sequence]                 | P(CtrlRight)  | Key sequence to activate plugin.                         |
| key_bindings.deactivate                     | [Sequence]                 | R(CtrlRight)  | Key sequence to deactivate plugin.                       |
| key_bindings.mouse_move_up                  | [Sequence]                 | P(K)          | Key sequence to simulate mouse move up.                  |
| key_bindings.mouse_move_down                | [Sequence]                 | P(J)          | Key sequence to simulate mouse move down.                |
| key_bindings.mouse_move_left                | [Sequence]                 | P(H)          | Key sequence to simulate mouse move left.                |
| key_bindings.mouse_move_right               | [Sequence]                 | P(L)          | Key sequence to simulate mouse move right.               |
| key_bindings.mouse_scroll_up                | [Sequence]                 | P(U)          | Key sequence to simulate mouse scroll up.                |
| key_bindings.mouse_scroll_down              | [Sequence]                 | P(D)          | Key sequence to simulate mouse scroll down.              |
| key_bindings.mouse_scroll_left              | [Sequence]                 | -             | Not currently bound (add key sequence for left scroll).  |
| key_bindings.mouse_scroll_right             | [Sequence]                 | -             | Not currently bound (add key sequence for right scroll). |
| key_bindings.mouse_left_button_click        | [Sequence]                 | P(I)R(I)      | Key sequence to simulate left mouse button click.        |
| key_bindings.mouse_right_button_click       | [Sequence]                 | P(A)R(A)      | Key sequence to simulate right mouse button click.       |
| key_bindings.mouse_middle_button_click      | [Sequence]                 | P(C)R(C)      | Key sequence to simulate middle mouse button click.      |
| key_bindings.mouse_drag_and_drop_activate   | [Sequence]                 | P(G)R(G)      | Key sequence to activate mouse drag and drop mode.       |
| key_bindings.mouse_drag_and_drop_deactivate | [Sequence]                 | P(P)R(P)      | Key sequence to deactivate mouse drag and drop mode.     |
