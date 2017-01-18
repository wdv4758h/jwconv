========================================
Japanese Writing System Converter
========================================


.. image:: https://cloud.githubusercontent.com/assets/2716047/22053139/b94afcf0-dd89-11e6-8662-73bc7054a3ad.png


.. contents:: Table of Contents


Components
========================================

There are several crates in this repo.

+--------+--------------------------------------------------------------------------------+
| Folder | Contents                                                                       |
+========+================================================================================+
| core   | all conversion functions                                                       |
+--------+--------------------------------------------------------------------------------+
| cli    | simple CLI wrapper of ``core``                                                 |
+--------+--------------------------------------------------------------------------------+
| ffi    | static/dynamic library for using from different languages (including C header) |
+--------+--------------------------------------------------------------------------------+

* For using in other Rust projects, use ``core`` (``jwconv`` in crates.io).
* For using in CLI, use ``cli`` (``jwconv-cli`` in crates.io).
* For using in other languages, use ``ffi`` (``jwconv-ffi`` in crates.io).



Installation (CLI)
========================================

From Source
------------------------------

.. code-block:: sh

    $ git clone https://github.com/wdv4758h/jwconv
    $ cargo install --path jwconv/cli



Usage (CLI)
========================================

.. code-block:: sh

    $ jwconv hana
    はな
    $ jwconv taiwan
    たいわん
    $ jwconv -m r2k hana
    ハナ
    $ jwconv -m r2k taiwan
    タイワン



FFI Examples
========================================

C
------------------------------

There is a C header in ``ffi/include/``,
and an example in ``ffi/examples/``.

To run the example:

.. code-block:: sh

    $ git clone git://github.com/wdv4758h/jwconv
    $ cd jwconv/ffi/examples/
    $ make all  # build & run


Python
------------------------------

There is a Python wrapper by using CFFI in ``ffi/bindings/python/cffi/``.

To try the wrapper:

.. code-block:: sh

    $ git clone git://github.com/wdv4758h/jwconv
    $ cd jwconv/ffi/
    $ cargo build --release
    $ python -i bindings/python/cffi/jwconv.py
    >>> romaji_to_katakana("taiwan")
    'タイワン'
    >>>



Special Thanks
========================================

* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

``jwconv`` is icensed under the Apache-2.0 License - see the ``LICENSE`` file for details
