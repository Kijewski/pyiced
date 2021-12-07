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
                radio(self.__season, 'select', 1, 'Spring'),
                radio(self.__season, 'select', 2, 'Summer'),
                radio(self.__season, 'select', 3, 'Fall'),
                radio(self.__season, 'select', 4, 'Winter'),
            ],
            padding=20, spacing=5,
            width=Length.FILL, height=Length.FILL,
        )


if __name__ == '__main__':
    RadioExample().run()
