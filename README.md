# quin

Utility to control mouse with keyboard.

[//]: # (todo link to technical docs)

### 🚧Work in progress🚧

Keep in mind the project is unstable and under development.
Only Windows is currently supported.
If you encounter a problem or have an idea for improvement,
feel free to [create issue](https://github.com/lkaratl/quin/issues/new).

## Features

[//]: # (todo link to technical docs)
[grid navigation mode]
* show grid use case
* show pivot use case

[//]: # (todo link to technical docs)
[mouse movement/buttons emulation]
* hjkl - movement
* ;',./ - mouse emulation events
* drag & drop

## Config

`common` - common app configurations

- `active_handlers` - list of enabled handlers.
  There are several available handlers:
  [`grid-mode-handler`](/src/registry/grid_mode_handler.rs),
  [`mouse-buttons-emulation-handler`](/src/registry/mb_emulation_handler.rs),
  [`precise-mode-handler`](/src/registry/precise_mode_handler.rs).
  Remove handler id from the list to disable handler.

`handlers` - list of handler configurations.

- `grid-mode-handler` - configuration for grid navigation mode.
    * `pivot_grid_density_px` - distance between pivot grid points.
    * `bindings` - map of label to hotkey bindings.
        * `gm_activate` - throwing this label activates the handler. If the label is thrown again, the handler will be
          deactivated.
- `mouse-buttons-emulation-handler` - configuration for mouse button events emulation.
    * `scroll_speed` - the speed at which the scroll is simulated.
    * `bindings` - map of label to hotkey bindings.
        * `mb_toggle` - this label activates the handler. If the label is thrown again, the handler will be deactivated.
        * `mb_activate` - this label activates the handler until `mb_deactivate` is thrown.
        * `mb_deactivate` - this label deactivates the handler.
        * `mb_left` - this label emulates a left mouse button click.
        * `mb_right` - this label emulates a right mouse button click.
        * `mb_middle` - this label emulates a middle mouse button click.
        * `mb_scroll_up` - this label emulates a scroll up movement.
        * `mb_scroll_down` - this label emulates a scroll down movement.
        * `mb_drag_and_drop` - this label emulates a left mouse button press. If it is thrown again, the left mouse
          button will be released.
- `precise-mode-handler` - configuration for precise mouse movement handler.
    * `cursor_speed` - the speed at which the cursor will be moved.
    * `bindings` - map of label to hotkey bindings.
        * `pm_toggle` - throwing this label activates the handler. If the label is thrown again, the handler will be
          deactivated.
        * `pm_activate` - throwing this label activate the handler until `pm_deactivate` will be thrown.
        * `pm_deactivate` - throwing this label deactivate the handler.
        * `pm_move_left` - throwing this label move cursor to left
        * `pm_move_right` - throwing this label move cursor to right
        * `pm_move_top` - throwing this label move cursor to top
        * `pm_move_bottom` - throwing this label move cursor to bottom

`P(<key>)` - emit press event
`R(<key>)` - emit release event
`<key>` - possible values see [here](https://github.com/lkaratl/quin/blob/fdcc29dd016a13399c4e2516176b40232510427e/src/core.rs#L79)

Source of [default config](config.toml) file.

## License

This project is licensed under the [GNU license](LICENSE).
