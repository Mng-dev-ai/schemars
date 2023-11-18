import pytest
import schemars
from datetime import date, datetime

def test_serialize_datetime():
    datetime_field = schemars.DateTime(strict=True)
    test_datetime = datetime(2021, 4, 5, 15, 30)
    result = datetime_field.serialize(test_datetime)
    assert result == test_datetime.isoformat()

def test_serialize_date_non_strict():
    datetime_field = schemars.DateTime()
    test_date = date(2021, 4, 5)
    result = datetime_field.serialize(test_date)
    expected_datetime = datetime.combine(test_date, datetime.min.time())
    assert result == expected_datetime.isoformat()

@pytest.mark.parametrize("input_value", ["2021-04-05T16:50:00", 1617641400, 1617641400.0])
def test_serialize_string_long_float_non_strict(input_value):
    datetime_field = schemars.DateTime()
    result = datetime_field.serialize(input_value)
    expected_datetime = datetime(2021, 4, 5, 16, 50)
    assert result == expected_datetime.isoformat()

def test_serialize_with_custom_format():
    datetime_field = schemars.DateTime(format="%Y/%m/%d %H:%M", strict=True)
    test_datetime = datetime(2021, 4, 5, 15, 30)
    result = datetime_field.serialize(test_datetime)
    assert result == test_datetime.strftime("%Y/%m/%d %H:%M")

@pytest.mark.parametrize("invalid_input", ["invalid datetime", 123456, True])
def test_serialize_invalid_data(invalid_input):
    datetime_field = schemars.DateTime(strict=True)
    with pytest.raises(schemars.ValidationError):
        datetime_field.serialize(invalid_input)
