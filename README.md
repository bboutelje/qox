qox is an early stage quantitative finance library built to mirror what QuantLib does in C++, but written in Rust. Initial benchmarking suggests it's about 13x faster to get the major Greeks for a European option using the finite difference method on a single thread. It should be at least 10x faster to calibrate volatility. The library is set up to use automated differentiation as a core feature and is intended for institutional grade risk analytics. The goal is to match QuantLib's functionality with less than 2% of the energy cost during batching, while providing more power and superior ergonomics.

The current roadmap is to implement the following in the immediate future:

- Time-stepping methods
- Discrete dividends
- American options
- Bootstrapping yield curves
- Volatility surface construction
