import math
import numpy as np


class MutiplyPowerTwo:
    def __init__(self):
        self.hashMap = {0: 1, 1: 2}
        self.powers_of_2 = np.array([2 ** i for i in range(1001)])

    def findClosestPowerOfTwo(self, num):
        return math.floor(math.log2(num))

    def findPower(self, n):
        if n not in self.hashMap:
            self.hashMap[n] = self.powers_of_2[n]
        return self.hashMap[n]

    def multiply(self, numA, numB):
        if numA == 0 or numB == 0:
            return 0

        closest_power_numA = self.findClosestPowerOfTwo(numA)
        closest_power_numB = self.findClosestPowerOfTwo(numB)

        a = closest_power_numA
        c = closest_power_numB
        b = numA - self.findPower(closest_power_numA)
        d = numB - self.findPower(closest_power_numB)

        a_pow_x_plus_c = self.findPower(a + c)
        d_times_a_pow_x = d << closest_power_numA
        b_times_c_pow_y = b << closest_power_numB

        d_times_b = 0
        while d > 0:
            if d & 1:
                d_times_b += b
            b <<= 1
            d >>= 1

        return a_pow_x_plus_c + d_times_a_pow_x + b_times_c_pow_y + d_times_b
