## Overview

Move a cursor to the desired region of the screen by pressing a sequence of keys.

## Config (navigation_grid.toml)

| Property name           | Type                   | Default value                | Description                                                               |
|-------------------------|------------------------|------------------------------|---------------------------------------------------------------------------|
| allowed_label_key       | String                 | "qwertyuiopasdfghjklzxcvbnm" | Key characters that can be used to generate labels for navigation points. |
| key_bindings            | NavigationGridBindings | -                            | A container for key bindings.                                             |
| key_bindings.activate   | [Sequence]             | P(AltRight)R(AltRight)       | Key sequence to activate plugin.                                          |
| key_bindings.deactivate | [Sequence]             | P(Escape)                    | Key sequence to deactivate plugin.                                        |
