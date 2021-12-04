from os.path import abspath, dirname, join

from pyiced import (
    Align, ContainerStyleSheet, button, ButtonState, ButtonStyleSheet,
    Color, column, container, IcedApp, Length, no_element, row, svg,
    SvgHandle, tooltip, TooltipPosition,
)


class ChessExample(IcedApp):
    def new(self):
        self.__button_states = [
            [ButtonState() for _ in range(8)] for _ in range(8)
        ]
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
            dirname(abspath(__name__)),
            'chess-pieces',
        )
        self.__selected = None

    def title(self):
        return 'Chess Example'

    def view(self):
        rows = row([
            column([
                tooltip(
                    button(
                        self.__button_states[y][x],
                        self.__piece_at(x, y),
                        ('select', x, y),
                        width=Length.units(80),
                        height=Length.units(80),
                        padding=2,
                        style=ButtonStyleSheet(
                            background=(
                                Color(0.2, 0.6, 0.8)
                                if self.__selected == (x, y) else
                                Color(0xff / 255, 0xce / 255, 0x9e / 255)
                                if (x + y) & 1 else
                                Color(0xd1 / 255, 0x8b / 255, 0x47 / 255)
                            ),
                            shadow_offset=(0, 0),
                        )
                    ),
                    f'{chr(ord("a") + 7 - y)}{x + 1}',
                    TooltipPosition.FOLLOW_CURSOR,
                )
                for y in range(8)
            ])
            for x in range(8)
        ])
        return container(
            rows,
            width=Length.FILL,
            height=Length.FILL,
            align_x=Align.CENTER,
            align_y=Align.CENTER,
            style=ContainerStyleSheet(
                background=Color(0xa0 / 255, 0x9c / 255, 0x9d / 255),
            ),
        )

    def update(self, msg, clipboard):
        match msg:
            case ('select', x, y):
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

    def __piece_at(self, x, y):
        piece = self.__pieces[y][x]
        if piece:
            return svg(
                SvgHandle.from_path(join(self.__pieces_root, piece)),
            )
        else:
            return no_element()


if __name__ == '__main__':
    ChessExample().run()
