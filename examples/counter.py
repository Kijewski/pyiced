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
