#!/bin/bash
mkdir -p testdir
cd testdir
touch f1 f2 f3
mkdir d1 d2 d3
touch d1/f{4,5,6}
touch d2/f{7,8,9}
touch d3/f{10,11,12,13,14}
mkdir d3/d4
touch d3/d4/f15
