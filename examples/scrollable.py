#!/usr/bin/env python3

from pyiced import (
    Align, container, IcedApp, Length, row, scrollable, ScrollableState, space, text,
)


class App(IcedApp):
    def __init__(self):
        class Alt1:
            title = 'Default Scrollbar'
            scrollable = ScrollableState()
            scrollbar_width = None
            scrollbar_margin = None
            scroller_width = None
        class Alt2:
            title = 'Slimmed & Margin'
            scrollable = ScrollableState()
            scrollbar_width=  4
            scrollbar_margin = 3
            scroller_width = 4
        class Alt3:
            title = 'Wide Scroller'
            scrollable = ScrollableState()
            scrollbar_width = 4
            scrollbar_margin = None
            scroller_width = 10
        class Alt4:
            title = 'Narrow Scroller'
            scrollable = ScrollableState()
            scrollbar_width = 10
            scrollbar_margin = None
            scroller_width = 4
        self.__variants = [Alt1, Alt2, Alt3, Alt4]

    def title(self):
        return 'Scrollable'

    def view(self):
        variants = [
            scrollable(
                variant.scrollable,
                [
                    text(variant.title),
                    (
                        text(f'scrollbar_width: {variant.scrollbar_width}')
                        if variant.scrollbar_width else
                        None
                    ),
                    (
                        text(f'scrollbar_margin: {variant.scrollbar_margin}')
                        if variant.scrollbar_margin else
                        None
                    ),
                    (
                        text(f'scroller_width: {variant.scroller_width}')
                        if variant.scroller_width else
                        None
                    ),
                    space(height=Length.units(100)),
                    text(
                        "Some content that should wrap within the "
                        "scrollable. Let's output a lot of short words, so "
                        "that we'll make sure to see how wrapping works "
                        "with these scrollbars."
                    ),
                    space(height=Length.units(1200)),
                    text("Middle"),
                    space(height=Length.units(1200)),
                    text("Bottom"),
                ],
                padding=10, spacing=10, width=Length.FILL, height=Length.FILL,
                scrollbar_width=variant.scrollbar_width, scrollbar_margin=variant.scrollbar_margin,
                scroller_width=variant.scroller_width,
            )
            for variant in self.__variants
        ]
        return container(
            row(variants, spacing=20, width=Length.FILL, height=Length.FILL),
            width=Length.FILL, height=Length.FILL, align_x=Align.CENTER, align_y=Align.CENTER,
        )


if __name__ == '__main__':
    App().run()
