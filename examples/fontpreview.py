from bisect import bisect_left, bisect_right

from pyiced import (
    Align, column, container, IcedApp, Length, PickListState, pick_list,
    row, text, text_input, TextInputState, systemfonts, findfont,
)


class FontPreview(IcedApp):
    class settings:
        default_text_size = 24

    def __init__(self):
        self.__font_bold = findfont(
            ['Arial', 'Noto Sans', 'DejaVu Sans', 'sans-serif'],
            weight='bold',
        ).load()

        self.__family_prefix_state = TextInputState()
        self.__family_prefix = ''
        self.__family_state = PickListState()
        self.__family = ''
        self.__families = sorted(
            {fontid.family for fontid in systemfonts()} |
            {'serif', 'sans-serif', 'cursive', 'fantasy', 'monospace'}
        )

        self.__weight = 'normal'
        self.__weight_state = PickListState()

        self.__stretch = 'normal'
        self.__stretch_state = PickListState()

        self.__style = 'normal'
        self.__style_state = PickListState()

    def title(self):
        return 'Font Preview'

    def view(self):
        if self.__family_prefix:
            def family_key(s):
                return cmp(s[:len(family_prefix)].lower(), family_prefix)

            family_prefix = self.__family_prefix.lower()
            families_start = bisect_left(
                self.__families, 0,
                key=family_key,
            )
            families_end = bisect_right(
                self.__families, 0, families_start,
                key=family_key,
            )
            families = self.__families[families_start:families_end][:10]
        else:
            families = None
        family = column(
            [
                text('font-family:', font=self.__font_bold),
                text_input(
                    'family_prefix',
                    self.__family_prefix_state,
                    '',
                    self.__family_prefix,
                    padding=4,
                ),
                pick_list(
                    'family',
                    self.__family_state,
                    self.__family,
                    families or [
                        'serif', 'sans-serif', 'cursive', 'fantasy',
                        'monospace',
                    ],
                ),
            ],
            max_width=300,
            spacing=10,
        )
        weight = column(
            [
                text('font-weight:', font=self.__font_bold),
                pick_list(
                    'weight',
                    self.__weight_state,
                    self.__weight,
                    [
                        'thin', 'extra-light', 'light', 'normal',
                        'medium', 'semibold', 'bold', 'extra-bold',
                        'black',
                    ],
                )
            ],
            max_width=300,
            spacing=10,
        )
        stretch = column(
            [
                text('font-stretch:', font=self.__font_bold),
                pick_list(
                    'stretch',
                    self.__stretch_state,
                    self.__stretch,
                    [
                        'ultra-condensed', 'extra-condensed', 'condensed',
                        'semi-condensed', 'normal', 'semi-expanded',
                        'expanded', 'extra-expanded', 'ultra-expanded',
                    ],
                )
            ],
            max_width=300,
            spacing=10,
        )
        style = column(
            [
                text('font-style:', font=self.__font_bold),
                pick_list(
                    'style',
                    self.__style_state,
                    self.__style,
                    ['normal', 'italic', 'oblique'],
                )
            ],
            max_width=300,
            spacing=10,
        )
        search = row([family, weight, stretch, style], spacing=10)

        font = findfont(
            self.__family, self.__weight, self.__stretch, self.__style,
        )
        font_data = column(
            [
                text(
                    'Found font:',
                    font=self.__font_bold,
                ),
                row(
                    [text('family:'), text(font.family)],
                    spacing=4,
                ),
                row(
                    [text('weight:'), text(repr(font.weight))],
                    spacing=4,
                ),
                row(
                    [text('stretch:'), text(repr(font.stretch))],
                    spacing=4,
                ),
                row(
                    [text('style:'), text(repr(font.style))],
                    spacing=4,
                ),
            ],
            spacing=10,
        )

        font = font.load()
        font_preview = column(
            [
                text(
                    'Preview:',
                    font=self.__font_bold,
                ),
                text(
                    '!"#$%&\'()*+,-./0123456789:;<=>?@'
                    ' ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`'
                    ' abcdefghijklmnopqrstuvwxyz{|}~',
                    font=font,
                ),
                text(
                    'The quick brown fox jumps over the lazy dog.',
                    font=font,
                ),
                text(
                    'Zwölf laxe Typen qualmen verdächtig süße Objekte.',
                    font=font,
                ),
                text(
                    'Dès Noël où un zéphyr haï me vêt de glaçons '
                    'würmiens, je dîne d’exquis rôtis de bœuf au kir '
                    'à l’aÿ d’âge mûr & cætera !',
                    font=font,
                ),
                text(
                    'Stróż pchnął kość w quiz gędźb vel fax myjń.',
                    font=font,
                ),
                text(
                    'Příliš žluťoučký kůň úpěl ďábelské ódy.',
                    font=font,
                ),
                text(
                    'Pijamalı hasta yağız şoföre çabucak güvendi',
                    font=font,
                ),
                text(
                    'Съешь ещё этих мягких французских булок, да '
                    'выпей чаю',
                    font=font,
                ),
            ],
            spacing=10,
        )

        return container(
            column(
                [
                    search,
                    row(
                        [
                            font_data,
                            font_preview,
                        ],
                        spacing=20,
                    ),
                ],
                spacing=20,
            ),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    def update(self, msg, clipboard):
        match msg:
            case ('family_prefix', family_prefix):
                self.__family_prefix = family_prefix
            case ('family', family):
                self.__family = family
            case ('weight', weight):
                self.__weight = weight
            case ('stretch', stretch):
                self.__stretch = stretch
            case ('style', style):
                self.__style = style


def cmp(a, b):
    return (a > b) - (a < b)


if __name__ == '__main__':
    FontPreview().run()
