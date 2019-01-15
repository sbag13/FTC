import time
from decimal import Decimal

from django.db.models import Q
from rest_framework import mixins, generics, status
from rest_framework.permissions import IsAuthenticatedOrReadOnly
from rest_framework.views import APIView
from rest_framework.response import Response

from offers.models import Offer, AuctionOffer, BuyNowOffer
from offers.serializers import OfferSerializer, BuyNowOfferSerializer, AuctionOfferSerializer
from users.models import User
from itertools import chain


class OfferFilterException(Exception):
    def __init__(self, message, filter):
        super(OfferFilterException, self).__init__(message)
        self.filter = filter


class OfferList(mixins.ListModelMixin,
                mixins.CreateModelMixin,
                generics.GenericAPIView):
    permission_classes = ((IsAuthenticatedOrReadOnly,))

    def get_serializer_class(self):
        offer_type = self.request.data.get('type', None)
        if offer_type == 'buynow':
            return BuyNowOfferSerializer
        elif offer_type == 'auction':
            return AuctionOfferSerializer
        else:
            return OfferSerializer

    def get_queryset(self):
        filters = self.request.query_params
        auction_offers = AuctionOffer.objects.all()
        buynow_offers = BuyNowOffer.objects.all()
        allowed_filters = ['contains', 'price_min', 'price_max', 'type', 'created_be_me']
        improper_filters = [f for f in filters if f not in allowed_filters]
        if improper_filters:
            raise OfferFilterException(f'filters {improper_filters} are not allowed', improper_filters)
        contains = filters.get('contains', None)
        price_min = filters.get('price_min', None)
        price_max = filters.get('price_max', None)
        type = filters.get('type', None)
        created_by_me = filters.get('created_by_me', None)

        auction_query = Q()
        buynow_query = Q()
        if contains:
            auction_query &= Q(description__contains=contains)
            buynow_query &= Q(description__contains=contains)
        if price_min:
            auction_query &= Q(last_bid__gte=Decimal(price_min))
            buynow_query &= Q(price__gte=Decimal(price_min))

        if price_max:
            auction_query &= Q(last_bid__lte=Decimal(price_max))
            buynow_query &= Q(price__lte=Decimal(price_max))

        if created_by_me:
            auction_query &= Q(owner__id__eq=self.request.user.id)
            buynow_query &= Q(owner__id__eq=self.request.user.id)

        auction_offers = auction_offers.filter(auction_query)
        buynow_offers = buynow_offers.filter(buynow_query)

        queryset = sorted(chain(auction_offers, buynow_offers),
                          key=lambda instance: instance.id)

        return queryset

    def get(self, request, *args, **kwargs):
        try:
            return self.list(request, *args, **kwargs)
        except OfferFilterException as offer_exception:
            return Response({"details": f"filters {offer_exception.filter} are not allowed"}, status=400)


    def post(self, request, *args, **kwargs):
        serializer = self.get_serializer(data=request.data)
        serializer.is_valid(raise_exception=True)
        offer_type = serializer.validated_data.get('type')
        if offer_type not in ['auction', 'buynow']:
            return Response({"details": "type must be auction or buynow"}, status=status.HTTP_400_BAD_REQUEST)

        if offer_type == 'buynow':
            missing = [k for k in ['description', 'price', 'amount'] if k not in serializer.validated_data.keys()]
            if missing:
                return Response({"details": f"{missing} are mandatory for offers of typt buynow"},
                                status=status.HTTP_400_BAD_REQUEST)

        serializer.save(owner=request.user)
        headers = self.get_success_headers(serializer.data)
        return Response({"offer_id": serializer.data.get('id')}, status=status.HTTP_201_CREATED, headers=headers)


class OfferDetail(generics.RetrieveUpdateDestroyAPIView):
    permission_classes = ((IsAuthenticatedOrReadOnly,))

    def get_queryset(self):
        pk = self.kwargs.get('pk')
        offer_type = Offer.objects.get(pk=pk).type
        if offer_type == 'buynow':
            queryset = BuyNowOffer.objects.all()
        elif offer_type == 'auction':
            queryset = AuctionOffer.objects.all()
        else:
            raise ValueError(f'unknown offer type {offer_type}')
        return queryset

    def get_serializer_class(self):
        obj = self.get_object()
        offer_type = obj.type
        if offer_type == 'buynow':
            return BuyNowOfferSerializer
        elif offer_type == 'auction':
            return AuctionOfferSerializer
        else:
            return OfferSerializer


class BuyView(APIView):
    def post(self, request, pk):
        offer = Offer.objects.get(pk=pk)
        if request.user == offer.owner:
            return Response({"conflict": "unable to order own items"}, status=409)
        offer_type = offer.type
        keys = list(request.data.keys())
        if offer_type == 'buynow':
            if ['amount'] != keys:
                return Response({"details": 'Request should contains only "amount" field'}, status=400)
            amount = request.data.get('amount', None)
            if not isinstance(amount, int):
                return Response({"details": '"amount" field must be integer'}, status=400)
            offer = BuyNowOffer.objects.get(pk=pk)
            max_amount = offer.amount
            if amount > max_amount:
                return Response({"max_amount": max_amount}, status=409)
            offer.amount -= 1
            offer.save()

        else:  # auction
            if ['bid'] != keys:
                return Response({"details": 'Request should contains only "bid" field'}, status=400)
            bid = request.data.get('bid', None)
            if not (isinstance(bid, int) or isinstance(bid, float)):
                return Response({"details": '"bid" field must be integer or float'}, status=400)
            offer = AuctionOffer.objects.get(pk=pk)
            if time.time() > offer.date:
                offer.status = 'expired'
                offer.save()
            if offer.status == 'expired':
                return Response({"status": "expired"}, status=409)
            last_bid = float(offer.last_bid)
            if bid < round(last_bid + 0.01, 2):
                return Response({"minimal_bid": round(last_bid + 0.01, 2)}, status=409)
            # todo update user - (winning user)
            print('new bid:', bid)
            offer.last_bid = bid
            offer.save()

        return Response(status=202)
