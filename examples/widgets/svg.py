from asyncio import open_connection
from contextlib import closing

from pyiced import Align, container, IcedApp, Length, svg, SvgHandle, text


class SvgExample(IcedApp):
    def __init__(self):
        self.__handle = None

    class settings:
        class window:
            size = (640, 320)

    def title(self):
        return 'An SVG'

    def new(self):
        return [load_svg()]

    def update(self, msg, clipboard):
        match msg:
            case ('SvgHandle', handle):
                self.__handle = handle

    def view(self):
        if self.__handle is None:
            return text('Loading …')

        return container(
            svg(
                self.__handle,
                height=Length.units(300), width=Length.units(300),
            ),
            align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )


async def load_svg():
    HOST = 'raw.githubusercontent.com'
    PATH = '/iced-rs/iced/master/docs/logo.svg'

    query = (
        f"GET {PATH} HTTP/1.0\r\n"
        f"Host: {HOST}\r\n"
        f"Connection: closed\r\n"
        f"User-Agent: Mozilla/1.22 (compatible; MSIE 2.0; Windows 95)\r\n"
        f"\r\n"
    ).encode('US-ASCII')

    reader, writer = await open_connection(HOST, 443, ssl=True)
    with closing(writer):
        writer.write(query)
        await writer.drain()
        while (await reader.readline()) != b'\r\n':
            continue

        data = await reader.read()
    await writer.wait_closed()

    return ('SvgHandle', SvgHandle.from_memory(data))


if __name__ == '__main__':
    SvgExample().run()
