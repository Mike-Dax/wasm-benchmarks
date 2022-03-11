#!/usr/bin/env python3
import pyperf
import time
import numpy as np

numpy_array = np.array(np.random.random(4096) * 1000, dtype = 'float64')

python_list = list(numpy_array)

def find_min_np():
    return np.amin(numpy_array)

def find_min_python():
    return min(python_list)

def find_min_python_for_loop():
    m = float(5000.0)
    
    for i in python_list:
        if i < m:
            m = i

runner = pyperf.Runner()
runner.bench_func('min np', find_min_np)
runner.bench_func('min py', find_min_python)
runner.bench_func('min pyf', find_min_python_for_loop)