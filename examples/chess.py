from asyncio import Future, open_connection, start_server
from contextlib import closing
from os.path import abspath, dirname, join
from traceback import print_exc

from pyiced import (
    Align, ContainerStyle, button, ButtonState, ButtonStyle, ButtonStyleSheet,
    Color, column, container, HorizontalAlignment, IcedApp, Length, no_element,
    row, stream, svg, SvgHandle, text, tooltip, TooltipPosition, text_input,
    TextInputState,
)


class ChessExample(IcedApp):
    def new(self):
        # select role:
        self.__role = None
        self.__select_role_btns = [
            ButtonState(),
            ButtonState(),
            ButtonState(),
        ]
        self.__subscription = None

        # server role:
        self.__server_address = None

        # client role:
        self.__client_inputs = [
            TextInputState(),
            TextInputState(),
            ButtonState(),
        ]

        # playing:
        self.__writer = None
        self.__pieces = [
            [
                'Chess_tile_rd.svg',
                'Chess_tile_nd.svg',
                'Chess_tile_bd.svg',
                'Chess_tile_qd.svg',
                'Chess_tile_kd.svg',
                'Chess_tile_bd.svg',
                'Chess_tile_nd.svg',
                'Chess_tile_rd.svg',
            ],
            ['Chess_tile_pd.svg'] * 8,
            *([None] * 8 for _ in range(4)),
            ['Chess_tile_pl.svg'] * 8,
            [
                'Chess_tile_rl.svg',
                'Chess_tile_nl.svg',
                'Chess_tile_bl.svg',
                'Chess_tile_ql.svg',
                'Chess_tile_kl.svg',
                'Chess_tile_bl.svg',
                'Chess_tile_nl.svg',
                'Chess_tile_rl.svg',
            ],
        ]
        self.__pieces_root = join(
            dirname(abspath(__file__)),
            'chess-pieces',
        )
        self.__button_states = [
            [ButtonState() for _ in range(8)] for _ in range(8)
        ]
        self.__selected = None

    def title(self):
        return 'Chess Example'

    def subscriptions(self):
        return [self.__subscription]

    def view(self):
        match self.__role:
            case 'server':
                elem = self.__view_server()
            case 'client':
                elem = self.__view_client()
            case 'playing':
                elem = self.__view_playing()
            case _:
                elem = self.__view_select_role()

        return container(
            elem,
            width=Length.FILL,
            height=Length.FILL,
            align_x=Align.CENTER,
            align_y=Align.CENTER,
        )

    def background_color(self):
        return Color(0.627, 0.612, 0.616)

    def __view_select_role(self):
        alone_state, server_state, client_state = self.__select_role_btns
        return container(
            column(
                [
                    text('Play as:'),
                    button(
                        alone_state,
                        text('Alone'),
                        ('role', 'alone'),
                        padding=4,
                    ),
                    button(
                        server_state,
                        text('Server'),
                        ('role', 'server'),
                        padding=4,
                    ),
                    button(
                        client_state,
                        text('Client'),
                        ('role', 'client'),
                        padding=4,
                    ),
                ],
                spacing=16,
                align_items=Align.CENTER,
            ),
            style=ContainerStyle(background=Color.WHITE),
            padding=32,
        )

    def __view_server(self):
        if not self.__server_address:
            return text('Opening server …')

        host, port = self.__server_address
        return container(
            column(
                [
                    text('Waiting for client:'),
                    text(f'Your IP: {host}'),
                    text(f'Your port: {port}'),
                ],
                spacing=16,
                align_items=Align.CENTER,
            ),
            style=ContainerStyle(background=Color.WHITE),
            padding=32,
        )

    def __view_client(self):
        if not self.__server_address:
            return text('Connecting to server …')

        def set_value(index, value):
            self.__server_address[index] = value

        return container(
            column(
                [
                    text('Connect to server:'),
                    row(
                        [
                            text_input(
                                self.__client_inputs[0],
                                'Host / IP address',
                                self.__server_address[0],
                                lambda value: set_value(0, value),
                                padding=4,
                                width=Length.units(148),
                            ),
                            text_input(
                                self.__client_inputs[1],
                                'Port',
                                self.__server_address[1],
                                lambda value: set_value(1, value),
                                padding=4,
                                width=Length.units(148),
                            ),
                        ],
                        spacing=16,
                    ),
                    button(
                        self.__client_inputs[2],
                        text(
                            'Connect',
                            horizontal_alignment=HorizontalAlignment.CENTER,
                        ),
                        ('client', self.__server_address),
                        padding=16,
                        width=Length.units(328),
                    ),
                ],
                spacing=16,
                align_items=Align.CENTER,
            ),
            style=ContainerStyle(background=Color.WHITE),
            padding=32,
        )

    def __view_playing(self):
        return row(
            [
                column(
                    [self.__cell_at(x, y) for y in range(8)],
                    width=Length.fill_portion(1),
                    height=Length.FILL,
                )
                for x in range(8)
            ],
            width=Length.units(8 * 80),
            height=Length.units(8 * 80),
        )

    def __cell_at(self, x, y):
        piece = self.__pieces[y][x]
        if piece:
            elem = svg(
                SvgHandle.from_path(join(self.__pieces_root, piece)),
            )
        else:
            elem = no_element()

        style = ButtonStyle(
            background=(
                Color(0.200, 0.600, 0.800)
                if self.__selected == (x, y) else
                Color(1.000, 0.808, 0.620)
                if (x + y) & 1 else
                Color(0.820, 0.545, 0.278)
            ),
            shadow_offset=(0, 0),
        )
        return tooltip(
            button(
                self.__button_states[y][x],
                container(
                    elem,
                    align_x=Align.CENTER,
                    align_y=Align.CENTER,
                    width=Length.FILL,
                    height=Length.FILL,
                ),
                ('select', x, y, True),
                width=Length.fill_portion(1),
                height=Length.fill_portion(1),
                style=ButtonStyleSheet(style, style, style, style),
            ),
            f'{chr(ord("a") + 7 - y)}{x + 1}',
            TooltipPosition.FOLLOW_CURSOR,
        )

    def update(self, msg, clipboard):
        match msg:
            case ('select', x, y, do_notify):
                if self.__selected == (x, y):
                    # deselect
                    self.__selected = None
                elif self.__selected:
                    # move
                    (x0, y0) = self.__selected
                    self.__pieces[y][x] = self.__pieces[y0][x0]
                    self.__pieces[y0][x0] = None
                    self.__selected = None
                elif self.__pieces[y][x]:
                    # select
                    self.__selected = (x, y)

                if do_notify and self.__writer:
                    async def write():
                        self.__writer.write(b'%d %d\n' % (x, y))
                        await self.__writer.drain()
                    return [write()]

            case ('role', 'alone'):
                self.__role = 'playing'

            case ('role', 'server'):
                self.__role = 'server'
                self.__subscription = stream(self.__role_server())

            case ('role', 'client'):
                self.__role = 'client'
                self.__server_address = ['0.0.0.0', '']

            case ('server', (host, port)):
                self.__server_address = host, port

            case ('client', (host, port)):
                self.__server_address = None
                self.__role = 'server'
                self.__subscription = stream(self.__role_client(host, port))

            case ('connected', (reader, writer)):
                self.__writer = writer
                self.__subscription = stream(self.__read_connection(reader))
                self.__role = 'playing'

    async def __read_connection(self, reader):
        while not reader.at_eof():
            line = await reader.readline()
            if not line:
                break
            x, y = line.split()
            yield 'select', int(x), int(y), False

    async def __role_client(self, host, port):
        try:
            yield 'connected', await open_connection(host, port)
        except Exception:
            print_exc()
            yield 'role', 'client'

    async def __role_server(self):
        query = (
            b'GET / HTTP/1.0\r\n'
            b'Host: whatismyip.akamai.com\r\n'
            b'Connection: closed\r\n'
            b'\r\n'
        )
        reader, writer = await open_connection('whatismyip.akamai.com', 80)
        with closing(writer):
            writer.write(query)
            await writer.drain()
            while (await reader.readline()) != b'\r\n':
                continue
            hostname = (await reader.read()).decode('US-ASCII').strip()
        await writer.wait_closed()

        client = Future()
        server = await start_server(
            lambda reader, writer: client.set_result((reader, writer)),
            '0.0.0.0',
            0,
        )
        port = next(iter(server.sockets)).getsockname()[1]
        yield 'server', (hostname, port)
        yield 'connected', await client


if __name__ == '__main__':
    ChessExample().run()
