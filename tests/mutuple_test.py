import mutuple

import pytest


def test_setitem():
    """Ensure that we can replace items in a tuple"""
    obj = (None, None, None)
    mutuple.setitem(obj, 0, 42)
    mutuple.setitem(obj, 2, 420)
    assert obj[0] == 42
    assert obj[1] is None
    assert obj[2] == 420



def test_setitem_non_tuple():
    """Ensure that mutuple is only used with tuples"""
    obj = [None]
    with pytest.raises(TypeError):
        mutuple.setitem(obj, 0, 42)


def test_setitem_out_of_range():
    """Ensure that an error is raised when setting an out-of-range value"""
    obj = (None, None)
    with pytest.raises(IndexError):
        mutuple.setitem(obj, 2, 42)


def test_setitem_custom_object():
    """Ensure mutuple works with custom objects"""
    class Custom:
        def __init__(self, value):
            self.value = value

    custom = Custom(42)
    obj = (None, None)
    mutuple.setitem(obj, 0, custom)

    assert obj[0] is custom
    assert obj[0].value == custom.value
