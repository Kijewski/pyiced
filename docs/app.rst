Programming an IcedApp
======================

Overview
--------

.. autosummary::
    ~pyiced.IcedApp
    ~pyiced.Element
    ~pyiced.Message
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

   :data:`~typing.Union`\ [\ :class:`~typing.Awaitable`\ [\ :data:`~typing.Optional`\ [\ :class:`object`]] | :class:`object`]

.. py:data:: pyiced.Commands

   :class:`~typing.Iterable`\ [\ :data:`~typing.Optional`\ [\ :class:`~pyiced.Command`]]
