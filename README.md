[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Kijewski/pyiced/CI)](https://github.com/Kijewski/pyiced/actions/workflows/ci.yml)
[![Documentation Status](https://readthedocs.org/projects/pyiced/badge/?version=latest)](https://pyiced.readthedocs.io/)
[![PyPI](https://img.shields.io/pypi/v/pyiced)](https://pypi.org/project/pyiced/)
![PyPI - Python Version](https://img.shields.io/pypi/pyversions/pyiced?color=informational)
![OS - Windows | Linux](https://img.shields.io/badge/os-windows%20|%20linux-informational.svg)
[![License](https://img.shields.io/pypi/l/pyiced?color=informational)](/LICENSES/MIT.txt)

Python bindings for **[Iced](https://github.com/iced-rs/iced)**.

Iced is a cross-platform GUI library focused on simplicity and type-safety. Inspired by Elm.


Installation
------------

**Precompiled wheel:**

```sh
$ pip install pyiced
```

**From source:**

```sh
$ pip install .
```

To install from source you need to have a recent version of [**Rust**](https://www.rust-lang.org/) installed in your $PATH.

[**Rustup**](https://rustup.rs/) is probably the most easy to use option to install and update [**Rust**](https://www.rust-lang.org/) on your system.


Quick Example
-------------

[![](https://raw.githubusercontent.com/Kijewski/pyiced/v0.3.0a4/examples/counter.png "Basic example: A counter.")](https://github.com/Kijewski/pyiced/blob/v0.3.0a4/examples/counter.py)

```py
from pyiced import (
    Align, button, ButtonState, column, container, IcedApp, Length, text,
)


class ExampleApp(IcedApp):
    def __init__(self):
        self.__incr_button_state = ButtonState()
        self.__decr_button_state = ButtonState()
        self.__value = 0

    def title(self):
        return 'Counter'

    def view(self):
        increment_button = button(
            self.__incr_button_state,  # To track the state across redraws.
            text('Increment'),         # This is content on the button.
            on_press='incr',           # This value is received in update().
        )
        value_label = text(f'{self.__value}', size=50)
        decrement_button = button(
            self.__decr_button_state,
            text('Decrement'),
            on_press='decr',
        )
        return container(
            column(
                [increment_button, value_label, decrement_button],
                align_items=Align.CENTER,
            ),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    def update(self, msg, clipboard):
        # When an event occurs, this method is called.
        # It can optionally return a list of async functions,
        # to handle the event.
        match msg:
            case 'incr':
                self.__value += 1
            case 'decr':
                self.__value -= 1


if __name__ == '__main__':
    # This function only returns if there is an error on start-up.
    # Otherwise the program gets terminated when the window is closed.
    ExampleApp().run()
```


Bigger Example
--------------

[![](https://raw.githubusercontent.com/Kijewski/pyiced/v0.3.0a4/examples/chess.png "A bigger example: Two-player online chess.")](https://github.com/Kijewski/pyiced/blob/v0.3.0a4/examples/chess.py)

Please find the source code in [examples/chess.py](https://github.com/Kijewski/pyiced/blob/v0.3.0a4/examples/chess.py).
