import random
import time

import jwt
import pytest
import requests
from faker import Faker
import logging

HOST = '127.0.0.1'
PORT = '8000'
PROTOCOL = 'http://'
BASE_URL = f'{PROTOCOL}{HOST}:{PORT}'

fake = Faker()

logging.basicConfig(
    level=logging.INFO,
    format="[%(levelname)-5.5s]  %(message)s",
    handlers=[
        logging.StreamHandler()
    ])

logger = logging.getLogger('api_logger')


@pytest.fixture(scope='session')
def known_user_credentials():
    email = 'user@gmail.com'
    password = 'prostehaslo123'
    return email, password


@pytest.fixture(scope='session')
def jwt_token_header():
    url = f'{BASE_URL}/login/'
    response = requests.post(url=url,
                             json={
                                 "email": 'user@gmail.com',
                                 "password": 'prostehaslo123'
                             })
    return {'Authorization': 'JWT {}'.format(response.json().get('token'))}


@pytest.fixture()
def password():
    return 'prostehaslo123'


def _validate_status_code(actual, expected, additional_info=None):
    msg = f'unexpected status code {actual} instead of {expected}'
    if additional_info:
        msg = f'{msg}, {additional_info}'
    assert expected == actual, msg


def _validate_offer_id(response):
    assert ["offer_id"] == list(response.json().keys()), \
        'response should contains only "offer_id" field'
    return response.json().get('offer_id')


def _create_offer(data, headers=None) -> requests.Response:
    url = f'{BASE_URL}/offers/'
    h = {"Content-Type": "application/json"}
    if headers:
        h.update(headers)
    response = requests.post(url, json=data, headers=headers)
    logger.info(f'creating offer {data} for test: {response.status_code}, {response.content}')
    return response


# @pytest.mark.skip
class TestUserRegistration:
    url = f'{BASE_URL}/registration/'

    @staticmethod
    def _log_test(data, response):
        logger.info(f'Creating new user with data: {data} status code: {response.status_code}, '
                    f'response: {response.content}')

    def _register(self, data) -> requests.Response:
        return requests.post(url=self.url, json=data)

    def test_improper_content_type(self):
        data = 'not a json'
        response = requests.post(url=self.url, data=data)
        if response.status_code != 400:
            logger.info(f'Response for request with improper body returns: {response.status_code}, {response.content}')
        _validate_status_code(actual=response.status_code, expected=400)

    def test_improper_json(self):
        data = 'not a json'
        response = requests.post(url=self.url, data=data, headers={"Content-Type": "application/json"})
        if response.status_code != 400:
            logger.info(f'Response for request with improper body returns: {response.status_code}, {response.content}')
        _validate_status_code(actual=response.status_code, expected=400)

    def test_register_new_user(self, password):
        data = {
            "email": fake.email(),
            "password": password
        }
        response = self._register(data=data)
        self._log_test(data=data, response=response)
        _validate_status_code(actual=response.status_code, expected=201)

    def test_register_user_with_improper_email(self, password):
        data = {
            "email": 'improper_email',
            "password": password
        }
        response = self._register(data=data)
        self._log_test(data=data, response=response)
        _validate_status_code(actual=response.status_code, expected=400)

    def test_register_existing_user(self, known_user_credentials):
        data = {
            "email": known_user_credentials[0],
            "password": known_user_credentials[1]
        }
        response = self._register(data=data)
        self._log_test(data=data, response=response)
        _validate_status_code(actual=response.status_code, expected=409)

    def test_register_with_improper_key_name(self, password):
        data = {
            "emaaaaaaaaaaail": fake.email(),
            "password": password
        }
        response = self._register(data=data)
        self._log_test(data=data, response=response)
        _validate_status_code(actual=response.status_code, expected=400)

    def test_register_with_aditional_key_in_body(self, password):
        data = {
            "email": fake.email(),
            "password": password,
            "foo": 'bar'
        }
        response = self._register(data=data)
        self._log_test(data=data, response=response)
        _validate_status_code(actual=response.status_code, expected=400)

    def test_register_with_missing_key_in_body(self):
        data = {
            "email": fake.email(),
        }
        response = self._register(data=data)
        self._log_test(data=data, response=response)
        _validate_status_code(actual=response.status_code, expected=400)

    @pytest.mark.parametrize('method', ['GET', 'PUT', 'DELETE'])
    def test_register_other_than_post_methods(self, method, password):
        data = {
            "email": fake.email(),
            "password": password
        }
        response = requests.request(method=method, url=f'{BASE_URL}/registration/', json=data)
        _validate_status_code(actual=response.status_code, expected=405)


@pytest.mark.skip
class TestUserLogin:
    def test_login_known_user(self, known_user_credentials):
        logger.info('Log in as known user with credentials: {}'.format(known_user_credentials))
        url = f'{BASE_URL}/login/'
        response = requests.post(url=url,
                                 json={
                                     "email": known_user_credentials[0],
                                     "password": known_user_credentials[1]
                                 })
        _validate_status_code(actual=response.status_code, expected=200)
        response_json = response.json()
        assert 'token' in response_json, '/login should return JWT token for proper user'
        decoded_token = jwt.decode(response_json.get('token'), verify=False)
        username = decoded_token.get('username', None)
        email = decoded_token.get('email', None)
        assert username or email in decoded_token, 'Token should contains username or email'
        if username:
            assert known_user_credentials[0] == username, f'Unexpected username {username} in JWT token'
        if email:
            assert known_user_credentials[0] == email, f'Unexpected email {email} in JWT token'

    def test_login_with_wrong_password(self, known_user_credentials):
        logger.info('Log in as known user with credentials: {}'.format(known_user_credentials))
        url = f'{BASE_URL}/login/'
        response = requests.post(url=url,
                                 json={
                                     "email": known_user_credentials[0],
                                     "password": "this_is_probably_wrong_password"
                                 })
        _validate_status_code(actual=response.status_code, expected=401)

    def test_login_unknown_user(self):
        fake_email = fake.email()
        logger.info(f'Log in as unknown random user with credentials: ({fake_email}, "prostehaslo123"')
        url = f'{BASE_URL}/login/'
        response = requests.post(url=url,
                                 json={
                                     "email": fake_email,
                                     "password": "prostehaslo123"
                                 })
        _validate_status_code(actual=response.status_code, expected=404)

    @pytest.mark.parametrize('method', ['GET', 'PUT', 'DELETE'])
    def test_login_wrong_http_method(self, method, known_user_credentials):
        url = f'{BASE_URL}/login/'
        response = requests.request(method=method,
                                    url=url,
                                    json={
                                        "email": known_user_credentials[0],
                                        "password": known_user_credentials[1]
                                    })
        _validate_status_code(actual=response.status_code, expected=405)


@pytest.mark.skip
class TestOfferCreate:
    url = f'{BASE_URL}/offers/'

    # @pytest.mark.skip
    def test_create_buynow_offer(self, jwt_token_header):
        data = {
            "type": "buynow",
            "description": fake.sentence(),
            "amount": 10,
            "price": 55.64
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        _validate_status_code(actual=r.status_code, expected=201)
        offer_id = _validate_offer_id(r)
        logger.info(f'created offer with id {offer_id}')

    def test_create_buynow_offer_negative_amount(self, jwt_token_header):
        data = {
            "type": "buynow",
            "description": fake.sentence(),
            "amount": -10,
            "price": 55.64
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        _validate_status_code(actual=r.status_code, expected=400)

    def test_create_buynow_offer_negative_price(self, jwt_token_header):
        data = {
            "type": "buynow",
            "description": fake.sentence(),
            "amount": 10,
            "price": -55.64
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        _validate_status_code(actual=r.status_code, expected=400)

    # @pytest.mark.skip
    def test_create_improper_type_offer(self, jwt_token_header):
        data = {
            "type": "improper_type",
            "description": fake.sentence(),
            "amount": 10,
            "price": 55.64
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        _validate_status_code(actual=r.status_code, expected=400)

    def test_create_auction_offer(self, jwt_token_header):
        data = {
            "type": "auction",
            "description": fake.sentence(),
            "date": int(time.time()) + 3600 * 24,
            "price": 35.25
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        _validate_status_code(actual=r.status_code, expected=201)
        offer_id = _validate_offer_id(r)
        logger.info(f'created offer with id {offer_id}')
        # todo - validate offer parameters

    def test_create_auction_offer_with_negative_minimal_price(self, jwt_token_header):
        data = {
            "type": "auction",
            "description": fake.sentence(),
            "date": int(time.time()) + 3600 * 24,
            "price": -35.25
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        logger.info(f'Creating offer auction with negative minimal price- status {r.status_code}, {r.content}')
        _validate_status_code(actual=r.status_code, expected=400)

    # @pytest.mark.skip
    def test_create_offer_without_type(self, jwt_token_header):
        data = {
            "description": fake.sentence(),
            "amount": 10,
            "price": 55.64
        }
        r = _create_offer(data=data, headers=jwt_token_header)
        logger.info(f'Creating offer without specifying type - status {r.status_code}, {r.content}')
        _validate_status_code(actual=r.status_code, expected=400)


class TestOfferSearch:
    url = f'{BASE_URL}/offers'

    def _get_offers(self, params=None, headers=None) -> requests.Response:
        h = {"Content-Type": "application/json"}
        if headers:
            h.update(headers)
        return requests.get(self.url, params=params, headers=headers)

    # @pytest.mark.skip
    def test_offer_list_unauthorized_user(self):
        r = self._get_offers()
        logger.info(f'get offers (unauthorized - not logged in user): '
                    f'status {r.status_code}, {r.content}')
        _validate_status_code(actual=r.status_code, expected=200,
                              additional_info='Unauthorized user should be able to get list of offers')

    @pytest.mark.skip
    def test_offer_list_authorized_user(self, jwt_token_header):
        r = self._get_offers(headers=jwt_token_header)
        logger.info(f'get offers (logged in user): '
                    f'status {r.status_code}, {r.content}')
        _validate_status_code(actual=r.status_code, expected=200)

    @pytest.mark.skip
    def test_improper_filter(self):
        params = {'foo': 'bar'}
        response = self._get_offers(params=params)
        logger.info(f'sent request: {response.url}, response: {response.content}')
        _validate_status_code(actual=response.status_code, expected=400)

    @pytest.mark.parametrize('data', [
        {
            "type": "auction",
            "description": '',
            "date": int(time.time()) + 3600 * 24,
            "price": 35.25},
        {
            "type": "buynow",
            "description": '',
            "amount": 10,
            "price": 55.64
        }], ids=['auction', 'buynow'])
    @pytest.mark.skip
    def test_full_description_as_contains_filter(self, data, jwt_token_header):
        description = fake.sentence()
        data['description'] = description
        created_offer = _create_offer(data=data, headers=jwt_token_header)
        created_offer = created_offer.json().get('offer_id')
        params = {"contains": description}
        response = self._get_offers(params=params)
        logger.info(f'sent request: {response.url}, response: {response.content}')
        _validate_status_code(actual=response.status_code, expected=200)
        response = response.json()
        assert response, f'got empty list, offer with description {description} not found'
        for offer in response:
            assert description == offer.get('description'), \
                f'Response contains offers with different description than excepted "{description}"'
        assert next((offer for offer in response if offer.get('id') == created_offer), None), \
            f'Response not contains created offer with id {created_offer}'

    def test_filter_by_price_max_auction(self, jwt_token_header):
        description = fake.sentence()
        random_price = random.randint(500, 1000)
        offer_1 = _create_offer(data={
            "type": "auction",
            "description": description,
            "date": int(time.time()) + 3600 * 24,
            "price": random_price + 1
        }, headers=jwt_token_header)
        _validate_status_code(actual=offer_1.status_code, expected=201)
        offer_2 = _create_offer(data={
            "type": "auction",
            "description": description,
            "date": int(time.time()) + 3600 * 24,
            "price": random_price - 1
        }, headers=jwt_token_header)
        _validate_status_code(actual=offer_2.status_code, expected=201)
        params = {"price_max": random_price}
        response = self._get_offers(params=params)
        logger.info(f'send request {response.url}, {response.content}')
        response = response.json()
        assert response, f'Got empty response for {response.url}'
        offers = [offer for offer in response if offer.get('description') == description]
        logger.info(f'filter offers in response by description "{description}": {offers}')
        assert offers, f'offers with description "{description}" not found'
        offer_1 = next((o for o in offers if o.get('id') == offer_1.json().get('offer_id')), None)
        offer_2 = next((o for o in offers if o.get('id') == offer_2.json().get('offer_id')), None)
        assert not offer_1, f'offer 1 with price bigger than {random_price} should be NOT present in {response}'
        assert offer_2, f'offer 2 with price smaller than {random_price} should be present in {response}'

    # @pytest.mark.skip
    def test_filter_by_price_max_buynow(self, jwt_token_header):
        description = fake.sentence()
        random_price = random.randint(500, 1000)
        offer_1 = _create_offer(data={
            "type": "buynow",
            "description": description,
            "amount": 10,
            "price": random_price + 1
        }, headers=jwt_token_header)
        _validate_status_code(actual=offer_1.status_code, expected=201)
        offer_2 = _create_offer(data={
            "type": "buynow",
            "description": description,
            "amount": 10,
            "price": random_price - 1
        }, headers=jwt_token_header)
        _validate_status_code(actual=offer_2.status_code, expected=201)
        params = {"price_max": random_price}
        response = self._get_offers(params=params)
        logger.info(f'send request {response.url}, {response.content}')
        response = response.json()
        assert response, f'Got empty response for {response.url}'
        offers = [offer for offer in response if offer.get('description') == description]
        logger.info(f'filter offers in response by description "{description}": {offers}')
        assert offers, f'offers with description "{description}" not found'
        offer_1 = next((o for o in offers if o.get('id') == offer_1.json().get('offer_id')), None)
        offer_2 = next((o for o in offers if o.get('id') == offer_2.json().get('offer_id')), None)
        assert not offer_1, f'offer 1 with price bigger than {random_price} should be NOT present in {response}'
        assert offer_2, f'offer 2 with price smaller than {random_price} should be present in {response}'
