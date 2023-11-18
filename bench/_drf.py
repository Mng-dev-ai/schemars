from rest_framework import serializers
from django.conf import settings
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

