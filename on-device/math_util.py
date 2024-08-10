import uarray
import math

def average(numbers):
    return sum(numbers) / len(numbers)

# Calculate standard deviation
def standard_deviation(numbers):
    avg = average(numbers)
    variance = sum((x - avg) ** 2 for x in numbers) / len(numbers)
    return math.sqrt(variance)


class RunningStats:
    def __init__(self):
        self.mean = 0.0
        self.sq_mean = 0.0
        self.min = float("inf")
        self.max = -float("inf")
        self.count = 0

    def update(self, value):
        self.count += 1
        delta = value - self.mean
        self.mean += delta / self.count
        self.sq_mean += delta * (value - self.mean)
        self.min = min(self.min, value)
        self.max = max(self.max, value)

    def get_mean(self):
        return self.mean

    def get_stddev(self):
        if self.count < 2:
            return 0.0
        variance = self.sq_mean / self.count
        return variance ** 0.5



