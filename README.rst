========================================
Japanese Writing System Converter
========================================


.. image:: https://cloud.githubusercontent.com/assets/2716047/22053139/b94afcf0-dd89-11e6-8662-73bc7054a3ad.png


.. contents:: Table of Contents


Installation
========================================

From Source
------------------------------

.. code-block:: sh

    $ git clone https://github.com/wdv4758h/jwconv
    $ cargo install --path jwconv/cli



Usage
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



Special Thanks
========================================

* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for arguments parsing
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

``jwconv`` is icensed under the Apache-2.0 License - see the ``LICENSE`` file for details
