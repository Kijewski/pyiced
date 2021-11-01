Programming an IcedApp
======================

TODO

Overview
--------

.. autosummary::
    ~pyiced.IcedApp
    ~pyiced.Element
    ~pyiced.Message
    ~pyiced.Subscription
    ~pyiced.Settings
    ~pyiced.WindowSettings

Details
-------

.. autoclass:: pyiced.IcedApp
   :members:
   :undoc-members:

.. autoclass:: pyiced.Element
   :members:
   :undoc-members:

.. autoclass:: pyiced.Message
   :members:
   :undoc-members:

.. autoclass:: pyiced.Settings
   :members:
   :undoc-members:

.. autoclass:: pyiced.WindowSettings
   :members:
   :undoc-members:

Type aliases
------------

.. py:data:: pyiced.Command

   :class:`~pyiced.Message` | :class:`~typing.Awaitable`\ [:class:`~pyiced.Message` | None]

.. py:data:: pyiced.Commands

   :class:`~typing.Iterable`\ [\ :class:`~pyiced.Command` | None]
