use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{ready, Future, Stream};
use pin_project_lite::pin_project;

use crate::{InstanceInfo, MakepressManager};

pin_project! {
    #[must_use = "streams do nothing unless polled"]
    pub struct GetMapper<St, M>
    where St: Stream,
    M: 'static
    {
        #[pin]
        stream: St,
        manager: &'static M,
        #[pin]
        pending_fut: Option<Pin<Box<dyn Future<Output = crate::Result<InstanceInfo>>>>>,
    }
}

impl<St, M> GetMapper<St, M>
where
    St: Stream,
    M: 'static,
{
    pub(crate) fn new(manager: &'static M, stream: St) -> Self {
        Self {
            stream,
            manager,
            pending_fut: None,
        }
    }
}

impl<St, M> Stream for GetMapper<St, M>
where
    St: Stream,
    St::Item: AsRef<str> + Send + 'static,
    M: MakepressManager + 'static,
{
    type Item = crate::Result<InstanceInfo>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        Poll::Ready(loop {
            if let Some(fut) = this.pending_fut.as_mut().as_pin_mut() {
                let res = ready!(fut.poll(cx));
                this.pending_fut.set(None);
                break Some(res);
            } else if let Some(name) = ready!(this.stream.as_mut().poll_next(cx)) {
                this.pending_fut.set(Some(this.manager.get(name)));
            } else {
                break None;
            }
        })
    }
}

pin_project! {
    #[must_use = "streams do nothing unless polled"]
    pub struct CreateMapper<St, M>
    where St: Stream,
    M: 'static
    {
        #[pin]
        stream: St,
        manager: &'static M,
        #[pin]
        pending_fut: Option<Pin<Box<dyn Future<Output = crate::Result<InstanceInfo>>>>>,
    }
}

impl<St, M> CreateMapper<St, M>
where
    St: Stream,
    M: 'static,
{
    pub(crate) fn new(manager: &'static M, stream: St) -> Self {
        Self {
            stream,
            manager,
            pending_fut: None,
        }
    }
}

impl<St, M> Stream for CreateMapper<St, M>
where
    St: Stream,
    St::Item: AsRef<str> + Send + 'static,
    M: MakepressManager + 'static,
{
    type Item = crate::Result<InstanceInfo>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        Poll::Ready(loop {
            if let Some(fut) = this.pending_fut.as_mut().as_pin_mut() {
                let res = ready!(fut.poll(cx));
                this.pending_fut.set(None);
                break Some(res);
            } else if let Some(name) = ready!(this.stream.as_mut().poll_next(cx)) {
                this.pending_fut.set(Some(this.manager.create(name, crate::CreateInfo { host_type: crate::HostType::Managed })));
            } else {
                break None;
            }
        })
    }
}

pin_project! {
    #[must_use = "streams do nothing unless polled"]
    pub struct StartMapper<St, M>
    where
    St: Stream,
    M: 'static,
    {
        #[pin]
        stream: St,
        manager: &'static M,
        #[pin]
        pending_fut: Option<Pin<Box<dyn Future<Output = crate::Result<InstanceInfo>>>>>,
    }
}

impl<St, M> StartMapper<St, M>
where
    St: Stream,
    M: 'static,
{
    pub(crate) fn new(manager: &'static M, stream: St) -> Self {
        Self {
            stream,
            manager,
            pending_fut: None,
        }
    }
}

impl<St, M> Stream for StartMapper<St, M>
where
    St: Stream,
    St::Item: AsRef<str> + Send + 'static,
    M: MakepressManager + 'static,
{
    type Item = crate::Result<InstanceInfo>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        Poll::Ready(loop {
            if let Some(fut) = this.pending_fut.as_mut().as_pin_mut() {
                let res = ready!(fut.poll(cx));
                this.pending_fut.set(None);
                break Some(res);
            } else if let Some(name) = ready!(this.stream.as_mut().poll_next(cx)) {
                this.pending_fut.set(Some(this.manager.start(name)));
            } else {
                break None;
            }
        })
    }
}

pin_project! {
    #[must_use = "streams do nothing unless polled"]
    pub struct StopMapper<St, M>
    where
    St: Stream,
    M: 'static
    {
        #[pin]
        stream: St,
        manager: &'static M,
        #[pin]
        pending_fut: Option<Pin<Box<dyn Future<Output = crate::Result<InstanceInfo>>>>>,
    }
}

impl<St, M> StopMapper<St, M>
where
    St: Stream,
    M: 'static,
{
    pub(crate) fn new(manager: &'static M, stream: St) -> Self {
        Self {
            stream,
            manager,
            pending_fut: None,
        }
    }
}

impl<St, M> Stream for StopMapper<St, M>
where
    St: Stream,
    St::Item: AsRef<str> + Send + 'static,
    M: MakepressManager + 'static,
{
    type Item = crate::Result<InstanceInfo>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        Poll::Ready(loop {
            if let Some(fut) = this.pending_fut.as_mut().as_pin_mut() {
                let res = ready!(fut.poll(cx));
                this.pending_fut.set(None);
                break Some(res);
            } else if let Some(name) = ready!(this.stream.as_mut().poll_next(cx)) {
                this.pending_fut.set(Some(this.manager.stop(name)));
            } else {
                break None;
            }
        })
    }
}

pin_project! {
    #[must_use = "streams do nothing unless polled"]
    pub struct DestroyMapper<St, M>
    where St: Stream,
    M: 'static
    {
        #[pin]
        stream: St,
        manager: &'static M,
        #[pin]
        pending_fut: Option<Pin<Box<dyn Future<Output = crate::Result<()>>>>>,
    }
}

impl<St, M> DestroyMapper<St, M>
where
    St: Stream,
    M: 'static,
{
    pub(crate) fn new(manager: &'static M, stream: St) -> Self {
        Self {
            stream,
            manager,
            pending_fut: None,
        }
    }
}

impl<St, M> Stream for DestroyMapper<St, M>
where
    St: Stream,
    St::Item: AsRef<str> + Send + 'static,
    M: MakepressManager + 'static,
{
    type Item = crate::Result<()>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        Poll::Ready(loop {
            if let Some(fut) = this.pending_fut.as_mut().as_pin_mut() {
                let res = ready!(fut.poll(cx));
                this.pending_fut.set(None);
                break Some(res);
            } else if let Some(name) = ready!(this.stream.as_mut().poll_next(cx)) {
                this.pending_fut.set(Some(this.manager.destroy(name)));
            } else {
                break None;
            }
        })
    }
}
