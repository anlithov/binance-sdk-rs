use crate::create_enum_with_fmt;

create_enum_with_fmt!(WebsocketStreamURL, {
  Base1 => "wss://stream.binance.com:9443",
  Base2 => "wss://stream.binance.com:443"
});
