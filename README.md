<div align="center">
    <h1>QUIN</h1>
    <strong>Set of utilities to enhance OS experience and increase user productivity and efficiency.</strong>
</div>

> [!WARNING]
> Project is unstable and underdevelopment. Only Windows is currently supported.
> If you encounter a problem or have an idea for improvement,
> feel free to [create issue](https://github.com/lkaratl/quin/issues/new).

![](https://img.shields.io/badge/status-experimental-orange)
![](https://img.shields.io/badge/maintenance-active-green)
[![Build status](https://badge.buildkite.com/9bd62cf4deef218e88237833d17de6d382383de6411e4e1b08.svg?branch=main)](https://buildkite.com/merk/quin-build)
[![codecov](https://codecov.io/gh/lkaratl/quin/graph/badge.svg?token=VK389L3N3V)](https://codecov.io/gh/lkaratl/quin)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## Features

| Feature                     | Plugin                                                     | Description                                                                                           |
|-----------------------------|------------------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| Navigate cursor with keys   | [navigation_grid](./plugins/navigation_grid/README.md)     | Move a cursor to the desired region of the screen by pressing a sequence of keys                      |
| Control mouse with keyboard | [keyboard_to_mouse](./plugins/keyboard_to_mouse/README.md) | Simulate mouse actions using keyboard                                                                 |
| Tray                        | -                                                          | Displays the program running in the background as a tray and allows to interact with it via tray menu |
| Auto startup                | -                                                          | Automatically launches the program when the operating system  starts                                  |

## Plugins

| Plugin                                                     | Windows            | Linux           | MacOS           |
|------------------------------------------------------------|--------------------|-----------------|-----------------|
| tray                                                       | :heavy_check_mark: | :x:             | :x:             |
| [global_input](./plugins/input/global/README.md)           | :heavy_check_mark: | :grey_question: | :grey_question: |
| [input_sequence](./plugins/input/sequence/README.md)       | :heavy_check_mark: | :grey_question: | :grey_question: |
| [gui](./plugins/gui/README.md)                             | :heavy_check_mark: | :grey_question: | :grey_question: |
| overlay                                                    | :heavy_check_mark: | :grey_question: | :grey_question: |
| [keyboard_to_mouse](./plugins/keyboard_to_mouse/README.md) | :heavy_check_mark: | :grey_question: | :grey_question: |
| [navigation_grid](./plugins/navigation_grid/README.md)     | :heavy_check_mark: | :grey_question: | :grey_question: |
| [mouse_output](./plugins/output/mouse/README.md)           | :heavy_check_mark: | :grey_question: | :grey_question: |

## License

This project is licensed under the [GNU license](LICENSE).
