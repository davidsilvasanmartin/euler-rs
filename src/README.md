## `digits_sum`

Generally, the most efficient way of summing the digits
of a number is to use mathematical operations (modulo
and division) rather than converting to a string. String
conversion often involves heap allocations and parsing,
which are slow operations.
