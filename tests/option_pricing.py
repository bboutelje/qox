import qox
from datetime import datetime, timezone, timedelta

expiry = datetime.now(timezone.utc) + timedelta(days=365)
stock_option = qox.StockOption(100.0, expiry, "call")
market_frame = qox.OptionMarketFrame(95.0, qox.RateCurve.continuous(0.05), qox.VolSurface.flat(0.2))
price = stock_option.evaluate(market_frame)

print(f"Price: {price}")