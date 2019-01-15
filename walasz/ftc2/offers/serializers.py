import datetime
from decimal import Decimal

from django.http import JsonResponse
from rest_framework import serializers
from rest_framework.compat import MinValueValidator
from rest_framework.response import Response

from offers.models import Offer, BuyNowOffer, AuctionOffer, OFFER_TYPE, AUCTION_STATUS


class OfferSerializer(serializers.Serializer):
    id = serializers.IntegerField(read_only=True)
    type = serializers.ChoiceField(OFFER_TYPE)
    description = serializers.CharField()
    owner = serializers.CurrentUserDefault()


class BuyNowOfferSerializer(OfferSerializer):
    amount = serializers.IntegerField(validators=[MinValueValidator(0)])
    price = serializers.DecimalField(max_digits=20, decimal_places=2,
                                     validators=[MinValueValidator(Decimal('0.01'))])

    def create(self, validated_data):
        return BuyNowOffer.objects.create(**validated_data)

    def update(self, instance, validated_data):
        instance.description = validated_data.get('description', instance.description)
        instance.price = validated_data.get('price', instance.price)
        instance.amount = validated_data.get('amount', instance.amount)
        instance.save()
        return instance


class AuctionOfferSerializer(OfferSerializer):
    date = serializers.IntegerField()
    status = serializers.ChoiceField(AUCTION_STATUS, default='active')
    last_bid = serializers.DecimalField(max_digits=20, decimal_places=2, default=0.0,
                                        read_only=True, validators=[MinValueValidator(Decimal('0.01'))])
    price = serializers.DecimalField(max_digits=20, decimal_places=2, write_only=True,
                                     validators=[MinValueValidator(Decimal('0.01'))])
    customer_id = serializers.PrimaryKeyRelatedField(many=False, default=None, read_only=True)

    def create(self, validated_data):
        data = validated_data.copy()
        data['last_bid'] = data.pop('price')
        return AuctionOffer.objects.create(**data)

    def update(self, instance, validated_data):
        instance.description = validated_data.get('description', instance.description)
        instance.last_bid = validated_data.get('price', instance.last_bid)
        # should price be changed??
        instance.save()
        return instance
