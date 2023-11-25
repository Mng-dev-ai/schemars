import schemars


def test_serialize_many():
    class Product:
        def __init__(self, name):
            self.name = name

    class ProductSchema(schemars.Schema):
        name = schemars.Str()

    schema = ProductSchema()
    products = [
        Product("Product 1"),
        Product("Product 2"),
    ]
    result = schema.serialize(products, many=True)
    assert result == [{"name": "Product 1"}, {"name": "Product 2"}]


def test_serialize_with_default():
    class Product:
        def __init__(self, name=None):
            self.name = name

    class ProductSchema(schemars.Schema):
        name = schemars.Str(default="Product 1")

    schema = ProductSchema()
    product = Product()
    result = schema.serialize(product)
    assert result == {"name": "Product 1"}


def test_serialize_with_default_none():
    class Product:
        def __init__(self, name=None):
            self.name = name

    class ProductSchema(schemars.Schema):
        name = schemars.Str(default=None)

    schema = ProductSchema()
    product = Product()
    result = schema.serialize(product)
    assert result == {"name": None}


def test_serialize_with_write_only():
    class Product:
        def __init__(self, name):
            self.name = name

    class ProductSchema(schemars.Schema):
        name = schemars.Str(write_only=True)

    schema = ProductSchema()
    product = Product("Product 1")
    result = schema.serialize(product)
    assert result == {}


def test_serialize_with_source():
    class User:
        def __init__(self, name, age):
            self.name = name
            self.age = age

    class Product:
        def __init__(self, user):
            self.user = user

    class ProductSchema(schemars.Schema):
        name = schemars.Str(source="user.name")
        age = schemars.Int(source="user.age")

    schema = ProductSchema()
    user = User("John", 30)
    product = Product(user)
    result = schema.serialize(product)
    assert result == {"name": "John", "age": 30}


def test_serialize_with_call():
    def custom_func():
        return "test"

    def get_tags():
        return [
            Tag("tag1"),
            Tag("tag2"),
        ]

    class Product:
        @property
        def test(self):
            return custom_func

        @property
        def tags(self):
            return get_tags

    class Tag:
        def __init__(self, name):
            self.name = name

    class TagSchema(schemars.Schema):
        name = schemars.Str()

    class ProductSchema(schemars.Schema):
        test = schemars.Str(call=True)
        tags = TagSchema(many=True, call=True)

    schema = ProductSchema()
    product = Product()
    result = schema.serialize(product)
    assert result == {"test": "test", "tags": [{"name": "tag1"}, {"name": "tag2"}]}


def test_serialize_with_serialize_func():
    class ProductSchema(schemars.Schema):
        name = schemars.Str(serialize_func=lambda name: name.upper())

    class Product:
        def __init__(self, name):
            self.name = name

    schema = ProductSchema()
    product = Product("Product 1")
    result = schema.serialize(product)
    assert result == {"name": "PRODUCT 1"}


def test_serialize_with_nested():
    class User:
        def __init__(self, name, age):
            self.name = name
            self.age = age

    class Product:
        def __init__(self, user):
            self.user = user

    class UserSchema(schemars.Schema):
        name = schemars.Str()
        age = schemars.Int()

    class ProductSchema(schemars.Schema):
        user = UserSchema()

    schema = ProductSchema()
    user = User("John", 30)
    product = Product(user)
    result = schema.serialize(product)
    assert result == {"user": {"name": "John", "age": 30}}


def test_serialize_with_nested_many():
    class User:
        def __init__(self, name, age):
            self.name = name
            self.age = age

    class Product:
        def __init__(self, users):
            self.users = users

    class UserSchema(schemars.Schema):
        name = schemars.Str()
        age = schemars.Int()

    class ProductSchema(schemars.Schema):
        users = UserSchema(many=True)

    schema = ProductSchema()
    user = User("John", 30)
    product = Product([user])
    result = schema.serialize(product)
    assert result == {"users": [{"name": "John", "age": 30}]}


def test_serialize_with_context():
    class Product:
        def __init__(self, name):
            self.name = name

    class ProductSchema(schemars.Schema):
        name = schemars.Str()
        method = schemars.Method()

        def get_method(self, obj):
            return self.context.get("suffix")

    schema = ProductSchema(context={"suffix": "test"})
    product = Product("Product 1")
    result = schema.serialize(product)
    assert result == {"name": "Product 1", "method": "test"}


def test_serialize_with_inheritance():
    class Product:
        def __init__(self, name, display_name, related_products):
            self.name = name
            self.display_name = display_name
            self.related_products = related_products

    class BaseProductSchema(schemars.Schema):
        name = schemars.Str()
        display_name = schemars.Str()

    class ProductSchema(BaseProductSchema):
        related_products = BaseProductSchema(many=True)

    schema = ProductSchema()
    product = Product("Product 1", "Product 1", [Product("Product 2", "Product 2", [])])
    result = schema.serialize(product)
    assert result == {
        "name": "Product 1",
        "display_name": "Product 1",
        "related_products": [{"name": "Product 2", "display_name": "Product 2"}],
    }


def test_serialize_with_custom_attributes():
    class Product:
        pass

    class ProductSchema(schemars.Schema):
        method = schemars.Method()

        def get_method(self, obj):
            self.test = "test"
            return self.test

    schema = ProductSchema()
    product = Product()
    result = schema.serialize(product)
    assert result == {"method": "test"}

def test_serialize_with_alias():
    class Product:
        def __init__(self):
            self.name = "Product 1"

    class ProductSchema(schemars.Schema):
        name = schemars.Str(alias="my_name")

    schema = ProductSchema()
    product = Product()
    result = schema.serialize(product)
    assert result == {"my_name": "Product 1"}