from pyiced import (
    Align, button, ButtonState, ButtonStyle, Color, container,
    ContainerStyle, IcedApp, Length, text,
)


class ButtonExample(IcedApp):
    class settings:
        class window:
            size = (640, 320)

    def __init__(self):
        self.__button_state = ButtonState()

    def title(self):
        return 'A Button'

    def view(self):
        styled_button = button(
            self.__button_state,
            text('Hello, world!', size=40),
            '',
            style=ButtonStyle(
                shadow_offset=(8, 8), border_radius=40, border_width=6,
                background=Color(0.17, 0.17, 0.17),
                border_color=Color(0.95, 0.87, 0.22),
                text_color=Color(1.00, 0.18, 0.13)
            ),
            padding=40,
        )
        return container(
            styled_button,
            style=ContainerStyle(background=Color(0.38, 0.60, 0.23)),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )


if __name__ == '__main__':
    ButtonExample().run()
