State Objects
=============

To keep the state of an :class:`~pyiced.Element` across multiple invocations of
:meth:`~pyiced.IcedApp.view`, e.g. the cursor position in a 
:func:`~pyiced.text_input`, you have to supply a state object.

.. warning::
   If the same state object is used for multiple elements in the same
   :meth:`~pyiced.IcedApp.view` call, only the first element get displayed.
   All and further elements with the same state become :func:`~pyiced.no_element`.

Overview
--------

.. autosummary::
   ~pyiced.ButtonState
   ~pyiced.PickListState
   ~pyiced.ScrollableState
   ~pyiced.SliderState
   ~pyiced.TextInputState

Details
-------

.. autoclass:: pyiced.ButtonState
   :members:
   :undoc-members:

.. autoclass:: pyiced.PickListState
   :members:
   :undoc-members:

.. autoclass:: pyiced.ScrollableState
   :members:
   :undoc-members:

.. autoclass:: pyiced.SliderState
   :members:
   :undoc-members:

.. autoclass:: pyiced.TextInputState
   :members:
   :undoc-members:
