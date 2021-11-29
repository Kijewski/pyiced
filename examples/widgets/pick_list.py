from asyncio import sleep

from pyiced import (
    Align, container, IcedApp, Length, pick_list, PickListState, text,
)


class PickListExample(IcedApp):
    class settings:
        class window:
            size = (640, 320)

    def __init__(self):
        self.__pick_list_state = PickListState()
        self.__selected = None
        self.__enabled = True

    def title(self):
        return 'A Pick List'

    def view(self):
        if self.__enabled:
            element = pick_list(
                self.__pick_list_state,
                ['Python', 'Rust', 'both', 'neither'],
                self.__selected,
                self.__select,
            )
        else:
            element = text(':-(')

        return container(
            element,
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    def update(self, msg, clipboard):
        match msg:
            case 'enable':
                self.__enabled = True
            case 'disable':
                self.__enabled = False
                return [reenable()]

    def __select(self, value):
        if value == 'neither':
            self.__selected = None
            return 'disable'

        self.__selected = value


async def reenable():
    await sleep(2.0)
    return 'enable'


if __name__ == '__main__':
    PickListExample().run()
