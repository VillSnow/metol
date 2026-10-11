pub type Int = i64;
pub type Real = rust_decimal::Decimal;

pub enum ChannelConnectionTag {}
pub type ChannelConnection = crate::StringType<ChannelConnectionTag>;
