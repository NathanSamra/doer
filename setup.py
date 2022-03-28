from setuptools import setup, find_packages

from do import metadata

setup(name=metadata.name,
      author=metadata.author,
      version=metadata.__version__,
      install_requires=['appdirs~=1.4.4'],
      packages=find_packages(),
      entry_points={
          'console_scripts': ['do=do.console.entry_point:enter']
      })
