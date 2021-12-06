from pyiced import column, css_color, IcedApp, Length, radio, text


class RadioExample(IcedApp):
    class settings:
        class window:
            size = (640, 320)

    def __init__(self):
        self.__season = None

    def title(self):
        return 'Radio Example'

    def background_color(self):
        match self.__season:
            case 1:
                return css_color.MEDIUMSPRINGGREEN
            case 2:
                return css_color.LIGHTGOLDENRODYELLOW
            case 3:
                return css_color.GOLDENROD
            case 4:
                return css_color.GHOSTWHITE

    def update(self, msg, clipboard):
        match msg:
            case 'select', value:
                self.__season = value

    def view(self):
        return column(
            [
                text("What's your favorite season?"),
                radio(1, 'Spring', self.__season, select_season),
                radio(2, 'Summer', self.__season, select_season),
                radio(3, 'Fall', self.__season, select_season),
                radio(4, 'Winter', self.__season, select_season),
            ],
            padding=20, spacing=5,
            width=Length.FILL, height=Length.FILL,
        )


def select_season(value):
    return 'select', value


if __name__ == '__main__':
    RadioExample().run()
