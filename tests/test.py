import qox

rate = qox.InterestRate(0.05, qox.DayCountConvention.Actual360, qox.Compounding.Compounded, qox.Frequency.Annual)
print(rate.discount_factor(1))