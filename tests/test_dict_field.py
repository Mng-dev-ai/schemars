import pytest
import schemars
from typing import Mapping


def test_serialize_valid_dict():
    dict_field = schemars.Dict(strict=True)
    test_dict = {"key1": 1, "key2": 2}
    result = dict_field.serialize(test_dict)
    assert result == test_dict


def test_serialize_mapping_non_strict():
    class CustomDict(Mapping):
        def __init__(self):
            self._dict = {"key1": 1, "key2": 2}

        def __getitem__(self, key):
            return self._dict[key]

        def __iter__(self):
            return iter(self._dict)

        def __len__(self):
            return len(self._dict)

    dict_field = schemars.Dict()
    test_obj = CustomDict()
    result = dict_field.serialize(test_obj)
    assert result == {"key1": 1, "key2": 2}


def test_serialize_non_dict_non_mapping_non_strict():
    dict_field = schemars.Dict()
    with pytest.raises(schemars.ValidationError):
        dict_field.serialize(123)


def test_serialize_with_child():
    dict_field = schemars.Dict(child=schemars.Int())
    test_dict = {"key1": 1, "key2": 2}
    result = dict_field.serialize(test_dict)
    assert result == test_dict


def test_serialize_with_invalid_child():
    dict_field = schemars.Dict(child=schemars.Str())
    test_dict = {"key1": 1, "key2": 2}
    with pytest.raises(schemars.ValidationError):
        dict_field.serialize(test_dict)


@pytest.mark.parametrize(
    "edge_case", [{}, {"key": "value", "nested": {"nested_key": 1}}, {"empty_dict": {}}]
)
def test_serialize_edge_cases(edge_case):
    dict_field = schemars.Dict(strict=True)
    result = dict_field.serialize(edge_case)
    assert result == edge_case
