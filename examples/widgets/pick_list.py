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
                self.__selected,
                'select',
                ['Python', 'Rust', 'both', 'neither'],
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
            case 'select', 'neither':
                self.__enabled = False
                return [reenable()]
            case 'select', value:
                self.__selected = value
            case 'enable':
                self.__enabled = True


async def reenable():
    await sleep(2.0)
    return 'enable'


if __name__ == '__main__':
    PickListExample().run()
