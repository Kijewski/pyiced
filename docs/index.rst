PyIced
======

|Python|
|OS|
|License|

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

.. image:: ../examples/counter.png
    :align: center
    :alt: 

.. literalinclude :: ../examples/counter.py
   :language: python

Bigger Example
--------------

.. image:: ../examples/chess.png
    :align: center
    :alt:

Please find the source code in the :ref:`examples/chess.py <examples:Two-player Online Chess>`.

Table of Contents
-----------------

.. toctree::
    :maxdepth: 3

    examples.rst
    app.rst
    elements.rst
    state_objects.rst
    values.rst
    colors.rst
    fonts.rst
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

.. |Python| image:: https://img.shields.io/badge/python-3.7%20|%203.8%20|%203.9%20|%203.10-84A6C7?logo=python&logoColor=efefef
.. |OS| image:: https://img.shields.io/badge/os-windows%20|%20linux-84A6C7?logo=windowsxp&logoColor=efefef
.. |License| image:: https://img.shields.io/badge/license-Apache--2.0-84A6C7?logo=apache&logoColor=efefef
