import fast_graphql
import asyncio
import graphene
import time


class Query(graphene.ObjectType):
    hello = graphene.String(description="A typical hello world")

    def resolve_hello(self, info):
        return "World"


iteration = 10000

start = time.time()
schema = graphene.Schema(query=Query)
query = """
    query {
      hello
    }
"""
for _ in range(iteration):
    result = schema.execute(query)
elapsed = time.time() - start
print(f"Execution time: {elapsed:.2f}s")


async def main():
    start = time.time()
    schema = fast_graphql.Schema()
    for _ in range(iteration):
        result = await schema.execute(query)
    elapsed = time.time() - start
    print(f"Execution time: {elapsed:.2f}s")


asyncio.run(main())
