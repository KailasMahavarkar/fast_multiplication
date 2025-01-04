class Naive:
    def multiply(self, numA, numB):
        # Convert numbers to strings to calculate length and split
        strA = str(numA)
        strB = str(numB)

        # Base case: if either number is single-digit
        if len(strA) == 1 or len(strB) == 1:
            return numA * numB

        n = max(len(strA), len(strB))
        m = n // 2

        strA = strA.zfill(n)
        strB = strB.zfill(n)

        highA, lowA = int(strA[:-m]), int(strA[-m:])
        highB, lowB = int(strB[:-m]), int(strB[-m:])

        # Multiply the parts (directly without any optimization)
        product_high = self.multiply(highA, highB)  # highA * highB
        product_low = self.multiply(lowA, lowB)    # lowA * lowB
        product_mixed1 = self.multiply(highA, lowB)  # highA * lowB
        product_mixed2 = self.multiply(lowA, highB)  # lowA * highB

        # Combine the results
        return (product_high * (10 ** (2 * m))) + ((product_mixed1 + product_mixed2) * (10 ** m)) + product_low
