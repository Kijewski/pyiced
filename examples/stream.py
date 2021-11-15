from asyncio import sleep

from pyiced import column, IcedApp, stream, text


class StreamExample(IcedApp):
    def __init__(self):
        self.__stream = stream(self.__generator_func())
        self.__index = 0

    class settings:
        class window:
            size = (640, 40)

    def title(self):
        return 'Stream Example'

    def view(self):
        return column([text(f'Index: {self.__index / 10:.1f}')])

    def subscriptions(self):
        if self.__stream is not None:
            return [self.__stream]

    def update(self, msg):
        match msg:
            case 'done':
                self.__stream = None
            case int(index):
                self.__index = index

    async def __generator_func(self):
        for i in range(1, 101):
            yield i
            await sleep(0.1)
        yield 'done'


if __name__ == '__main__':
    StreamExample().run()
