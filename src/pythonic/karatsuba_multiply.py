class Karatsuba:
    def multiply(self, x, y):
        if x < 10 or y < 10:
            return x * y

        size = max(len(str(x)), len(str(y)))
        half_size = size // 2

        high_x = x // 10**half_size
        low_x = x % 10**half_size
        high_y = y // 10**half_size
        low_y = y % 10**half_size

        z0 = self.multiply(low_x, low_y)
        z1 = self.multiply(low_x + high_x, low_y + high_y)
        z2 = self.multiply(high_x, high_y)

        # Combine the results
        return z2 * 10**(2 * half_size) + (z1 - z2 - z0) * 10**half_size + z0
