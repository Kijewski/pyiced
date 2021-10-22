from abc import ABCMeta, abstractmethod
from contextlib import contextmanager
from asyncio import Event, get_event_loop, run as _run
from queue import Queue
from threading import Thread
from typing import Awaitable, List, NoReturn, Optional, Tuple

from . import _pyiced


# KEEP SYNCHRONOUS TO MODULE EXPORTS
__all__ = [
    # states
    'ButtonState', 'PickListState', 'ScrollableState', 'SliderState', 'TextInputState',

    # widgets
    'Element', 'no_element', 'button', 'checkbox', 'column', 'container', 'image', 'pick_list',
    'progress_bar', 'radio', 'row', 'rule', 'scrollbar', 'slider', 'space', 'svg', 'text',
    'text_input', 'tooltip',

    # wrapped
    'Align', 'Color', 'Font', 'HorizontalAlignment', 'ImageHandle', 'Length', 'Message',
    'SvgHandle', 'TooltipPosition', 'VerticalAlignment',
]

for name in __all__:
    exec(f'{name} = _pyiced.{name}')

__all__ += ['run_iced', 'IcedApp']

__author__ = _pyiced.__author__
__version__ = _pyiced.__version__

Command = Awaitable[Optional[Message]]
Commands = List[Command]


class WindowSettings:
    @property
    def size(self) -> Tuple[int, int]:
        '''
        Dimensions of the newly crated window.
    
        Returns
        -------
        (int, int)
            size in pixels
        '''
        return (1024, 768)

    @property
    def min_size(self) -> Optional[Tuple[int, int]]:
        return None

    @property
    def max_size(self) -> Optional[Tuple[int, int]]:
        return None

    @property
    def resizable(self) -> bool:
        return True

    @property
    def decorations(self) -> bool:
        return True

    @property
    def transparent(self) -> bool:
        return False

    @property
    def always_on_top(self) -> bool:
        return False

    # TODO: pub icon: Option<Icon>,


class Settings:
    @property
    def default_text_size(self) -> int:
        return 20

    @property
    def exit_on_close_request(self) -> bool:
        return True

    @property
    def antialiasing(self) -> bool:
        return True

    # TODO: default_font

    @property
    def window(self) -> Optional[WindowSettings]:
        return None


class IcedApp(metaclass=ABCMeta):
    def settings(self) -> Optional[Settings]:
        return None

    def new(self) -> Optional[Commands]:
        return None

    def title(self) -> str:
        return f'PyIced {__version__}'

    def should_exit(self) -> bool:
        return False

    def scale_factor(self) -> float:
        return 1.0

    def fullscreen(self) -> bool:
        return False

    def update(self, msg: Message) -> Optional[Commands]:
        return None

    @abstractmethod
    def view(self) -> Element:
        ...


def run_iced(app: IcedApp, *, run=_run) -> NoReturn:
    new = app.new
    title = app.title
    update = app.update
    should_exit = app.should_exit
    scale_factor = app.scale_factor
    fullscreen = app.fullscreen
    view = app.view
    settings = app.settings

    with in_async_loop(run) as loop:
        return _pyiced.run_iced(
            loop, new, title, update, should_exit, scale_factor, fullscreen, view, settings,
        )


async def thread_code(put_task):
    def done(task):
        loop.call_soon_threadsafe(done_event.set)

    done_event = Event()
    loop = get_event_loop()
    put_task.put((loop, done))
    await done_event.wait()


@contextmanager
def in_async_loop(run):
    put_task = Queue(1)
    thread = Thread(None, run, args=(thread_code(put_task),))
    thread.start()
    try:
        (loop, done) = put_task.get()
        try:
            yield loop
        finally:
            done()
    finally:
        thread.join()
