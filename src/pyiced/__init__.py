# Copyright (c) 2021 René Kijewski <rene.[surname]@fu-berlin.de>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

from abc import ABCMeta, abstractmethod
from contextlib import contextmanager
from asyncio import Event, get_event_loop, run as _run
from queue import Queue
from threading import Thread
from typing import Awaitable, Iterable, NoReturn, Optional, Tuple

from . import _pyiced


# KEEP SYNCHRONOUS TO MODULE EXPORTS
__all__ = [
    # states
    'ButtonState', 'PickListState', 'ScrollableState', 'SliderState', 'TextInputState',

    # widgets
    'Element', 'no_element', 'button', 'checkbox', 'column', 'container', 'image', 'pick_list',
    'progress_bar', 'radio', 'row', 'rule', 'scrollable', 'slider', 'space', 'svg', 'text',
    'text_input', 'tooltip',

    # wrapped
    'Align', 'Color', 'Font', 'HorizontalAlignment', 'ImageHandle', 'Length', 'Line', 'Message',
    'SliderHandle', 'SliderHandleShape', 'SvgHandle', 'TooltipPosition', 'VerticalAlignment',

    # styles
    'ButtonStyleSheet', 'CheckboxStyle', 'CheckboxStyleSheet', 'ContainerStyleSheet',
    'PaneGridStyleSheet', 'ProgressBarStyleSheet', 'SliderStyle', 'SliderStyleSheet',

    # subscription
    'Subscription',
]

for name in __all__:
    exec(f'{name} = _pyiced.{name}')

__all__ += [
    # interfaces
    'IcedApp', 'Settings', 'WindowSettings',

    # aliases
    'ButtonStyle', 'ContainerStyle', 'PaneGridStyle', 'ProgressBarStyle',
]

__author__ = _pyiced.__author__
__version__ = _pyiced.__version__
__license__ = _pyiced.__license__

Command = Awaitable[Optional[Message]]
Commands = Iterable[Command]

ButtonStyle = ButtonStyleSheet
ContainerStyle = ContainerStyleSheet
PaneGridStyle = PaneGridStyleSheet
ProgressBarStyle = ProgressBarStyleSheet


class WindowSettings:
    @property
    def size(self) -> Tuple[int, int]:
        '''
        Dimensions of the newly crated window.
        '''
        return (1024, 768)

    @property
    def min_size(self) -> Optional[Tuple[int, int]]:
        '''
        TODO
        '''
        return None

    @property
    def max_size(self) -> Optional[Tuple[int, int]]:
        '''
        TODO
        '''
        return None

    @property
    def resizable(self) -> bool:
        '''
        TODO
        '''
        return True

    @property
    def decorations(self) -> bool:
        '''
        TODO
        '''
        return True

    @property
    def transparent(self) -> bool:
        '''
        TODO
        '''
        return False

    @property
    def always_on_top(self) -> bool:
        '''
        TODO
        '''
        return False

    # TODO: pub icon: Option<Icon>,


class Settings:
    @property
    def default_text_size(self) -> int:
        '''
        TODO
        '''
        return 20

    @property
    def exit_on_close_request(self) -> bool:
        '''
        TODO
        '''
        return True

    @property
    def antialiasing(self) -> bool:
        '''
        TODO
        '''
        return True

    # TODO: default_font

    @property
    def window(self) -> Optional[WindowSettings]:
        '''
        TODO
        '''
        return None


class IcedApp(metaclass=ABCMeta):
    def run(self, *, run=_run) -> NoReturn:
        '''
        TODO
        '''
        return run_iced(self, run=run)

    def settings(self) -> Optional[Settings]:
        '''
        TODO
        '''
        return None

    def new(self) -> Optional[Commands]:
        '''
        TODO
        '''
        return None

    def title(self) -> str:
        '''
        TODO
        '''
        return f'PyIced {__version__}'

    def should_exit(self) -> bool:
        '''
        TODO
        '''
        return False

    def scale_factor(self) -> float:
        '''
        TODO
        '''
        return 1.0

    def fullscreen(self) -> bool:
        '''
        TODO
        '''
        return False

    def update(self, msg: Message) -> Optional[Commands]:
        '''
        TODO
        '''
        return None

    def subscriptions(self) -> Optional[Iterable[Subscription]]:
        '''
        TODO
        '''
        return None

    def background_color(self) -> Optional[Color]:
        '''
        TODO
        '''
        return Color.WHITE

    @abstractmethod
    def view(self) -> Element:
        '''
        TODO
        '''
        ...


def run_iced(app: IcedApp, *, run=_run) -> NoReturn:
    '''
    TODO
    '''
    with in_async_loop(run) as loop:
        return _pyiced.run_iced(
            pyloop=loop,
            new=app.new,
            title=app.title,
            update=app.update,
            should_exit=app.should_exit,
            scale_factor=app.scale_factor,
            fullscreen=app.fullscreen,
            view=app.view,
            subscriptions=app.subscriptions,
            settings=app.settings,
            background_color=app.background_color,
        )


async def thread_code(put_task):
    def done():
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
