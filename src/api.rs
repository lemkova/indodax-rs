#![allow(clippy::all)]
pub enum API {
    Private(Private),
}

pub enum Private {
    GetInfo,
    TransactionHistory,
    CreateOrder,
    TradeHistory,
    OpenOrders,
    OrderHistory,
    GetOrder,
    GetOrderByClientOrderId,
    CancelOrder,
    CancelOrderByClientOrderId,
}

impl From<API> for String {
    fn from(item: API) -> Self {
        String::from(match item {
            API::Private(method) => match method {
                Private::GetInfo => "getInfo",
                Private::TransactionHistory => "transHistory",
                Private::CreateOrder => "trade",
                Private::TradeHistory => "tradeHistory",
                Private::OpenOrders => "openOrders",
                Private::OrderHistory => "orderHistory",
                Private::GetOrder => "getOrder",
                Private::GetOrderByClientOrderId => "getOrderByClientOrderId",
                Private::CancelOrder => "cancelOrder",
                Private::CancelOrderByClientOrderId => "cancelOrderByClientOrderId",
            }
        })
    }
}

pub trait Indodax {
    fn new(api_key: Option<String>, secret_key: Option<String>, host: String) -> Self;
}