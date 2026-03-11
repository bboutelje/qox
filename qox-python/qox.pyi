from enum import Enum
from typing import Optional, overload
from datetime import datetime, date
from typing import List

class Compounding(Enum):
    Simple = "Simple"
    Compounded = "Compounded"
    Continuous = "Continuous"
    SimpleThenCompounded = "SimpleThenCompounded"

class DayCountConvention(Enum):
    Actual360 = "Actual360"
    Actual365Fixed = "Actual365Fixed"
    ActActISDA = "ActActISDA"
    Thirty360US = "Thirty360US"

class Frequency:
    Annual: Frequency
    SemiAnnual: Frequency
    Quarterly: Frequency
    Monthly: Frequency
    Once: Frequency
    Infinite: Frequency

    # Constructor with optional value
    def __init__(self, value: Optional[int] = None) -> None: ...
    
    # Expose the getter
    @property
    def value(self) -> Optional[int]: ...
    
    # @staticmethod
    # def from_int(value: int) -> "Frequency": ...
    
    def __repr__(self) -> str: ...


class Tenor:
    """
    A representation of a time tenor for financial calculations.
    """

    @staticmethod
    def days(n: int) -> 'Tenor':
        """Creates a tenor of n days."""
        ...

    @staticmethod
    def weeks(n: int) -> 'Tenor':
        """Creates a tenor of n weeks."""
        ...

    @staticmethod
    def months(n: int) -> 'Tenor':
        """Creates a tenor of n months."""
        ...

    @staticmethod
    def years(n: int) -> 'Tenor':
        """Creates a tenor of n years."""
        ...

    def advance(self, from_date: date) -> date:
        """Returns the date advanced by the tenor from the provided date."""
        ...

class InterestRate:
    def __init__(
        self, 
        rate: float, 
        dcc: DayCountConvention, 
        compounding: Compounding, 
        frequency: Frequency
    ) -> None: ...

    # @property
    # def rate(self) -> float: ...
    
    def discount_factor(self, t: float) -> float: ...



class RateCurve:
    """
    A unified wrapper for various rate curve types.
    """
    
    @staticmethod
    def flat(rate: InterestRate) -> 'RateCurve':
        """Creates a flat rate curve."""
        ...

    @staticmethod
    def continuous(value: float) -> 'RateCurve':
        """Creates a continuous compounding rate curve."""
        ...

    @staticmethod
    def interpolated(
        reference_date: date, 
        tenors: List[Tenor], 
        rates: List[InterestRate]
    ) -> 'RateCurve':
        """Creates an interpolated rate curve."""
        ...

    def zero_rate(self, t: float) -> float:
        """Returns the zero rate for a given time t."""
        ...

    def discount_factor(self, t: float) -> float:
        """Returns the discount factor for a given time t."""
        ...



class VolSurface:
    """
    A unified wrapper for flat and interpolated volatility surfaces.
    """

    @staticmethod
    def flat(vol: float) -> 'VolSurface':
        """Creates a flat volatility surface with a constant value."""
        ...

    @staticmethod
    def interpolated(
        reference_date: date, 
        tenors: List[Tenor], 
        vols: List[float]
    ) -> 'VolSurface':
        """Creates an interpolated volatility surface from tenors and values."""
        ...

    def volatility(self, strike: float, t: float) -> float:
        """Returns the volatility for a given strike and time t."""
        ...


class OptionMarketFrame:
    """
    A market framework containing spot price, rate curve, and volatility surface.
    """

    def __init__(
        self, 
        spot_price: float, 
        rate_curve: RateCurve, 
        vol_surface: VolSurface
    ) -> None:
        """Initializes the market frame."""
        ...

    @property
    def spot_price(self) -> float:
        """Returns the current spot price."""
        ...

class StockOption:
    """
    A wrapper around the Rust-backed StockOption implementation.
    """
    def __init__(self, strike: float, expiry: datetime, option_type: str) -> None:
        ...

    @property
    def strike(self) -> float:
        ...

    @property
    def years_to_expiry(self) -> float:
        ...

    def evaluate(self, market_view: OptionMarketFrame) -> float:
        ...