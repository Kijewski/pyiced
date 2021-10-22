#!/usr/bin/env python3

from pyiced import button, ButtonState, column, container, Message, IcedApp, Length, text,


class App(IcedApp):
    def __init__(self):
        self.__incr_button = button(ButtonState(), text('Increment'), on_press=Message('incr'))
        self.__decr_button = button(ButtonState(), text('Decrement'), on_press=Message('decr'))
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
                self.__incr_button,
                text(f'{self.__value}', size=50),
                self.__decr_button,
            ]),
            padding=20, center_x=True, center_y=True, width=Length.FILL, height=Length.FILL,
        )


if __name__ == '__main__':
    App().run()
