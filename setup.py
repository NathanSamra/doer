from setuptools import setup, find_packages

from doer import metadata

setup(
    name=metadata.name,
    author=metadata.author,
    version=metadata.__version__,
    packages=find_packages(),
    entry_points={'console_scripts': ['doer=doer.console:enter']},
)
