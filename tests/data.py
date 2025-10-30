from typing import Any
import string
import random
from typing import TypedDict


type JsonData = dict[str, Any]


class BenchmarkResult(TypedDict):
    size: int
    case_name: str
    qrydict: float
    jmespth: float


def rand_str(k: int) -> str:
    return "".join(random.choices(string.ascii_lowercase, k=k))


DATA_USER: dict[str, Any] = {
    "users": [
        {"name": "Ada", "age": 36},
        {"name": "Bob", "age": 17},
        {"name": "Cy", "age": 20},
    ]
}

DATA_MIXED: dict[str, Any] = {
    "foo": {"bar": [{"baz": 1}, {"baz": 2}]},
    "stats": {"a": 3, "b": 1, "c": 2},
    "arr": [3, 1, 2, 2],
    "nested": [[1, 2], [3], 4],
}

DATA_EDGE: dict[str, Any] = {
    "numbers": [0, 1, 2],
    "truth": [True, False],
    "obj": {"x": {"y": {"z": 5}}},
}


def generate_user(i: int) -> dict[str, Any]:
    return {
        "id": i,
        "name": rand_str(10),
        "age": random.randint(18, 65),
        "active": random.choice([True, False]),
        "tags": [
            random.choice(["tag1", "tag2", "tag3"]) for _ in range(random.randint(1, 5))
        ],
    }


def generate_data(n: int) -> JsonData:
    return {"users": [generate_user(i) for i in range(n)]}
