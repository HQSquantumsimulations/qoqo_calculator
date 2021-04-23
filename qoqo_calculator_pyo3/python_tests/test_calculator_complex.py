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
from qoqo_calculator_pyo3 import CalculatorFloat, CalculatorComplex
import math

@pytest.mark.parametrize("start_value", [
    0,
    1.0,
    np.array([0])[0],
    1+1j,
    12j
])
def test_init(start_value):
    cf = CalculatorComplex(start_value)
    assert cf.real.value == np.real(start_value)
    assert cf.imag.value == np.imag(start_value)
    cf2 = CalculatorComplex(cf)
    assert cf2.real.value == np.real(start_value)
    assert cf2.imag.value == np.imag(start_value)

@pytest.mark.parametrize("start_value", [
    (0,0),
    (0,"a"),
    (np.array([0])[0],"b")
])
def test_from_pair(start_value):
    cf = CalculatorComplex.from_pair(*start_value)
    assert cf.real == CalculatorFloat(start_value[0])
    assert cf.imag == CalculatorFloat(start_value[1])

def test_str_init():
    cf = CalculatorComplex("start_value")
    assert cf.real.value == "start_value"
    assert cf.imag.value == 0
    cf2 = CalculatorComplex(cf)
    assert cf.real.value == "start_value"
    assert cf.imag.value == 0


def test_failed_init():
    with pytest.raises(TypeError):
        cf = CalculatorComplex(dict())


@pytest.mark.parametrize("init", [
    (1, 0, 0),
    ("a", 0, 0),
])
def test_div_fail(init):
    cf = CalculatorComplex(init[0])
    with pytest.raises(ZeroDivisionError):
        (cf / init[1])
    with pytest.raises(ZeroDivisionError):
        cf /= init[1]
    with pytest.raises(ZeroDivisionError):
        cf = CalculatorComplex(init[1])
        (init[0] / cf)

@pytest.mark.parametrize("init", [
    (2+1j, 1+4j, 3+5j),
])
def test_add(init):
    cf = CalculatorComplex(init[0])
    assert (cf + init[1]) == CalculatorComplex(init[2])
    cf += init[1]
    assert cf == CalculatorComplex(init[2])
    cf = CalculatorComplex(init[1])
    assert (init[0] + cf) == CalculatorComplex(init[2])


@pytest.mark.parametrize("init", [
    (2+1j, 1+4j, 1-3j),
])
def test_sub(init):
    cf = CalculatorComplex(init[0])
    assert (cf - init[1]) == CalculatorComplex(init[2])
    cf -= init[1]
    assert cf == CalculatorComplex(init[2])
    cf = CalculatorComplex(init[1])
    assert (init[0] - cf) == CalculatorComplex(init[2])


@pytest.mark.parametrize("init", [
    (2+1j, 1+4j, (2+1j)*(1+4j)),
])
def test_mult(init):
    cf = CalculatorComplex(init[0])
    assert (cf * init[1]) == CalculatorComplex(init[2])
    cf *= init[1]
    assert cf == CalculatorComplex(init[2])
    cf = CalculatorComplex(init[1])
    assert (init[0] * cf) == CalculatorComplex(init[2])


@pytest.mark.parametrize("init", [
    (2+1j, 1+4j, (2+1j)/(1+4j)),
])
def test_div(init):
    cf = CalculatorComplex(init[0])
    assert (cf / init[1]) == CalculatorComplex(init[2])
    cf /= init[1]
    assert cf == CalculatorComplex(init[2])
    cf = CalculatorComplex(init[1])
    assert (init[0] / cf) == CalculatorComplex(init[2])


@pytest.mark.parametrize("initial", [
    ((1, 0), (1, 0), True),
    ((0, 1), (0, 1), True),
    ((1, 0), (1, 1), False),
    ((1, 1), (0, 1), False),
    (('a', 'c'), ('a', 'c'), True),
    (('a', 'c'), ('a', 'a'), False),
    (('a', 'c'), ('c', 'c'), False),
    ((1+1e-9, 1), (1, 1-1e-9), True),
])
def test_complex_isclose(initial):
    t = CalculatorComplex.from_pair(initial[0][0], initial[0][1]).isclose(
        CalculatorComplex.from_pair(initial[1][0], initial[1][1])
    )
    assert t == initial[2]

@pytest.mark.parametrize("initial", [
    ((0, 1), (0, -1)),
    ((1, 0), (1, 0)),
    (('a', 'b'), ('a', '(-b)')),
])
def test_complex_conj(initial):
    t = CalculatorComplex.from_pair(*initial[0]).conj()
    assert t == CalculatorComplex.from_pair(*initial[1])

@pytest.mark.parametrize("initial", [
    ((1, 0), 0),
    ((0, 1), np.pi/2),
    ((-1, 0), np.pi),
    ((3, 3), np.pi/4),
    ((0, 0), 0),
    (('a', 'b'), 'atan2(b, a)'),
])
def test_complex_arg(initial):
    arg = CalculatorComplex.from_pair(*initial[0]).arg()
    assert arg.isclose(initial[1])

@pytest.mark.parametrize("initial", [
    ((1, 0), 1),
    ((0, 2), 2),
    ((-1, 0), 1),
    ((3, 3), np.sqrt(18)),
    ((0, 0), 0),
    (('a', 'b'), 'sqrt(((a * a) + (b * b)))'),
])
def test_complex_abs(initial):
    aabs = abs(CalculatorComplex.from_pair(*initial[0]))
    assert aabs.isclose(initial[1])

@pytest.mark.parametrize("initial", [
    (1+1j, 1+1j),
])
def test_complex_cast(initial):
    cc = CalculatorComplex(initial[0])
    assert complex(cc) == initial[1]

def test_complex_cast_fail():
    cc = CalculatorComplex.from_pair("a","b")
    with pytest.raises(ValueError):
        assert complex(cc)

if __name__ == '__main__':
    pytest.main(sys.argv)