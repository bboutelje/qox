qox is an early stage quantitative finance library built to mirror what QuantLib does in C++, but written in Rust. Initial benchmarking suggest it's about 13x faster to get the major Greeks for a European option using the finite difference method on a single thread. It should be at least 10x faster to calibrate volatility on a single thread.

The library is set up to use automated differentiation as a core feature and the idea is to do what QuantLib does with less than 2% of the energy when batching calculations. The current roadmap is to implement the following in the immediate future:

-Time stepping methods
-Discrete dividends
-American options
-Bootstrapping yield curves
