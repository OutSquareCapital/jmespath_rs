import jmespath_rs as jr

data = {"users": [{"age": 17, "name": "a"}, {"age": 20, "name": "b"}]}
print(jr.compile_expr("users[?age >= `18`].name | [0]").search(data))  # -> "b"

expr = jr.compile_expr("users[*].age")
print(expr.search(data))  # -> [17, 20]
