import fast_graphql
import asyncio

res = fast_graphql.concat("a", "b")
print(res)

res = asyncio.run(fast_graphql.concat("a", "b"))
print(res)
