from typing import Any, Callable, Optional

class FieldBase:
    def __init__(
        self,
        write_only: bool = False,
        strict: bool = False,
        call: bool = False,
        default: Optional[Any] = None,
        source: Optional[str] = None,
        serialize_func: Optional[Callable] = None
    ) -> None: ...


class DateFieldBase(FieldBase):
    def __init__(
        self,
        format: Optional[str] = None,
        **kwargs: Any
    ) -> None: ...

class Str(FieldBase): ...
class Bytes(FieldBase): ...
class Int(FieldBase): ...
class Bool(FieldBase): ...
class Float(FieldBase): ...

class Decimal(FieldBase): ...

class Date(DateFieldBase): ...
class DateTime(DateFieldBase): ...

class Dict(FieldBase):
    def __init__(
        self,
        child: Optional[FieldBase] = None,
        **kwargs: Any
    ) -> None: ...

class List(FieldBase):
    def __init__(
        self,
        child: Optional[Any] = None,
        **kwargs: Any
    ) -> None: ...
    
class Uuid(FieldBase): ...

class Union(FieldBase):
    def __init__(
        self,
        fields: Any,
        **kwargs: Any
    ) -> None: ...

class Method:
    def __init__(self, method_name: Optional[str] = None) -> None: ...

class Schema:
    def __new__(cls, **kwargs) -> None: ...
    def __init__(self, **kwargs) -> None: ...
    def serialize(self, instance: Any, many: bool = False, cls: Optional[Any] = None) -> Any: ...

class ValidationError(BaseException):
    def __init__(self, errors: Any) -> None: ...
