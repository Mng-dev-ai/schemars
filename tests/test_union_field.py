import pytest
import schemars

def test_serialize_success_union_field():
    union_field = schemars.Union(fields=[schemars.Str(), schemars.Int()])
    test_str = "test"
    result = union_field.serialize(test_str)
    assert result == test_str

    test_int = 123
    result = union_field.serialize(test_int)
    assert result == test_int

def test_serialize_failure_union_field():
    union_field = schemars.Union(fields=[schemars.Str(), schemars.Int()])
    with pytest.raises(schemars.ValidationError):
        union_field.serialize(1.23)

@pytest.mark.parametrize("edge_case", ["123", 123])
def test_serialize_edge_cases_union_field(edge_case):
    union_field = schemars.Union(fields=[schemars.Str(), schemars.Int()])
    result = union_field.serialize(edge_case)
    assert result == edge_case
