## Dev notes

Create the `testdir` folder

```
bash mk_testdir.sh
```

Install editably locally

```
pew new lsvine
pip3 install -e .
```

Packaging to pypi

```
update version in setup.py
update version in changelog
commit with 'version bump 0.1.0'
git tag 0.1.0
git push origin 0.1.0
git push github 0.1.0
```

publish to pypi

```
pip3 install twine
rm build/* -rf
rm dist/* -rf
python3 setup.py sdist bdist_wheel
twine upload dist/*
```

Got pypi badge from
https://badge.fury.io/for/py/git-remote-aws



