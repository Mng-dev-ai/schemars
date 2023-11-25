from schemars._schemars import (
    Schema as RustSchema,
    ValidationError,
    Str,
    Int,
    Bool,
    Float,
    Date,
    DateTime,
    Dict,
    List,
    Union,
    Method,
    Decimal,
    Bytes,
    Uuid,
    Any,
)

class Schema(RustSchema):
    field_types = (
        Str,
        Int,
        Bool,
        Float,
        Date,
        DateTime,
        Dict,
        List,
        Union,
        Method,
        Decimal,
        Bytes,
        Uuid,
        Any
    )

    def __new__(cls, **kwargs):
        fields = cls._collect_fields(cls)
        return RustSchema.__new__(
            cls,
            fields=fields,
            write_only=kwargs.get("write_only", False),
            strict=kwargs.get("strict", False),
            default=kwargs.get("default", None),
            source=kwargs.get("source", None),
            call=kwargs.get("call", False),
            serialize_func=kwargs.get("serialize_func", None),
            context=kwargs.get("context", {}),
            alias=kwargs.get("alias", None),
        )

    @classmethod
    def _collect_fields(cls, target_class):
        fields = {}
        for klass in reversed(target_class.mro()):
            for key, value in klass.__dict__.items():
                if isinstance(value, cls.field_types):
                    fields[key] = value
                elif isinstance(value, Schema):
                    fields[key] = (value, getattr(value, "many", False))
        return fields

    def __init__(self, **kwargs):
        self.many = kwargs.get("many", False)
        self.source = kwargs.get("source", None)
        self.write_only = kwargs.get("write_only", False)
        self.strict = kwargs.get("strict", False)
        self.call = kwargs.get("call", False)
        self.default = kwargs.get("default", None)
        self.serialize_func = kwargs.get("serialize_func", None)
        self.context = kwargs.get("context", {})
        self.alias = kwargs.get("alias", None)
        
    def serialize(self, instance, many=None):
        many = self.many if many is None else many
        return super().serialize(instance, many, self.__class__)
