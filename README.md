# mutuple

Replace items in Python's "immutable" tuples.

## Why?

yolo

## How?

We use Rust's PyO3 crate to manipulate tuples directly using the Stable CPython API.

## DO NOT USE MUTUPLE IN PRODUCTION!

This project is just for fun. Do not use mutuple in production!

## Usage

```python
import mutuple


def test_mutable_tuple():
    """mutuple tuple demo"""
    value = ('read-only', 'tuple')
    mutuple.setitem(value, 0, 'mutable')

    assert value == ('mutable', 'tuple')
```

## Gotchas

Modifying tuples is a wild thing to do in Python.

While there are (questionably?) safe ways to use this package, there many things
that can go wrong when miused. This is not an exhaustive list of gotchas.

* Editing a tuple changes its `hash(...)` value. This means that bad things will
happen if you edit a tuple after adding it to an associative container
(i.e. `set` objects and `dict` keys).

* Python `set` containers expect that the `hash(...)` of the objects does not change once
the object is added to a `set`. Editing a tuple effectively changes its `hash()` value,
which "breaks" python and creates a `set` with a `tuple` in it that cannot be removed!

* Editing a tuple after using it as a key in a `dict` behaves the same way ~ the tuple's
entry cannot be removed from the `dict`. This is also due to the fact that the tuple's
`hash(...)` value changed after adding it to the container.

## Maintenance Status

`mutuple` is feature complete and stable. New features will not be added outside of
corrections to the implementation, packaging, documentation and test suite.

## Links

* [mutuple source code repository](https://github.com/davvid/mutuple)

* [mutuple pypi releases](https://pypi.org/project/mutuple/)

* [mutuple at crates.io](https://crates.io/crates/mutuple)
