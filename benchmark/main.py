#!/usr/bin/env python3
#
# Usage ./main.py <api-gateway-url>
#

import random
import string
import sys
from typing import List

import boto3
import requests


if len(sys.argv) <= 1:
    raise Exception('Must provide a URL parameter.')

URL = sys.argv[1]
API_KEY_NAME = 'benchmark'
TOTAL_REQUESTS = 1000


class Item:
    def __init__(self, id: int, description: str, count: int):
        self.id = id
        self.description = description
        self.count = count

    def __repr__(self):
        return f"{self.__class__}: {self.__dict__}"


def get_api_key() -> str:
    client = boto3.client('apigateway')
    resp = client.get_api_keys(limit=1, nameQuery=API_KEY_NAME, includeValues=True)
    return resp['items'][0]['value']

def make_requests(api_key: str, bodies: List[Item]):
    session = requests.Session()
    headers = { 'X-API-Key': api_key }
    for body in bodies:
        session.put(URL, headers=headers, json=body.__dict__)
        print(f"Performed PUT for item {body}")


if __name__ == '__main__':
    api_key = get_api_key()
    items = []
    for item_id in range(TOTAL_REQUESTS):
        description = ''.join(random.choices(string.ascii_uppercase + string.digits, k=50))
        items.append(Item(item_id, description, item_id + 25))

    make_requests(api_key, items)


