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

    def view(self):
        increment_button = button(
            self.__incr_button_state,  # To track the state across redraws.
            text('Increment'),         # This is content on the button.
            on_press=Message('incr'),  # This value is received in update().
        )
        value_label = text(f'{self.__value}', size=50)
        decerement_button = button(
            self.__decr_button_state,
            text('Decrement'),
            on_press=Message('decr'),
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

    def update(self, message):
        # When an event occurs, this method is called.
        # It can optionally return a list of async functions,
        # to handle the event.
        match message:
            case Message('incr'):
                self.__value += 1
            case Message('decr'):
                self.__value -= 1


if __name__ == '__main__':
    # This function only returns if there is an error on start-up.
    # Otherwise the program gets terminated when the window is closed.
    App().run()
