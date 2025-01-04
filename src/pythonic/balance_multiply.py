class BalanceMultiply:
    def balancePair(self, numA, numB):
        accumulator = 0

        while True:
            if numA < numB:
                numA, numB = numB, numA

            if numA == 0 or numB == 0:
                return (numA, numB, accumulator)

            if numA % 2 == 1 and numB % 2 == 1:
                if numA // numB > 1:
                    accumulator += (numA - 1) + numB
                    numA = (numA - 1) >> 1
                    numB = (numB - 1) << 1
                    continue

            if numA % 2 == 0 and numB % 2 == 0:
                if numA // numB > 1:
                    numA = numA >> 1
                    numB = numB << 1
                    continue

            if numA % 2 == 1 and numB % 2 == 0:
                if numA // numB > 1:
                    accumulator += numB
                    numA = (numA - 1) >> 1
                    numB = numB << 1
                    continue

            if numA // numB > 1:
                accumulator += numA
                numA = numA >> 1
                numB = (numB - 1) << 1
                continue
            return (numA, numB, accumulator)

    def square(self, num):
        pow = 0
        res = 0
        tmp = num
        while (tmp):
            if (tmp & 1):
                res += (num << pow)
            pow += 1
            tmp = tmp >> 1
        return res

    def solve(self, numA, numB):
        if numA == 0 or numB == 0:
            return 0
        result = 0
        if numA < numB:
            numA, numB = numB, numA

        while numB > 0:
            quotient = numA // numB
            result += quotient * self.square(numB)
            numA, numB = numB, numA % numB
        return result

    def multiply(self, numA, numB):
        ans = self.balancePair(numA, numB)
        return self.solve(ans[0], ans[1]) + ans[2]
