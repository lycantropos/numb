from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin

from . import strategies


@given(strategies.ints_with_builtins, strategies.ints)
def test_connection_with__sub__(
    subtrahend_with_builtin: IntWithBuiltin, minuend: Int
) -> None:
    subtrahend, subtrahend_builtin = subtrahend_with_builtin

    assert subtrahend_builtin - minuend == subtrahend - minuend
