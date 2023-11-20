from rest_framework import serializers
from django.conf import settings
from marshmallow import Schema, fields
import schemars
import random
import time
from datetime import date
import string
from pydantic import BaseModel
from typing import List

settings.configure()


class InnerSerializer(serializers.Serializer):
    field1 = serializers.IntegerField()
    field2 = serializers.CharField()


class MiddleSerializer(serializers.Serializer):
    inner = InnerSerializer(many=True)
    field3 = serializers.CharField()


class OuterSerializer(serializers.Serializer):
    middle = MiddleSerializer(many=True)
    field4 = serializers.DateField()


class InnerSchema(Schema):
    field1 = fields.Int()
    field2 = fields.Str()


class MiddleSchema(Schema):
    inner = fields.Nested(InnerSchema, many=True)
    field3 = fields.Str()


class OuterSchema(Schema):
    middle = fields.Nested(MiddleSchema, many=True)
    field4 = fields.Date()


class SchemarsInnerSchema(schemars.Schema):
    field1 = schemars.Int()
    field2 = schemars.Str()


class SchemarsMiddleSchema(schemars.Schema):
    inner = SchemarsInnerSchema(many=True)
    field3 = schemars.Str()


class SchemarsOuterSchema(schemars.Schema):
    middle = SchemarsMiddleSchema(many=True)
    field4 = schemars.Date()


def random_string(length=5):
    letters = string.ascii_lowercase
    return "".join(random.choice(letters) for _ in range(length))


class Inner:
    def __init__(self, field1, field2):
        self.field1 = field1
        self.field2 = field2


class Middle:
    def __init__(self, inner, field3):
        self.inner = inner
        self.field3 = field3


class Outer:
    def __init__(self, middle, field4):
        self.middle = middle
        self.field4 = field4


instances = [
    Outer(
        middle=[
            Middle(
                inner=[
                    Inner(field1=random.randint(0, 100), field2=random_string())
                    for _ in range(10)
                ],
                field3=random_string(),
            )
            for _ in range(10)
        ],
        field4=date.today(),
    )
    for _ in range(1000)
]


class PydanticInnerSchema(BaseModel):
    field1: int
    field2: str


class PydanticMiddleSchema(BaseModel):
    inner: List[PydanticInnerSchema]
    field3: str


class PydanticOuterSchema(BaseModel):
    middle: List[PydanticMiddleSchema]
    field4: date


pydantic_instances = [
    PydanticOuterSchema(
        middle=[
            PydanticMiddleSchema(
                inner=[
                    PydanticInnerSchema(
                        field1=random.randint(0, 100), field2=random_string()
                    )
                    for _ in range(10)
                ],
                field3=random_string(),
            )
            for _ in range(10)
        ],
        field4=date.today(),
    )
    for _ in range(1000)
]

marshmallow_schema = OuterSchema()
start_time = time.time()
marshmallow_serialized_data = marshmallow_schema.dump(instances, many=True)
marshmallow_time = time.time() - start_time

serializer = OuterSerializer(instances, many=True)
start_time = time.time()
drf_serialized_data = serializer.data
drf_time = time.time() - start_time

schemars_schema = SchemarsOuterSchema()
start_time = time.time()
schemars_serialized_data = [
    schemars_schema.serialize(instance) for instance in instances
]
schemars_time = time.time() - start_time

pydantic_schema = PydanticOuterSchema
start_time = time.time()
pydantic_serialized_data = [
    pydantic_schema.model_dump(instance) for instance in pydantic_instances
]
pydantic_time = time.time() - start_time

print(f"Marshmallow: {marshmallow_time}")
print(f"DRF: {drf_time}")
print(f"Schemars: {schemars_time}")
print(f"Pydantic: {pydantic_time}")
