use crate::create_enum_with_fmt;

create_enum_with_fmt!(OrderType, {
  Limit => "LIMIT",
  Market => "MARKET",
  StopLoss => "STOP_LOSS",
  StopLossLimit =>"STOP_LOSS_LIMIT",
  TakeProfit => "TAKE_PROFIT",
  TakeProfitLimit => "TAKE_PROFIT_LIMIT",
  LimitMaker =>"LIMIT_MAKER"
});

create_enum_with_fmt!(OrderSide, {
  Buy => "BUY",
  Sell => "SELL"
});

create_enum_with_fmt!(TimeInForce, {
  GTC => "GTC",
  IOC => "IOC",
  FOK => "FOK",
});
