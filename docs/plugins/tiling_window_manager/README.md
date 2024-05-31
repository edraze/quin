## Overview

This plugin provides integration between quin input handling and [Komorebi](https://github.com/LGUG2Z/komorebi) tiling
manager.

## Config (tiling_window_manager.toml)

| Property name                | Type       | Default value              | Description                                                       |
|------------------------------|------------|----------------------------|-------------------------------------------------------------------|
| key_bindings.focus_left      | [Sequence] | P(AltLeft)P(J)             | Move focus to the window on the left in tiles view.               |
| key_bindings.focus_right     | [Sequence] | P(AltLeft)P(;)             | Move focus to the window on the right in tiles view.              |
| key_bindings.focus_up        | [Sequence] | P(AltLeft)P(L)             | Move focus to the window above in tiles view.                     |
| key_bindings.focus_down      | [Sequence] | P(AltLeft)P(K)             | Move focus to the window below in tiles view.                     |
| key_bindings.move_left       | [Sequence] | P(AltLeft)P(ShiftLeft)P(J) | Move the currently focused window to the left in tiles view.      |
| key_bindings.move_right      | [Sequence] | P(AltLeft)P(ShiftLeft)P(;) | Move the currently focused window to the right in tiles view.     |
| key_bindings.move_up         | [Sequence] | P(AltLeft)P(ShiftLeft)P(L) | Move the currently focused window to the top in tiles view.       |
| key_bindings.move_down       | [Sequence] | P(AltLeft)P(ShiftLeft)P(K) | Move the currently focused window to the bottom in tiles view.    |
| key_bindings.stack_left      | [Sequence] | P(AltLeft)P(LeftArrow)     | Add the currently focused window to the stack on the left.        |
| key_bindings.stack_right     | [Sequence] | P(AltLeft)P(RightArrow)    | Add the currently focused window to the stack on the right.       |
| key_bindings.stack_up        | [Sequence] | P(AltLeft)P(UpArrow)       | Add the currently focused window to the stack above.              |
| key_bindings.stack_down      | [Sequence] | P(AltLeft)P(DownArrow)     | Add the currently focused window to the stack below.              |
| key_bindings.unstack         | [Sequence] | P(AltLeft)P(Escape)        | Remove the currently focused window from its stack.               |
| key_bindings.toggle_maximize | [Sequence] | P(AltLeft)P(O)             | Toggle maximize/unmaximize state of the currently focused window. |
| key_bindings.toggle_monocle  | [Sequence] | P(AltLeft)P(T)             | Toggle monocle mode (show only the focused window).               |
| key_bindings.toggle_float    | [Sequence] | P(AltLeft)P(F)             | Toggle float mode (make the window untiled and floating).         |
| key_bindings.minimize        | [Sequence] | P(AltLeft)P(M)             | Minimize the currently focused window.                            |
| key_bindings.close           | [Sequence] | P(AltLeft)P(X)             | Close the currently focused window.                               |

> [!NOTE]
> Komorebi configuratins such as `komorebi.json` and `applications.yml` stored in path `./config/komorebi`.
> If you need to customize komorebi behavior use this configuration.
> For more details [see docs](https://lgug2z.github.io/komorebi/).
