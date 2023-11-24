import pytest
import schemars
import uuid

def test_serialize_valid_uuid():
    uuid_field = schemars.Uuid(strict=True)
    valid_uuid = uuid.uuid4()
    result = uuid_field.serialize(valid_uuid)
    assert result == str(valid_uuid)

def test_serialize_non_uuid_strict():
    uuid_field = schemars.Uuid(strict=True)
    with pytest.raises(schemars.ValidationError):
        uuid_field.serialize(str(uuid.uuid4()))

def test_serialize_uuid_string_non_strict():
    uuid_field = schemars.Uuid()
    valid_uuid_str = str(uuid.uuid4())
    result = uuid_field.serialize(valid_uuid_str)
    assert result == valid_uuid_str

def test_serialize_uuid_bytes_non_strict():
    uuid_field = schemars.Uuid()
    valid_uuid_bytes = uuid.uuid4().bytes
    result = uuid_field.serialize(valid_uuid_bytes)
    assert result == str(uuid.UUID(bytes=valid_uuid_bytes))

@pytest.mark.parametrize("invalid_input", ["not a uuid", 123, True, b'invalid'])
def test_serialize_invalid_input_non_strict(invalid_input):
    uuid_field = schemars.Uuid()
    with pytest.raises(schemars.ValidationError):
        uuid_field.serialize(invalid_input)

def test_serialize_edge_cases():
    uuid_field = schemars.Uuid(strict=True)
    edge_cases = [uuid.UUID(int=0), uuid.UUID(int=(2**128)-1)]
    for edge_case in edge_cases:
        result = uuid_field.serialize(edge_case)
        assert result == str(edge_case)
