from contextlib import contextmanager
from asyncio import Event, get_event_loop, run
from enum import Enum
from queue import Queue
from threading import Thread

from . import pyiced as _pyiced


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

__all__ += ['run_iced']

__author__ = _pyiced.__author__
__version__ = _pyiced.__version__


async def thread_code(put_task):
    def done(task):
        loop.call_soon_threadsafe(done_event.set)

    done_event = Event()
    loop = get_event_loop()
    put_task.put((loop, done))
    await done_event.wait()


@contextmanager
def in_async_loop():
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


def run_iced(obj):
    new = getattr(obj, 'new', None)
    title = getattr(obj, 'title', None)
    update = getattr(obj, 'update', None)
    should_exit = getattr(obj, 'should_exit', None)
    scale_factor = getattr(obj, 'scale_factor', None)
    fullscreen = getattr(obj, 'fullscreen', None)
    view = getattr(obj, 'view', None)
    settings = getattr(obj, 'settings', None)
    with in_async_loop() as loop:
        return _pyiced.run_iced(
            loop, new, title, update, should_exit, scale_factor, fullscreen, view, settings,
        )
