# 2019-10-24 Not sure why find_packages was commented out .. bringing it back
from setuptools import setup, find_packages

# copied from https://github.com/awslabs/git-remote-codecommit/blob/master/setup.py
import os
def read(fname):
  return open(os.path.join(os.path.dirname(__file__), fname)).read()
  

# follow https://github.com/awslabs/git-remote-codecommit/blob/master/setup.py
# and https://packaging.python.org/tutorials/packaging-projects/
setup(
    name='lsvine',
    version='0.1.3',
    author="Shadi Akiki, AutofitCloud",
    author_email="shadi@autofitcloud.com",
    url='https://github.com/autofitcloud/lsvine',
    description="List contents of directories in a vine-like format.",

    long_description = read('README.md'),
    long_description_content_type="text/markdown",
    
    packages=find_packages(),
    include_package_data=True,
    install_requires=[
        'click==7.0',
        'pandas==0.25.3',
        'tabulate==0.8.6',
        'termcolor==1.1.0',
    ],
    entry_points='''
        [console_scripts]
        lsvine=lsvine.cli:cli_core
    ''',
)
