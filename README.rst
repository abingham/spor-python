===========
spor-python
===========

Python bindings for `spor <https://github.com/abingham/spor>`_.

Building
========

You can do basic building with cargo::

    cargo build

Wheels, etc.
------------

Alternatively, you can build with `maturin <https://github.com/PyO3/maturin>`_. This not only does a normal cargo build,
but it can also things like build wheels.

To use it, you first need to install it with `pip`::

    pip install maturin

Then use maturin to build::

    maturin build

Note that you may need to specify a particular Python interpreter::

    maturin build --i python3.8

Build issues?
-------------

We use `rust-cpython <https://github.com/dgrunwald/rust-cpython>`_ to generate the Python bindings, so read their docs
if you have any trouble building (particularly on macos).
