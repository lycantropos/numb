from hypothesis import given

from rithm.integer import Int

from . import strategies


@given(strategies.ints)
def test_idempotence(int_: Int) -> None:
    assert +int_ == int_
