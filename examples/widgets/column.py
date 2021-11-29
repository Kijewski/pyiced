from pyiced import column, IcedApp, text


class ColumnExample(IcedApp):
    class settings:
        class window:
            size = (640, 320)

    def title(self):
        return 'A Column'

    def view(self):
        return column(
            [text('Hello,'), text('world!')],
            padding=80, spacing=120,
        )


if __name__ == '__main__':
    ColumnExample().run()
