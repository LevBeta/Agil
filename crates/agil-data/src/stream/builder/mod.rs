use crate::{error::AgilDataError, subscription::SubKind};

use std::{future::Future, pin::Pin};

//pub(crate) type SubscribeFuture = Pin<Box<dyn Future<Output = Result<(), AgilDataError>>>>;

//pub struct StreamBuilder {
//    pub futures: Vec<SubscribeFuture>,
//}

pub struct StreamBuilder {}
