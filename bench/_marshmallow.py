from marshmallow import Schema, fields

class InnerSchema(Schema):
    field1 = fields.Int()
    field2 = fields.Str()

class MiddleSchema(Schema):
    inner = fields.Nested(InnerSchema, many=True)
    field3 = fields.Str()

class OuterSchema(Schema):
    middle = fields.Nested(MiddleSchema, many=True)
    field4 = fields.Date()