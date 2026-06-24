"""Ancora Python SDK.

Provides a :class:`Runtime` handle and :func:`version` for the Ancora
agent runtime, backed by a native Rust extension via PyO3.

Example::

    import ancora
    with ancora.Runtime() as rt:
        print(rt)
"""

from ancora._ancora import AncorError, Runtime, version

__all__ = ["AncorError", "Runtime", "version"]
