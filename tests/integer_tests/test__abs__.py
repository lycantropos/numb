from hypothesis import given

from tests.utils import IntWithBuiltin, is_equivalent_to_builtin_int

from . import strategies


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    assert is_equivalent_to_builtin_int(abs(int_), abs(builtin_int))
