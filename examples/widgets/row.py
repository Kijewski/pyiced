from pyiced import IcedApp, row, text


class RowExample(IcedApp):
    class settings:
        class window:
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
