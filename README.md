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
