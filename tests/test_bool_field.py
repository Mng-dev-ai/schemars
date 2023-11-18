import pytest
import schemars

@pytest.mark.parametrize("input_bool", [True, False])
def test_serialize_valid_boolean(input_bool):
    bool_field = schemars.Bool(strict=True)
    result = bool_field.serialize(input_bool)
    assert result == input_bool

@pytest.mark.parametrize("input_str, expected", [("true", True), ("false", False)])
def test_serialize_string_non_strict(input_str, expected):
    bool_field = schemars.Bool()
    result = bool_field.serialize(input_str)
    assert result == expected

@pytest.mark.parametrize("input_value, expected", [(1, True), (0, False)])
def test_serialize_int_float_non_strict(input_value, expected):
    bool_field = schemars.Bool()
    result = bool_field.serialize(input_value)
    assert result == expected

@pytest.mark.parametrize("invalid_input", ["invalid", 2, 0.5])
def test_serialize_invalid_data_non_strict(invalid_input):
    bool_field = schemars.Bool()
    with pytest.raises(schemars.ValidationError):
        bool_field.serialize(invalid_input)

@pytest.mark.parametrize("edge_case", ["yes", "no", "1", "0", "t", "f"])
def test_serialize_edge_cases(edge_case):
    bool_field = schemars.Bool()
    result = bool_field.serialize(edge_case)
    assert result in [True, False]
    