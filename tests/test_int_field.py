import pytest
import schemars

def test_serialize_valid_integer():
    int_field = schemars.Int(strict=True)
    result = int_field.serialize(42)
    assert result == 42

def test_serialize_non_integer_strict():
    int_field = schemars.Int(strict=True)
    with pytest.raises(schemars.ValidationError):
        int_field.serialize("42")

@pytest.mark.parametrize("input_bool, expected", [(True, 1), (False, 0)])
def test_serialize_boolean_non_strict(input_bool, expected):
    int_field = schemars.Int()
    result = int_field.serialize(input_bool)
    assert result == expected

def test_serialize_string_non_strict():
    int_field = schemars.Int()
    result = int_field.serialize("42")
    assert result == 42

def test_serialize_float_non_strict():
    int_field = schemars.Int()
    result = int_field.serialize(42.0)
    assert result == 42

@pytest.mark.parametrize("invalid_input", ["not a number", 42.5])
def test_serialize_invalid_string_float_non_strict(invalid_input):
    int_field = schemars.Int()
    with pytest.raises(schemars.ValidationError):
        int_field.serialize(invalid_input)

@pytest.mark.parametrize("edge_case", [2147483647, -2147483648, 0])
def test_serialize_edge_cases(edge_case):
    int_field = schemars.Int(strict=True)
    result = int_field.serialize(edge_case)
    assert result == edge_case
