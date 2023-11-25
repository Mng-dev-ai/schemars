import schemars

def test_serialize_valid_any():
    any_field = schemars.Any()
    valid_value = "Hello World" 
    result = any_field.serialize(valid_value)
    assert result == valid_value