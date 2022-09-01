use crate::core::traits::ibc::HasIbcTypes;

pub type Height<Context> = <Context as HasIbcTypes>::Height;

pub type Timestamp<Context> = <Context as HasIbcTypes>::Timestamp;

pub type ClientId<Context> = <Context as HasIbcTypes>::ClientId;
