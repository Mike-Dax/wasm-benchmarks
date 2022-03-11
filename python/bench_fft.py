#!/usr/bin/env python3
import pyperf
import time
import numpy as np

numpy_array = np.array(np.random.random(4096) * 1000, dtype = 'float64')

python_list = list(numpy_array)

def find_min_np():
    return np.fft.fft(numpy_array)


runner = pyperf.Runner()
runner.bench_func('fft np', find_min_np)