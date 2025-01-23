# Copyright © 2019-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
# in compliance with the License. You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software distributed under the License
# is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express
# or implied. See the License for the specific language governing permissions and limitations under
# the License.
import pytest
import sys
import numpy as np
import numpy.testing as npt
import os
from qoqo_calculator_pyo3 import CalculatorFloat
import math


def test_number():
    c = CalculatorFloat(1)
    c = CalculatorFloat(0.1)


def test_string():
    c = CalculatorFloat("test")


def test_cf():
    c = CalculatorFloat("test")
    c2 = CalculatorFloat(c)
    c = CalculatorFloat(1)
    c2 = CalculatorFloat(c)


@pytest.mark.parametrize("start_value", [0, 1.0, np.array([0])[0]])
def test_init(start_value):
    cf = CalculatorFloat(start_value)
    assert cf.value == start_value
    assert cf.is_float
    cf2 = CalculatorFloat(cf)
    assert cf.value == start_value
    assert cf.is_float


def test_str_init():
    cf = CalculatorFloat("start_value")
    assert cf.value == "start_value"
    assert not cf.is_float
    cf2 = CalculatorFloat(cf)
    assert cf.value == "start_value"
    assert not cf.is_float


def test_failed_init():
    with pytest.raises(TypeError):
        cf = CalculatorFloat(dict())


@pytest.mark.parametrize(
    "init",
    [
        (0, 1, 1),
        (0, "a", "a"),
        (1, "a", "(1e0 + a)"),
        (2, "a", "(2e0 + a)"),
        ("a", 0, "a"),
        ("a", 1, "(a + 1e0)"),
        ("a", 2, "(a + 2e0)"),
    ],
)
def test_add(init):
    print("test")
    cf = CalculatorFloat(init[0])
    assert (cf + init[1]) == CalculatorFloat(init[2])
    cf += init[1]
    assert cf == CalculatorFloat(init[2])
    cf = CalculatorFloat(init[1])
    assert (init[0] + cf) == CalculatorFloat(init[2])


@pytest.mark.parametrize(
    "init",
    [
        (0, 1, -1),
        (0, "a", "(-a)"),
        (1, "a", "(1e0 - a)"),
        (2, "a", "(2e0 - a)"),
        ("a", 0, "a"),
        ("a", 1, "(a - 1e0)"),
        ("a", 2, "(a - 2e0)"),
    ],
)
def test_sub(init):
    cf = CalculatorFloat(init[0])
    assert (cf - init[1]) == CalculatorFloat(init[2])
    cf -= init[1]
    assert cf == CalculatorFloat(init[2])
    cf = CalculatorFloat(init[1])
    assert (init[0] - cf) == CalculatorFloat(init[2])


@pytest.mark.parametrize(
    "init",
    [
        (0, 1, 0),
        (2, 1, 2),
        (0, "a", 0),
        (1, "a", "a"),
        (2, "a", "(2e0 * a)"),
        ("a", 0, 0),
        ("a", 1, "a"),
        ("a", 2, "(a * 2e0)"),
    ],
)
def test_mult(init):
    cf = CalculatorFloat(init[0])
    assert (cf * init[1]) == CalculatorFloat(init[2])
    cf *= init[1]
    assert cf == CalculatorFloat(init[2])
    cf = CalculatorFloat(init[1])
    assert (init[0] * cf) == CalculatorFloat(init[2])


@pytest.mark.parametrize(
    "init",
    [
        (0, 1, 0),
        (2, 1, 2),
        (0, "a", 0),
        (1, "a", "(1e0 / a)"),
        (2, "a", "(2e0 / a)"),
        ("a", 1, "a"),
        ("a", 2, "(a / 2e0)"),
    ],
)
def test_div(init):
    cf = CalculatorFloat(init[0])
    assert (cf / init[1]) == CalculatorFloat(init[2])
    cf /= init[1]
    assert cf == CalculatorFloat(init[2])
    cf = CalculatorFloat(init[1])
    assert (init[0] / cf) == CalculatorFloat(init[2])


@pytest.mark.parametrize(
    "init",
    [
        (1, 0, 0),
        ("a", 0, 0),
    ],
)
def test_div_fail(init):
    cf = CalculatorFloat(init[0])
    with pytest.raises(ZeroDivisionError):
        (cf / init[1])
    with pytest.raises(ZeroDivisionError):
        cf /= init[1]
    with pytest.raises(ZeroDivisionError):
        cf = CalculatorFloat(init[1])
        (init[0] / cf)


@pytest.mark.parametrize(
    "initial",
    [
        (1, 1),
        (-1, 1),
        (
            "a",
            "abs(a)",
        ),
    ],
)
def test_float_abs(initial):
    t = abs(CalculatorFloat(initial[0]))
    assert t.isclose(initial[1])


@pytest.mark.parametrize(
    "initial",
    [
        (np.pi / 2, 1),
        (-np.pi / 2, -1),
        (2, 1),
        (-2, -1),
        (0, 1),
        ("a", "sign(a)"),
    ],
)
def test_float_sign(initial):
    t = CalculatorFloat(initial[0]).sign()
    assert t.isclose(initial[1])


@pytest.mark.parametrize(
    "initial",
    [
        (np.pi / 2, 1),
        (-np.pi / 2, -1),
        (0, 0),
        (
            "a",
            "sin(a)",
        ),
    ],
)
def test_float_sin(initial):
    t = CalculatorFloat(initial[0]).sin()
    assert t.isclose(initial[1])


@pytest.mark.parametrize(
    "initial",
    [
        (1, np.arccos(1)),
        (-1, np.arccos(-1)),
        (0, np.arccos(0)),
        (
            "a",
            "acos(a)",
        ),
    ],
)
def test_float_acos(initial):
    t = CalculatorFloat(initial[0]).acos()
    assert t.isclose(initial[1])


@pytest.mark.parametrize(
    "initial",
    [
        (1, 1, True),
        ("a", "a", True),
        (1, "1", True),
        (1, 2, False),
        (1, "2", False),
        (1, 1 + 1e-9, True),
        (1, 1 + 1e-4, False),
    ],
)
def test_float_isclose(initial):
    t = CalculatorFloat(initial[0]).isclose(initial[1])
    assert t == initial[2]


@pytest.mark.parametrize(
    "initial",
    [
        (1, 1),
    ],
)
def test_float_cast(initial):
    cc = CalculatorFloat(initial[0])
    assert float(cc) == initial[1]


def test_float_cast_fail():
    cc = CalculatorFloat("a")
    with pytest.raises(ValueError):
        assert float(cc)


if __name__ == "__main__":
    pytest.main(sys.argv)
