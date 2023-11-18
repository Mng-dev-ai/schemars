import pytest
import schemars
from decimal import Decimal as PyDecimal


def test_serialize_decimal_compatible():
    decimal_field = schemars.Decimal(strict=True)
    test_decimal = PyDecimal("10.5")
    result = decimal_field.serialize(test_decimal)
    assert result == str(test_decimal)


def test_serialize_float_non_strict():
    decimal_field = schemars.Decimal()
    test_float = 10.5
    result = decimal_field.serialize(test_float)
    assert result == str(PyDecimal(test_float))


def test_serialize_string_non_strict():
    decimal_field = schemars.Decimal()
    test_str = "10.5"
    result = decimal_field.serialize(test_str)
    assert result == str(PyDecimal(test_str))


def test_serialize_integer_non_strict():
    decimal_field = schemars.Decimal()
    test_int = 10
    result = decimal_field.serialize(test_int)
    assert result == str(PyDecimal(test_int))


@pytest.mark.parametrize("invalid_input", [True, "not a number", 1 + 1j])
def test_serialize_invalid_data_non_strict(invalid_input):
    decimal_field = schemars.Decimal()
    with pytest.raises(schemars.ValidationError):
        decimal_field.serialize(invalid_input)
