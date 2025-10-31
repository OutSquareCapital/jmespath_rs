from typing import Any
import random
from typing import TypedDict, NamedTuple

from enum import StrEnum, auto
from factory import base, Sequence, LazyAttribute, DictFactory
from faker import Faker

type Table = list[dict[str, Any]]


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

    @classmethod
    def to_dict(cls):
        return dict((tag.value, i) for i, tag in enumerate(cls))


TAGS_LIST = [tag.value for tag in Tags]
CATEGORIES: list[str] = ["VIP", "Regular", "Guest"]

fake = Faker()


class UserFactory(base.DictFactory):
    id = Sequence(lambda n: n + 1)
    name = LazyAttribute(lambda _: fake.name())
    address = LazyAttribute(
        lambda _: {"street": fake.street_address(), "city": fake.city()}
    )
    age = LazyAttribute(lambda _: fake.random_int(min=18, max=65))
    active = LazyAttribute(lambda _: fake.pybool())
    category = LazyAttribute(
        lambda _: [random.choice(CATEGORIES) for _ in range(random.randint(1, 3))]
    )


class ProductFactory(base.DictFactory):
    product_id = Sequence(lambda n: n + 1)
    name = LazyAttribute(lambda _: fake.word().capitalize())
    price = LazyAttribute(
        lambda _: round(fake.pyfloat(min_value=5.0, max_value=100.0, right_digits=2), 2)
    )
    in_stock = LazyAttribute(lambda _: fake.pybool())
    tags = LazyAttribute(lambda _: random.choices(TAGS_LIST))


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
    users: Table
    sales: Table
    products: Table
    tags: dict[str, int]


def generate_db(n: int) -> DataBase:
    users: Table = UserFactory.build_batch(n)
    products: Table = ProductFactory.build_batch(20)

    return DataBase(
        users=users,
        sales=_get_sales(users, products, n),
        products=products,
        tags=Tags.to_dict(),
    )


def _get_sales(users: Table, products: Table, n: int) -> Table:
    return SaleRecordFactory.build_batch(
        n * 2, customer=random.choice(users), product=random.choice(products)
    )
