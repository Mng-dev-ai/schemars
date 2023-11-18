import pytest
import schemars
from datetime import date, datetime

def test_serialize_date():
    date_field = schemars.Date(strict=True)
    test_date = date(2021, 4, 5)
    result = date_field.serialize(test_date)
    assert result == test_date.isoformat()

def test_serialize_datetime_non_strict():
    date_field = schemars.Date()
    test_datetime = datetime(2021, 4, 5, 12, 30)
    result = date_field.serialize(test_datetime)
    assert result == test_datetime.date().isoformat()

@pytest.mark.parametrize("input_value", ["2020-01-01", 1577836800])
def test_serialize_string_long_float_non_strict(input_value):
    date_field = schemars.Date()
    result = date_field.serialize(input_value)
    assert result == "2020-01-01"

def test_serialize_with_custom_format():
    date_field = schemars.Date(format="%Y/%m/%d", strict=True)
    test_date = date(2021, 4, 5)
    result = date_field.serialize(test_date)
    assert result == "2021/04/05"

@pytest.mark.parametrize("invalid_input", ["invalid date", 123456, True])
def test_serialize_invalid_data(invalid_input):
    date_field = schemars.Date(strict=True)
    with pytest.raises(schemars.ValidationError):
        date_field.serialize(invalid_input)
