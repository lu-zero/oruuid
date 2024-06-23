# oruuid
> An example wrapper around [uuid](https://crates.io/crates/uuid).

## Status
It is just part of a presentation/tutorial on [PyO3](https://crates.io/crates/pyO3) and [maturin](https://crates.io/crates/maturin),
if you find it useful you are welcome to fork and develop it further.

## Building
As usual you may use a `venv`:
``` sh
$ python -m venv .venv
$ .venv/bin/activate
```
You need [maturin](crates.io/crates/maturin), you can install it using [pypi](https://pypi.org/project/maturin/)
``` sh
$ pip install maturin
```
 or [cargo](https://crates.io/crates/maturin):
``` sh
$ cargo install maturin
```

And then use the `develop` command:
``` sh
$ maturin develop
```

## Testing
You may use [pytest](https://pypi.org/project/pytest/):
``` sh
$ maturin develop
$ pip install -r rest-requirements.txt
$ pytest
```
