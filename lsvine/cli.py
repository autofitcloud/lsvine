"""
Installation:
  pew new os2d
  pip3 install tabulate pandas termcolor
  python3 ls_trello.py testdir

Example:
Check README
"""
from tabulate import tabulate
import sys
import os
from os import listdir
from os.path import isfile, join
import pandas as pd
import click
import numpy as np


def doit(mypath):
  # files list
  onlyfiles = listdir(mypath) # if isfile(join(mypath, f))]
  
  # empty folder
  if len(onlyfiles)==0:
    return
 
  # dataframe
  df = pd.DataFrame({'fn_full': onlyfiles})
  
  # add if is file or dir
  df['isfile'] = df.fn_full.apply(lambda f: isfile(join(mypath, f)))
  
  # add files in dirs
  df['subfiles'] = df.apply(lambda row: None if row.isfile else listdir(join(mypath, row.fn_full)), axis=1)
  
  # print
  #print(df)
  
  # convert dataframe back into list of lists
  df_files = df[df.isfile]
  df_dirs  = df[~df.isfile]
  
  # if only list of files
  if df_dirs.shape[0]==0:
    lol_dirs = {}
  else:
    lol_dirs = df_dirs.apply(lambda row: {row.fn_full: row.subfiles}, axis=1).tolist()
  
  # flatten list of dict each with 1 key to dict
  lol_dirs = {list(x.keys())[0]: x[list(x.keys())[0]] for x in lol_dirs}
  
  # append direct files
  if df_files.shape[0] > 0:
    lol_dirs['0_root'] = df_files.fn_full.tolist()
  
  # skip folders that begin with .
  lol_dirs = {k: v for i, (k,v) in enumerate(lol_dirs.items()) if not k.startswith('.') and not k.startswith('_')}
  lol_dirs = {k: [fn for fn in v if not fn.startswith('.')] for i, (k,v) in enumerate(lol_dirs.items())}
  
  # truncate number of folders
  # lol_dirs = {k: v for i, (k,v) in enumerate(lol_dirs.items()) if i<10 or k=='0_root'}
  
  # sort alphabetically
  lol_dirs = {k: sorted(v) for i, (k,v) in enumerate(lol_dirs.items())}

  # prepare for max filename length to display
  max_fnl = 50

  # calculate the maximum number of columns that can be displayed
  lengths_cumsum = np.cumsum([
                          np.max([min(len(fn), max_fnl) for fn in v + [k]]) + 3 # assume 3 spaces between columns. Take column name into account
                          for i, (k, v) in enumerate(lol_dirs.items())
                       ])
  lengths_available = os.get_terminal_size().columns
  rownumbers = [x//lengths_available for x in lengths_cumsum] # returns which rownumber each item in the dict should use
 
  # color dirs and truncate filenames
  # TODO instead of truncating, insert a new-line character. tabulate can display multi-line cells
  # https://github.com/astanin/python-tabulate#multiline-cells
  from termcolor import colored
  lol_dirs = {k: [  colored( fn[:max_fnl],
                             "grey" if isfile(join(mypath,'.' if k=='0_root' else k,fn)) else "blue",
                             attrs=[] if isfile(join(mypath,'.' if k=='0_root' else k,fn)) else ['bold']
                           )
                    for fn in v]
                 for i, (k,v) in enumerate(lol_dirs.items())
             }
  #lol_dirs = {k: [print(join(mypath,k,fn), isfile(join(mypath,k,fn))) for fn in v] for i, (k,v) in enumerate(lol_dirs.items())}
  
  # change lol_dirs to an OrderedDict with sorted keys
  import collections
  lol_dirs = collections.OrderedDict(sorted(lol_dirs.items()))
  
  #
  # print(lol_dirs)
  # print(tabulate(lol_dirs, headers = lol_dirs.keys()))

 
  # instead of printing a singe tabulate call of a list of lists, break into muptiple rows in case of lots of folders
  #max_cols = 5
  lol_dirs2 = {}
  for i, ((k,v), j) in enumerate(zip(lol_dirs.items(), rownumbers)):
    #j = int(i//max_cols)
    if j not in lol_dirs2.keys(): lol_dirs2[j] = {}
    lol_dirs2[j][k] = v
  
  # print each row
  for i in sorted(list(lol_dirs2.keys())):
    print(tabulate(lol_dirs2[i], headers = lol_dirs2[i].keys()))
    print("")
  
  # back to dataframe
  #df_print = pd.DataFrame(lol_dirs)
    
  # print
  #print(df_print)


# https://click.palletsprojects.com/en/7.x/
@click.command()
@click.argument("path", default='.', type=click.Path(exists=True))
@click.version_option()
def cli_core(path):
  doit(path)


if __name__ == '__main__':
    cli_core()
