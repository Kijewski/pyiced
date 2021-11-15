Usage Examples
==============

Quick Example
-------------

A simple counter with two buttons to increment and decrement a value:

.. image:: _static/images/examples/Counter.png
    :width: 471
    :height: 361
    :align: center
    :alt:

.. literalinclude :: ../examples/counter.py
   :language: python

Custom Styles
-------------

.. image:: _static/images/examples/Styling.png
    :width: 622
    :height: 444
    :align: center
    :alt: 

.. literalinclude :: ../examples/styling.py
   :language: python

Asychronous Messages
--------------------

:meth:`~pyiced.IcedApp.new` and :meth:`~pyiced.IcedApp.update` can either return a :class:`~pyiced.Message`
(or a sequence of messages in the latter case), or
`a coroutine / coroutines <https://docs.python.org/3/library/asyncio-task.html>`_
to asynchronously generate a messages.

.. image:: _static/images/examples/AsyncMessages.png
    :width: 688
    :height: 405
    :align: center
    :alt: 

.. literalinclude :: ../examples/async_messages.py
   :language: python

AsyncGenerator Generating Messages
----------------------------------

An application can :meth:`subscribe <pyiced.IcedApp.subscriptions>` to :class:`~typing.AsyncGenerator`\ s
to receive :class:`~pyiced.Message`\ s about asynchronously generated information, e.g. a pending web download.

.. image:: _static/images/examples/StreamExample.png
    :width: 688
    :height: 125
    :align: center
    :alt: 

.. literalinclude :: ../examples/stream.py
   :language: python