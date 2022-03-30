from setuptools import setup, find_packages

from doer import metadata

setup(name=metadata.name,
      author=metadata.author,
      version=metadata.__version__,
      install_requires=['appdirs~=1.4.4', 'django~=4.0.3'],
      packages=find_packages(),
      entry_points={
          'console_scripts': ['doer=doer.console:enter']
      })
