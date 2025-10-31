from typing import Any
import random
from typing import TypedDict, NamedTuple

from enum import StrEnum, auto

from factory import base, Sequence, LazyAttribute, DictFactory
import factory.fuzzy as fz
from faker import Faker

type JsonData = dict[str, Any]


class BenchmarkResult(NamedTuple):
    size: int
    query: str
    qrydict: float
    jmespth: float


class Tags(StrEnum):
    NEW = auto()
    POPULAR = auto()
    LIMITED = auto()
    EXCLUSIVE = auto()


fake = Faker()


class UserFactory(base.DictFactory):
    id = Sequence(lambda n: n + 1)
    name = LazyAttribute(lambda _: fake.name())
    age = LazyAttribute(lambda _: fake.random_int(min=18, max=65))
    active = LazyAttribute(lambda _: fake.pybool())
    tags = LazyAttribute(
        lambda _: [
            random.choice(["tag1", "tag2", "tag3"]) for _ in range(random.randint(1, 3))
        ]
    )


class ProductFactory(base.DictFactory):
    product_id = Sequence(lambda n: n + 1)
    name = LazyAttribute(lambda _: fake.word().capitalize())
    price = LazyAttribute(
        lambda _: round(fake.pyfloat(min_value=5.0, max_value=100.0, right_digits=2), 2)
    )
    in_stock = LazyAttribute(lambda _: fake.pybool())
    tag = fz.FuzzyChoice([tag.value for tag in Tags])


class SaleRecordFactory(DictFactory):
    order_id = Sequence(lambda n: n + 1)
    customer_id = LazyAttribute(lambda o: o.customer["id"])
    product_id = LazyAttribute(lambda o: o.product["product_id"])
    items = LazyAttribute(lambda _: fake.random_int(min=1, max=10))
    amount = LazyAttribute(lambda o: round(o.product["price"] * o.items, 2))
    shipped = LazyAttribute(lambda _: fake.pybool())

    class Params:
        customer = None
        product = None


class DataBase(TypedDict):
    users: list[dict[str, Any]]
    sales: list[dict[str, Any]]
    products: list[dict[str, Any]]
    tags: dict[str, int]
    metadata: dict[str, Any]


def _metadata() -> dict[str, Any]:
    return {
        "a": {"b": [{"c": 1}, {"c": 2}]},
        "d": {"e": 3, "f": 1, "g": 2},
        "h": [3, 1, 2, 2],
        "i": [[1, 2], [3], 4],
        "j": [0, 1, 2],
        "k": [True, False],
        "l": {"m": {"n": {"o": 5}}},
        "m": ["a", "b", "c"],
    }


def generate_db(n: int) -> DataBase:
    product_count = 20
    users: list[dict[str, Any]] = UserFactory.build_batch(n)
    products: list[dict[str, Any]] = ProductFactory.build_batch(product_count)
    sales = [
        SaleRecordFactory.build(
            customer=random.choice(users), product=random.choice(products)
        )
        for _ in range(n * 2)
    ]

    return DataBase(
        users=users,
        sales=sales,
        products=products,
        tags=dict((tag.value, i) for i, tag in enumerate(Tags)),
        metadata=_metadata(),
    )
