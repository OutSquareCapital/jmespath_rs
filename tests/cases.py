from tests.bench import Case
import jmespath_rs as qd

CASES: list[Case] = [
    Case(
        qd.field("metadata").a.b.index(0).c,
        "metadata.a.b[0].c",
    ),
    Case(
        qd.field("users").index(0).name,
        "users[0].name",
    ),
    Case(
        qd.field("users").slice(1, 10, 2),
        "users[1:10:2]",
    ),
    Case(
        qd.field("users").project("name"),
        "users[].name",
    ),
    Case(
        qd.field("metadata").d.vproject("e"),
        "metadata.d.*.e",
    ),
    Case(
        qd.field("metadata").i.flatten(),
        "metadata.i[][]",
    ),
    Case(
        qd.field("users").pipe(qd.identity().length()),
        "length(users)",
    ),
    Case(
        qd.field("users")
        .filter(qd.field("age").ge(30).and_(qd.field("active").eq(True)))
        .then(qd.field("name")),
        "users[?age >= `30` && active == `true`].name",
    ),
    Case(
        qd.select_dict(
            names=qd.field("users").project("name"), count=qd.field("users").length()
        ),
        "{names: users[].name, count: length(users)}",
    ),
    Case(
        qd.select_list(qd.field("users").index(0), qd.field("products").index(0)),
        "[users[0], products[0]]",
    ),
    Case(
        qd.field("metadata").j.index(1).eq(1),
        "metadata.j[1] == `1`",
    ),
    Case(
        qd.field("metadata").k.index(0).eq(True),
        "metadata.k[0] == `true`",
    ),
    Case(
        qd.field("metadata")
        .l.m.n.o.gt(1)
        .and_(qd.field("metadata").l.m.n.o.eq(5).not_())
        .or_(0),
        "(metadata.l.m.n.o > `1` && !(metadata.l.m.n.o == `5`)) || `0`",
    ),
    Case(qd.field("metadata").d.keys(), "keys(metadata.d)"),
    Case(qd.field("users").length(), "length(users)"),
    Case(
        qd.field("users").project("name").map_with(qd.identity().length()),
        "map(&length(@), users[].name)",
    ),
    Case(
        qd.field("products").max_by(qd.field("price")),
        "max_by(products, &price)",
    ),
    Case(
        qd.field("users").min_by(qd.field("age")),
        "min_by(users, &age)",
    ),
    Case(qd.field("metadata").h.sort(), "sort(metadata.h)"),
    Case(
        qd.field("users").sort_by(qd.field("age")).project("name"),
        "sort_by(users, &age)[].name",
    ),
    Case(qd.field("metadata").a.to_array(), "to_array(metadata.a)"),
    Case(qd.field("metadata").d.to_string(), "to_string(metadata.d)"),
    Case(qd.lit("42").to_number(), "to_number(`42`)"),
    Case(
        qd.field("metadata").d.values(),
        "values(metadata.d)",
    ),
    Case(
        qd.field("metadata").h.map_with(qd.identity().abs()),
        "map(&abs(@), metadata.h)",
    ),
    Case(
        qd.field("metadata").h.avg(),
        "avg(metadata.h)",
    ),
    Case(
        qd.field("metadata").h.map_with(qd.identity().ceil()),
        "map(&ceil(@), metadata.h)",
    ),
    Case(
        qd.field("metadata").h.map_with(qd.identity().floor()),
        "map(&floor(@), metadata.h)",
    ),
    Case(
        qd.field("metadata").h.max(),
        "max(metadata.h)",
    ),
    Case(
        qd.field("metadata").h.min(),
        "min(metadata.h)",
    ),
    Case(
        qd.field("metadata").h.reverse(),
        "reverse(metadata.h)",
    ),
    Case(
        qd.field("metadata").h.sum(),
        "sum(metadata.h)",
    ),
    Case(
        qd.field("metadata").a.type_(),
        "type(metadata.a)",
    ),
    Case(
        qd.field("metadata").h.contains(3),
        "contains(metadata.h, `3`)",
    ),
    Case(
        qd.lit("hello").ends_with("lo"),
        'ends_with(`"hello"`, `"lo"`)',
    ),
    Case(
        qd.lit("hello").starts_with("he"),
        'starts_with(`"hello"`, `"he"`)',
    ),
    Case(
        qd.field("metadata").m.join(", "),
        "join(`, `, metadata.m)",
    ),
    Case(
        qd.merge(qd.field("metadata").d, qd.field("metadata").l),
        "merge(metadata.d, metadata.l)",
    ),
    Case(
        qd.not_null(qd.lit(None), qd.lit("a"), qd.lit("b")),
        'not_null(null, `"a"`, `"b"`)',
    ),
]
