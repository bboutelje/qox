from enum import Enum
from typing import Optional, overload

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