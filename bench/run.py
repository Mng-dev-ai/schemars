import random
import time
from datetime import date
import string

import _drf
import _marshmallow
import _schemars


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

marshmallow_schema = _marshmallow.OuterSchema()
start_time = time.time()
serialized_data = marshmallow_schema.dump(instances, many=True)
print(f"--- Marshmallow Serialization took {time.time() - start_time} seconds ---")

serializer = _drf.OuterSerializer(instances, many=True)
start_time = time.time()
serialized_data = serializer.data
print(f"--- Drf Serialization took {time.time() - start_time} seconds ---")

schemars_schema = _schemars.OuterSchema()
start_time = time.time()
serialized_data = [schemars_schema.serialize(instance) for instance in instances]
print(f"--- Schemars Serialization took {time.time() - start_time} seconds ---")
