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
from asyncio import get_event_loop, Queue as AsyncQueue, run as _run, run_coroutine_threadsafe
from queue import Queue as SyncQueue
from threading import Thread
from typing import Awaitable, Callable, Iterable, NewType, NoReturn, Optional, Tuple, Union

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
    'Align', 'Clipboard', 'Color', 'FillMode', 'Font', 'HorizontalAlignment', 'ImageHandle',
    'Instant', 'Length', 'Line', 'Message', 'Point', 'Rectangle', 'SliderHandle',
    'SliderHandleShape', 'SvgHandle', 'TextInputCursor', 'TooltipPosition', 'VerticalAlignment',

    # styles
    'ButtonStyleSheet', 'CheckboxStyle', 'CheckboxStyleSheet', 'ContainerStyleSheet',
    'PaneGridStyleSheet', 'PickListMenu', 'PickListStyle', 'PickListStyleSheet',
    'ProgressBarStyleSheet', 'RadioStyle', 'RadioStyleSheet', 'RuleStyleSheet',
    'ScrollableStyleSheet', 'ScrollbarStyle', 'ScrollerStyle', 'Size', 'SliderStyle',
    'SliderStyleSheet', 'TextInputStyle', 'TextInputStyleSheet',

    # subscription
    'every', 'stream', 'Subscription',
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
__license__ = _pyiced.__license__
__version__ = _pyiced.__version__

Command = NewType('Command', Union[Awaitable[Optional[object]], object])
Commands = NewType('Commands', Iterable[Optional[Command]])

ButtonStyle = ButtonStyleSheet
ContainerStyle = ContainerStyleSheet
PaneGridStyle = PaneGridStyleSheet
ProgressBarStyle = ProgressBarStyleSheet
RuleStyle = RuleStyleSheet


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
        The minimum size of the window.
        '''
        return None

    @property
    def max_size(self) -> Optional[Tuple[int, int]]:
        '''
        The maximum size of the window.
        '''
        return None

    @property
    def resizable(self) -> bool:
        '''
        Whether the window should be resizable or not.
        '''
        return True

    @property
    def decorations(self) -> bool:
        '''
        Whether the window should have a border, a title bar, etc. or not.
        '''
        return True

    @property
    def transparent(self) -> bool:
        '''
        Whether the window should be transparent.
        '''
        return False

    @property
    def always_on_top(self) -> bool:
        '''
        Whether the window will always be on top of other windows.
        '''
        return False

    # TODO: pub icon: Option<Icon>,


class Settings:
    @property
    def default_text_size(self) -> int:
        '''
        The text size that will be used by default.
        '''
        return 20

    @property
    def exit_on_close_request(self) -> bool:
        '''
        Whether the :class:`~pyiced.IcedApp` should exit when the user requests the window to close (e.g. the user presses the close button).
        '''
        return True

    @property
    def antialiasing(self) -> bool:
        '''
        If set to true, the renderer will try to perform antialiasing for some primitives.

        Enabling it can produce a smoother result in some widgets, like the Canvas, at a performance cost.
        '''
        return True

    @property
    def window(self) -> Optional[WindowSettings]:
        '''
        The window settings.
        '''
        return None

    @property
    def default_font(self) -> Optional[Font]:
        '''
        The font that will be used by default.

        If `None` or `Font.DEFAULT` is provided, a default system font will be chosen.
        '''
        Font.DEFAULT


class IcedApp(metaclass=ABCMeta):
    def run(self, *, run: Optional[Callable[[Awaitable], None]]=None) -> NoReturn:
        '''
        Runs the application.

        This method will take control of the current thread and will NOT return unless there is an error during startup.

        It should probably be that last thing you call in your main function.

        Arguments
        ---------
        run
            Coroutine executor. Defaults to :func:`asyncio.run()`.
        '''
        return run_iced(self, run=run)

    @property
    def settings(self) -> Optional[Settings]:
        '''
        The initial settings of the program.

        Once queried once.
        '''
        return None

    def new(self) -> Optional[Commands]:
        '''
        Initialize the application.

        You can return :class:`~pyiced.Commands` if you need to perform some async action in the background on startup. This is useful if you want to load state from a file, perform an initial HTTP request, etc.
        '''
        return None

    def title(self) -> str:
        '''
        The current title of the application.

        This title can be dynamic! The runtime will automatically update the title of your application when necessary.
        '''
        return f'PyIced {__version__}'

    def should_exit(self) -> bool:
        '''
        Returns whether the application should be terminated.

        This will kill the Python instance, too.
        '''
        return False

    def scale_factor(self) -> float:
        '''
        Returns the scale factor of the application.

        It can be used to dynamically control the size of the UI at runtime (i.e. zooming).

        For instance, a scale factor of 2.0 will make widgets twice as big, while a scale factor of 0.5 will shrink them to half their size.
        '''
        return 1.0

    def fullscreen(self) -> bool:
        '''
        True if the program should run in fullscreen mode.

        The runtime will automatically transition your application if a new mode is returned.
        '''
        return False

    def update(self, msg: Union[Message, object], clipboard: Clipboard) -> Optional[Commands]:
        '''
        Handles a message and updates the state of the application.

        This is where you define your update logic. All the messages, produced by either user interactions or commands, will be handled by this method.

        Any :class:`~pyiced.Command` returned will be executed immediately in the background.
        '''
        return None

    def subscriptions(self) -> Optional[Iterable[Optional[Subscription]]]:
        '''
        Returns the event :ref:`subscriptions <subscriptions:Event Listening>` for the current state of the application.

        A subscription will be kept alive as long as you keep returning it, and the messages produced will be handled by update.
        '''
        return None

    def background_color(self) -> Optional[Color]:
        '''
        Returns the background color of the application.
        '''
        return Color.WHITE

    @abstractmethod
    def view(self) -> Element:
        '''
        Returns the :ref:`widget <elements:Displayable Elements>` to display in the application.

        These widgets can produce messages based on user interaction.
        '''
        ...


def run_iced(app: IcedApp, *, run=None) -> NoReturn:
    return _pyiced.run_iced(
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
        taskmanager=make_loop(run),
    )


def make_loop(run=None):
    put_task = SyncQueue(1)
    thread = Thread(
        None,
        run if run is not None else _run,
        args=(thread_code(put_task),),
        name='PyIced-AsyncLoop',
    )
    thread.start()
    return put_task.get()


async def thread_code(put_task):
    loop = get_event_loop()
    task_queue = AsyncQueue()
    put_task.put((
        loop,
        lambda task=None: run_coroutine_threadsafe(task_queue.put(task), loop),
    ))
    while True:
        taskobj = await task_queue.get()
        if taskobj is None:
            break

        try:
            taskobj.result = None, (await taskobj.task)
        except SystemExit:
            raise
        except BaseException as ex:
            taskobj.result = ex, None
        taskobj()
