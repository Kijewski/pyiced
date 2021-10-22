#!/usr/bin/env python3

from pyiced import button, ButtonState, column, container, Message, IcedApp, Length, text


class App(IcedApp):
    def __init__(self):
        self.__incr_button_state = ButtonState()
        self.__decr_button_state = ButtonState()
        self.__value = 0

    def title(self):
        return 'Counter'

    def update(self, message):
        match message:
            case Message(python='incr'):
                self.__value += 1
            case Message(python='decr'):
                self.__value -= 1

    def view(self):
        return container(
            column([
                button(self.__incr_button_state, text('Increment'), on_press=Message('incr')),
                text(f'{self.__value}', size=50),
                button(self.__decr_button_state, text('Decrement'), on_press=Message('decr')),
            ]),
            padding=20, center_x=True, center_y=True, width=Length.FILL, height=Length.FILL,
        )


if __name__ == '__main__':
    App().run()
