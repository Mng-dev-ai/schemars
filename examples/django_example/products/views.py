from rest_framework.views import APIView
from rest_framework.response import Response
import schemars

from .models import Product
from .schema import ProductSchema

class ProductView(APIView):
    def get(self, request):
        products = Product.objects.all()
        try:
            data =  ProductSchema(context={'request': request}).serialize(products, many=True)
        except schemars.ValidationError as e:
            return Response(e.errors, status=400)
        return Response(data)