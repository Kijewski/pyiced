from asyncio import open_connection
from contextlib import closing

from pyiced import Align, container, IcedApp, image, ImageHandle, Length, text


class ImageExample(IcedApp):
    def __init__(self):
        self.__handle = None

    class settings:
        class window:
            size = (640, 320)

    def title(self):
        return 'An Image'

    def new(self):
        return load_image()

    def update(self, msg, clipboard):
        match msg:
            case ('ImageHandle', handle):
                self.__handle = handle

    def view(self):
        if self.__handle is None:
            return text('Loading …')

        return container(
            image(
                self.__handle,
                height=Length.units(300),
                width=Length.units(600),  # the aspect ratio is preserved
            ),
            align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )


async def load_image():
    HOST = 'upload.wikimedia.org'
    PATH = '/wikipedia/de/b/bb/Png-logo.png'

    query = (
        f"GET {PATH} HTTP/1.0\r\n"
        f"Host: {HOST}\r\n"
        f"Connection: closed\r\n"
        f"\r\n"
    ).encode('US-ASCII')

    reader, writer = await open_connection(HOST, 443, ssl=True)
    with closing(writer):
        writer.write(query)
        while (await reader.readline()) != b'\r\n':
            continue

        data = await reader.read()
    await writer.wait_closed()
    return ('ImageHandle', ImageHandle.from_memory(data))


if __name__ == '__main__':
    ImageExample().run()
