from django.db import models
from users.models import User

AUCTION = 'auction'
BUYNOW = 'buynow'
OFFER_TYPE = (
    (AUCTION, 'auction'),
    (BUYNOW, 'buynow'),
)


class Offer(models.Model):
    type = models.CharField(max_length=10, choices=OFFER_TYPE)
    description = models.TextField()
    owner = models.ForeignKey(User, on_delete=models.CASCADE)

    # class Meta:
    #     abstract = True


class BuyNowOffer(Offer):
    # todo - type = 'buynow'
    price = models.DecimalField(max_digits=20, decimal_places=2)
    amount = models.BigIntegerField()


ACTIVE = 'active'
EXPIRED = 'expired'
AUCTION_STATUS = (
    (ACTIVE, 'active'),
    (EXPIRED, 'expired'),
)


class AuctionOffer(Offer):
    # todo - type = 'auction'
    date = models.BigIntegerField()  # NOTE: this is expiration date, not creation date!
    # date = models.DateTimeField()  # NOTE: this is expiration date, not creation date!
    # date = UnixDateTimeField()  # NOTE: this is expiration date, not creation date!
    status = models.CharField(max_length=10, choices=AUCTION_STATUS)
    last_bid = models.DecimalField(max_digits=20, decimal_places=2)
    customer_id = models.ForeignKey(User, on_delete=models.PROTECT, null=True)  # todo - SET to NULL?
