import schemars

class InnerSchema(schemars.Schema):
    field1 = schemars.Int()
    field2 = schemars.Str()

class MiddleSchema(schemars.Schema):
    inner = InnerSchema(many=True)
    field3 = schemars.Str()

class OuterSchema(schemars.Schema):
    middle = MiddleSchema(many=True)
    field4 = schemars.Date()

