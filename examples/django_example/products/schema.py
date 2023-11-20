import schemars

class TagSchema(schemars.Schema):
    name = schemars.Str()
    
class BaseProductSchema(schemars.Schema):
    name = schemars.Str()
    description = schemars.Str()
    tags = TagSchema(many=True, source='tags.all', call=True)
    created_at = schemars.DateTime()
    updated_at = schemars.DateTime()
    

class ProductSchema(BaseProductSchema):
    related_products = BaseProductSchema(many=True, source='related_products.all', call=True)
    ip = schemars.Method()
    
    def get_ip(self, obj):
        request = self.context.get('request')
        return request.META.get('REMOTE_ADDR')

