qox is an early stage quantitative finance library built to mirror what QuantLib does. Initial benchmarking suggests it's at least 10x faster for finite difference methods when calculating option risk. The goal is to improve on QuantLib's functionality, flexibility and above all else, its speed.

Provided you don't have dividends, it can price an American option with the finite difference method.

The current roadmap is to implement the following:

- Discrete dividends
- Greeks
- Implied Vol
