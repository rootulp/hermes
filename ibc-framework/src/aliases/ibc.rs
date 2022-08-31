use crate::traits::ibc::IbcTypes;

pub type Height<Context> = <Context as IbcTypes>::Height;

pub type Timestamp<Context> = <Context as IbcTypes>::Timestamp;

pub type ClientId<Context> = <Context as IbcTypes>::ClientId;
