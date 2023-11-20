from django.contrib import admin
from .models import Product, Tag

admin.site.register(Product)
admin.site.register(Tag)