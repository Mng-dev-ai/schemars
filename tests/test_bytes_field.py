import pytest
import schemars

def test_serialize_valid_bytes():
    bytes_field = schemars.Bytes(strict=True)
    test_bytes = b"Hello World"
    result = bytes_field.serialize(test_bytes)
    assert result == test_bytes

def test_serialize_string_non_strict():
    bytes_field = schemars.Bytes()
    test_str = "Hello World"
    result = bytes_field.serialize(test_str)
    assert result == test_str.encode()

@pytest.mark.parametrize("invalid_input", [123, 42.0, True, [1, 2, 3]])
def test_serialize_invalid_data_non_strict(invalid_input):
    bytes_field = schemars.Bytes()
    with pytest.raises(schemars.ValidationError):
        bytes_field.serialize(invalid_input)

@pytest.mark.parametrize("edge_case", ["", "😊", "a" * 1000])
def test_serialize_edge_cases(edge_case):
    bytes_field = schemars.Bytes()
    result = bytes_field.serialize(edge_case)
    assert result == edge_case.encode()
