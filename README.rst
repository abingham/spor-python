===========
spor-python
===========

Python bindings for `spor <https://github.com/abingham/spor>`_.

Building
========

You need to install `maturin <https://github.com/PyO3/maturin>`_ to build the extension::

    pip install maturin

Then use maturin to build::

    maturin build

Note that you may need to specify a particular Python interpreter::

    maturin build --i python3.8