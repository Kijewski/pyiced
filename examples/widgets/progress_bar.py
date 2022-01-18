from datetime import timedelta

from pyiced import (
    Align, container, every, IcedApp, Length, progress_bar, Settings,
    WindowSettings,
)


class ProgressBarExample(IcedApp):
    class settings(Settings):
        class window(WindowSettings):
            size = (640, 320)

    def __init__(self):
        self.__value = 0.0

    def title(self):
        return 'A Progress Bar'

    def subscriptions(self):
        if self.__value < 1:
            return [every(timedelta(milliseconds=10), 'progress')]

    def update(self, msg, clipboard):
        match msg:
            case ('progress', _):
                self.__value = (self.__value + 0.001)

    def view(self):
        return container(
            progress_bar(0, 1, self.__value),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )


if __name__ == '__main__':
    ProgressBarExample().run()
