qox is an early stage quantitative finance library built to mirror what QuantLib does. Initial benchmarking suggests it's at least 10x faster for finite difference methods when calculating the Greeks. The goal is to improve on QuantLib's functionality and flexibility while also opening the door to faster calculations. In theory you could price 1,000 options in the same time Quantlib takes to price 1.


The current roadmap is to implement the following in the immediate future:

- Time-stepping methods
- Discrete dividends
- American options
- Volatility surface mechanics
