# mutuple

Replace items in Python's "immutable" tuples.

## Why?

yolo

## How?

We use Rust's PyO3 crate to manipulate tuples directly using the Stable CPython API.
Editing tuples can lead to issues but works in practice for limited use cases.

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

Modifying tuples is (obviously) a wild thing to do in Python.

While there are safe ways to use this package, there are also many things
that can go wrong. This is not an exhaustive list of gotchas; it's just a sampler.

* Editing a tuple changes its `hash(...)` value.

* Editing a tuple after adding it to a `set` is questionable at best.
Python `set` containers expect that the `hash(...)` of the objects does not change once
the object is added, but editing a tuple effectively changes its `hash()` value.

* Editing a tuple after using it as a key in a `dict` can lead to situations where
the tuple's entry cannot be removed from the container. This is also due to the fact
that the tuple's `hash(...)` value is changed.

## Links

* [mutuple source code repository](https://github.com/davvid/mutuple)

* [mutuple pypi releases](https://pypi.org/project/mutuple/)

* [mutuple at crates.io](https://crates.io/crates/mutuple)
