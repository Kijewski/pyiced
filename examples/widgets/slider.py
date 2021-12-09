from pyiced import (
    Align, column, container, IcedApp, Length, SliderState, slider, text,
)


class SliderApp(IcedApp):
    class settings:
        class window:
            size = (640, 320)

    def __init__(self):
        self.__state = SliderState()
        self.__value = 0.5
        self.__messages = [' '] * 10

    def title(self):
        return 'Slider Example'

    def view(self):
        return container(
            column(
                [
                    text(f'{self.__value * 100:.1f} %'),
                    slider(
                        'slider', self.__state, 0, 1, self.__value, 0.0001,
                        width=Length.units(200),
                    ),
                    text(' '),
                    text('Last values:'),
                    *map(text, self.__messages),
                ],
                align_items=Align.CENTER,
            ),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    def update(self, msg, clipboard):
        match msg:
            case 'slider', value:
                self.__value = value
            case 'slider', None, 'release':
                self.__messages.pop()
                self.__messages[:0] = (f'{self.__value * 100:.1f} %',)


if __name__ == '__main__':
    SliderApp().run()
