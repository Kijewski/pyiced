from pyiced import (
    Align, Color, container, ContainerStyle, IcedApp, Length, text, Subscription
)

import asyncio
import urllib.parse

FONT_HOST = 'fonts.cdnfonts.com'
FONT_PATH = '/s/16054/Yellowtail-Regular.ttf'

async def load_font():
    print('AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA')
    reader, writer = await asyncio.open_connection(FONT_HOST, 443, ssl=True)

    query = (
        f"HEAD {FONT_PATH} HTTP/1.0\r\n"
        f"Host: {FONT_HOST}\r\n"
        f"\r\n"
    )

    writer.write(query.encode('US-ASCII'))
    while True:
        line = await reader.readline()
        if not line:
            break

        line = line.decode('latin1').rstrip()
        if line:
            print(f'HTTP header> {line}')

    # Ignore the body, close the socket
    writer.close()


class FontExample(IcedApp):
    def __init__(self):
        self.__font = None

    def new(self):
        print('TEST0')
        return load_font()

    def title(self):
        return 'Styling'

    def view(self):
        return container(
            text('Hello, world!', size=80, font=self.__font),
            style=ContainerStyle(
                text_color=Color(0.95, 0.87, 0.22),
                background=Color(0.38, 0.60, 0.23),
            ),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    def update(self, datum):
        async def test():
            print('XXXX', datum, 'YYYY')
        return [test()]

    def subscriptions(self):
        return [Subscription.UNCAPTURED]


if __name__ == '__main__':
    FontExample().run()
