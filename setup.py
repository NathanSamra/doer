from setuptools import setup, find_packages

from do import metadata

setup(name=metadata.name,
      author=metadata.author,
      version=metadata.__version__,
      packages=find_packages(),
      entry_points={
          'console_scripts': ['do=do.console.entry_point:enter']
      })
