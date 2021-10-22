from pyiced import (
    Align, button, ButtonState, column, container, Message, IcedApp, Length,
    text,
)


class App(IcedApp):
    def __init__(self):
        self.__incr_button_state = ButtonState()
        self.__decr_button_state = ButtonState()
        self.__value = 0

    def title(self):
        return 'Counter'

    def update(self, message):
        match message:
            case Message(python='increment'):
                self.__value += 1
            case Message(python='decrement'):
                self.__value -= 1

    def view(self):
        increment_button = button(
            self.__incr_button_state,
            text('Increment'),
            on_press=Message('increment'),
        )
        value_label = text(f'{self.__value}', size=50)
        decerement_button = button(
            self.__decr_button_state,
            text('Decrement'),
            on_press=Message('decrement'),
        )
        return container(
            column([
                increment_button,
                value_label,
                decerement_button,
            ]),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )


if __name__ == '__main__':
    App().run()
