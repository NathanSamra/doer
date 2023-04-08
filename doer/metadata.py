from importlib import metadata

from semantic_version import Version

name = "doer"
author = "Nathan Samra"

__version__ = Version(metadata.version(__package__))
version = __version__
