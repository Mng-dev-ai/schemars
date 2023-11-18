import pytest
import schemars

def test_serialize_valid_float():
    float_field = schemars.Float(strict=True)
    result = float_field.serialize(42.0)
    assert result == 42.0

def test_serialize_non_float_strict():
    float_field = schemars.Float(strict=True)
    with pytest.raises(schemars.ValidationError):
        float_field.serialize("42.0")

@pytest.mark.parametrize("input_bool, expected", [(True, 1.0), (False, 0.0)])
def test_serialize_boolean_non_strict(input_bool, expected):
    float_field = schemars.Float()
    result = float_field.serialize(input_bool)
    assert result == expected

def test_serialize_string_non_strict():
    float_field = schemars.Float()
    result = float_field.serialize("42.0")
    assert result == 42.0

def test_serialize_integer_non_strict():
    float_field = schemars.Float()
    result = float_field.serialize(42)
    assert result == 42.0

@pytest.mark.parametrize("invalid_input", ["not a number", "42.5a"])
def test_serialize_invalid_string_non_strict(invalid_input):
    float_field = schemars.Float()
    with pytest.raises(schemars.ValidationError):
        float_field.serialize(invalid_input)

@pytest.mark.parametrize("edge_case", [1.79e308, -1.79e308, 0.0])
def test_serialize_edge_cases(edge_case):
    float_field = schemars.Float(strict=True)
    result = float_field.serialize(edge_case)
    assert result == edge_case
