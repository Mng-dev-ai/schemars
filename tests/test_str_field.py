import pytest
import schemars

def test_serialize_valid_string():
    str_field = schemars.Str(strict=True)
    result = str_field.serialize("Hello World")
    assert result == "Hello World"

def test_serialize_non_string_strict():
    str_field = schemars.Str(strict=True)
    with pytest.raises(schemars.ValidationError):
        str_field.serialize(123)

def test_serialize_bytes_non_strict():
    str_field = schemars.Str()
    result = str_field.serialize(b"Hello World")
    assert result == "Hello World"

def test_serialize_non_utf8_bytes_non_strict():
    str_field = schemars.Str()
    result = str_field.serialize(b'\xff\xfe\xfd')
    assert result == '\ufffd\ufffd\ufffd'

def test_serialize_non_string_non_bytes_non_strict():
    str_field = schemars.Str()
    with pytest.raises(schemars.ValidationError):
        str_field.serialize([1, 2, 3])

@pytest.mark.parametrize("input_str", ["", "😊", "a" * 1000])
def test_serialize_edge_cases(input_str):
    str_field = schemars.Str(strict=True)
    result = str_field.serialize(input_str)
    assert result == input_str
