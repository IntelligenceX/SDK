#!/usr/bin/python3
from setuptools import setup, find_packages
from distutils.core import setup
from os import path
here = path.abspath(path.dirname(__file__))
with open(path.join(here, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()

setup(name='intelx',
      version='0.5',
      description='IntelX is a Python command-line utility and API wrapper for intelx.io, made to perform any kind of open-source intelligence.',
      long_description=long_description,
      long_description_content_type='text/markdown',
      author='Kleissner Investments s.r.o./ Dominik Penner',
      author_email='info@intelx.io',
      keywords='intelx intelligence x intelx.io _IntelligenceX IntelligenceX _intelx',
      url='https://github.com/IntelligenceX/SDK/tree/master/Python',
      packages=find_packages(),
      scripts=['cli/intelx.py'],
      install_requires=['requests', 'pygments', 'termcolor', 'tabulate']
      )
