PyIced
======

Python bindings for |iced|_.

Iced is a cross-platform GUI library focused on simplicity and type-safety. Inspired by Elm.

Installation
------------

.. code:: bash

    $ pip install pyiced

To install from source you need to have a recent version of |rust|_ installed in your $PATH.

|rustup|_ is probably the most easy to use option to install and update |rust|_ on your system.

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

Table of Contents
-----------------

.. toctree::
    :maxdepth: 3

    examples.rst
    app.rst
    elements.rst
    state_objects.rst
    values.rst
    styles.rst
    subscriptions.rst

-------------------------------------------------------------------------------

:ref:`Glossary / Index <genindex>`


.. _rustup: https://rustup.rs/
.. |rustup| replace:: **Rustup**

.. _rust: https://www.rust-lang.org/
.. |rust| replace:: **Rust**

.. _iced: https://github.com/iced-rs/iced
.. |iced| replace:: **Iced**
