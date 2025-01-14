import pytest
import fast_graphql


def test_sum_as_string():
    assert fast_graphql.sum_as_string(1, 1) == "2"
