import time
import random
from balance_multiply import BalanceMultiply
from naive_multiply import Naive
from karatsuba_multiply import Karatsuba
from power_of_two_multiply import MutiplyPowerTwo

import sys
sys.set_int_max_str_digits(10000)


def benchmark():
    def bulitin(a, b): return a * b
    naive = Naive()
    karatsuba = Karatsuba()
    power = MutiplyPowerTwo()
    balance = BalanceMultiply()

    methods = {
        "Python Built-in": bulitin,
        "Naive": naive.multiply,
        "Karatsuba": karatsuba.multiply,
        "PowerTwo": power.multiply,
        "Balance": balance.multiply
    }

    input_sizes = [x for x in range(1, 100)]
    runs = 50
    results = {method: [] for method in methods}

    for size in input_sizes:
        numA = 1024
        numB = random.randint(10**(size - 1), 10**size - 1)

        print(f"\nTesting with input size: {size} digits")
        for name, func in methods.items():
            total_time = 0
            for _ in range(runs):
                start_time = time.time_ns()
                func(numA, numB)  # Run the multiplication
                total_time += (time.time_ns() - start_time)

            avg_time = total_time / runs
            results[name].append(avg_time)
            print(f"{name} Avg Time: {avg_time / 1e9:.6f}s")

    return results


benchmark_results = benchmark()
