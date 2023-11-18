import pytest
import schemars

class Product:
    pass

product = Product()


def test_method_field_serialization():
    class ProductSchema(schemars.Schema):
        method = schemars.Method()

        def get_method(self, obj):
            return "cool"

    product_schema = ProductSchema()
    result = product_schema.serialize(product)
    assert result == {"method": "cool"}

def test_method_field_missing_method():
    class AnotherSchema(schemars.Schema):
        method = schemars.Method()

    another_schema = AnotherSchema()
    with pytest.raises(schemars.ValidationError):
        another_schema.serialize(product)

def test_method_name():
    class ProductSchema(schemars.Schema):
        method = schemars.Method(method_name="test_method")

        def test_method(self, obj):
            return "cool"

    product_schema = ProductSchema()
    result = product_schema.serialize(product)
    assert result == {"method": "cool"}