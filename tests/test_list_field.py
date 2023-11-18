import pytest
import schemars

def test_serialize_valid_list():
    list_field = schemars.List(strict=True)
    test_list = [1, 2, 3]
    result = list_field.serialize(test_list)
    assert result == test_list

def test_serialize_non_list_iterable_non_strict():
    list_field = schemars.List()
    test_tuple = (1, 2, 3)
    result = list_field.serialize(test_tuple)
    assert result == list(test_tuple)

def test_serialize_non_iterable_non_strict():
    list_field = schemars.List()
    with pytest.raises(TypeError):
        list_field.serialize(123)

def test_serialize_with_valid_child_serializer():
    list_field = schemars.List(child=schemars.Int())
    test_list = [1, 2, 3]
    result = list_field.serialize(test_list)
    assert result == test_list
    
def test_serialize_with_invalid_child_serializer():
    list_field = schemars.List(child=schemars.Str())
    test_list = [1, 2, 3]
    with pytest.raises(schemars.ValidationError):
        list_field.serialize(test_list)

@pytest.mark.parametrize("edge_case", [[], [1, "a", [2, 3]], [[1], [2, 3]]])
def test_serialize_edge_cases(edge_case):
    list_field = schemars.List(strict=True)
    result = list_field.serialize(edge_case)
    assert result == edge_case
