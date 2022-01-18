from pyiced import IcedApp, row, Settings, text, WindowSettings


class RowExample(IcedApp):
    class settings(Settings):
        class window(WindowSettings):
            size = (640, 320)

    def title(self):
        return 'A Row'

    def view(self):
        return row(
            [text('Hello,'), text('world!')],
            padding=120, spacing=80,
        )


if __name__ == '__main__':
    RowExample().run()
