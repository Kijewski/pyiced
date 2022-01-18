from pyiced import (
    Color, column, every, FillMode, IcedApp, Length, row, rule,
    RuleStyleSheet, Settings, text, WindowSettings,
)


class RuleExample(IcedApp):
    class settings(Settings):
        class window(WindowSettings):
            size = (640, 320)

    def new(self):
        self.__percent = 0

    def title(self):
        return 'Rule Example'

    def subscriptions(self):
        return [every(0.010, 'tick')]

    def view(self):
        vertical = column(
            [
                text('top'),
                rule(horizontal=1),
                text('middle'),
                rule(horizontal=80),
                text('bottom'),
            ],
            padding=20, spacing=5,
            width=Length.FILL, height=Length.FILL,
        )
        separator = rule(
            vertical=80,
            style=RuleStyleSheet(
                color=Color(0, 1, 0),
                width=40,
                radius=10,
                fill_mode=FillMode.percent(self.__percent),
            ),
        )
        horizontal = row(
            [
                text('left'),
                rule(vertical=1),
                text('center'),
                rule(vertical=80),
                text('right'),
            ],
            padding=20, spacing=5,
            width=Length.FILL, height=Length.FILL,
        )
        return row([vertical, separator, horizontal])

    def update(self, msg, clipboard):
        match msg:
            case ('tick', _):
                self.__percent = (self.__percent + 1) % 100


if __name__ == '__main__':
    RuleExample().run()
