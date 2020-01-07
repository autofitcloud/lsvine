#!/bin/bash

# Test 1
root=testdir/test1
rm -rf $root
mkdir -p $root
touch $root/f{1,2,3}
mkdir $root/d{1,2,3}
touch $root/d1/f{4,5,6}
touch $root/d2/f{7,8,9}
touch $root/d3/f{10,11,12,13,14}
mkdir $root/d3/d4
touch $root/d3/d4/f15


# Test 2: empty dir
root=testdir/test2
rm -rf $root
mkdir -p testdir/test2

# Test 3: 30 dirs with 30 dirs each
root=testdir/test3
rm -rf $root
mkdir -p $root
# level 1
mkdir $root/d{0..30}

# level 2
array=($(seq 0 30))
for i in "${array[@]}"
do
  mkdir $root/d$i/d{0..30}
done

