from asyncio import open_connection
from contextlib import closing

from pyiced import (
    Align, Color, container, ContainerStyle, Font, IcedApp, Length, text,
)


class AsyncMessageExample(IcedApp):
    def __init__(self):
        self.__font = None

    class settings:
        class window:
            size = (640, 320)

    def title(self):
        return 'Asynchronous Messages'

    def new(self):
        return [load_font()]

    def update(self, msg, clipboard):
        match msg:
            case ('Font', font):
                self.__font = font

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


async def load_font():
    FONT_NAME = 'Yellowtail'
    FONT_HOST = 'fonts.cdnfonts.com'
    FONT_PATH = '/s/16054/Yellowtail-Regular.ttf'

    query = (
        f"GET {FONT_PATH} HTTP/1.0\r\n"
        f"Host: {FONT_HOST}\r\n"
        f"Connection: closed\r\n"
        f"\r\n"
    ).encode('US-ASCII')

    reader, writer = await open_connection(FONT_HOST, 443, ssl=True)
    with closing(writer):
        writer.write(query)
        await writer.drain()
        while (await reader.readline()) != b'\r\n':
            continue

        data = await reader.read()
    await writer.wait_closed()

    return ('Font', Font(FONT_NAME, data))


if __name__ == '__main__':
    AsyncMessageExample().run()
