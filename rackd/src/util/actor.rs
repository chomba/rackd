use std::future::Future;
use tokio::sync::oneshot;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

// pub fn spawn<A: Actor + Send + 'static>(actor: A) -> Handle<A::Message> {
//     let (tx, rx) = mpsc::channel::<A::Message>(10);
//     let handle = Handle { sender: tx };
//     tokio::task::spawn_blocking(A::run(actor, rx));
//     handle
// } 

/// Actor that synchronously receives and processes messages
pub trait Actor {
    type Message: Send + 'static;
    
    fn receive(&mut self, message: Self::Message);

    fn spawn<T: Actor + Send + 'static>(actor: T) -> Handle<T::Message> {
        let (tx, rx) = mpsc::channel::<T::Message>(10);
        let handle = Handle { sender: tx };
        tokio::task::spawn_blocking(T::run(actor, rx));
        handle
    }

    fn run<T: Actor + Send + 'static>(mut actor: T, mut receiver: mpsc::Receiver<T::Message>) -> impl FnOnce() -> () + Send + 'static {
        move || { 
            while let Some(message) = receiver.blocking_recv() {
                actor.receive(message);
            } 
        }
    }

    // fn run<M, T>(mut actor: T, mut receiver: mpsc::Receiver<M>) -> impl FnOnce() -> () where T: Actor<Message=M>, M: Send + 'static {
    //     move || { 
    //         while let Some(message) = receiver.blocking_recv() {
    //             actor.receive(message);
    //         } 
    //     }
    // }
}

/// Actor that asyncronously receives and processes messages
pub trait AsyncActor {
    type Message: Send + 'static;
    fn receive(&mut self, message: Self::Message) -> impl Future<Output=()>;

    async fn run<M, T>(actor: T, receiver: mpsc::Receiver<M>, cancel: CancellationToken) where T: AsyncActor<Message=M>, M: Send + 'static {
        tokio::select! {
            _ = cancel.cancelled() => {
                // Log termination
            }
            _ = work(actor, receiver) => {
                // Work Terminated
            }
        }
    
        async fn work<M, T>(mut actor: T, mut receiver: mpsc::Receiver<M>) where T: AsyncActor<Message=M>, M: Send + 'static {
            while let Some(message) = receiver.recv().await {
                actor.receive(message).await;
            }    
        }
    }
}

pub trait Payload {
    type Ok;
    type Err;
    // type Message = Msg<Payload>
}

/// Functional Core - Execute a command (should take a context) produce events and hydrate a view
pub trait Execute: Sized + Payload {
    type In;
    type Out;

    fn exec(self, input: Self::In) -> Result<Self::Out, Self::Err>;
}

pub struct Msg<P> where P: Payload {
    pub payload: P,
    pub respond_to: oneshot::Sender<Result<P::Ok, P::Err>>
}

pub trait Process: Payload {
    type Actor;
    fn process(self, actor: &mut Self::Actor) -> Result<Self::Ok, Self::Err>;
}

pub trait AsyncProcess: Payload {
    type Actor;
    fn process(self, actor: &mut Self::Actor) -> impl Future<Output=Result<Self::Ok, Self::Err>>;
}

pub struct Handle<M> {
    pub sender: mpsc::Sender<M>
}

impl<M> Handle<M>  {
    /// send: Sends Message and doesn't return until it has received a response back
    pub async fn send<P>(&self, payload: P) -> Result<P::Ok, P::Err> where P: Payload, M: From<Msg<P>>  {
        let (sender, receiver) = oneshot::channel::<Result<P::Ok, P::Err>>();
        let msg  = Msg { payload, respond_to: sender };
        let message: M = msg.into();
        let _ = self.sender.send(message).await;
        receiver.await.expect("Actor has been killed - send()")
    }

    // Don't call this method within async functions
    pub fn blocking_send<P>(&self, payload: P) -> Result<P::Ok, P::Err> where P: Payload, M: From<Msg<P>>  {
        let (sender, receiver) = oneshot::channel::<Result<P::Ok, P::Err>>();
        let msg  = Msg { payload, respond_to: sender };
        let message: M = msg.into();
        let _ = self.sender.blocking_send(message);
        receiver.blocking_recv().expect("Actor has been killed - blocking_send()")
    }

    /// emit: Sends Message and immediately returns (Fire and Forget)
    pub async fn emit<P>(&self, payload: P) -> () where P: Payload, M: From<Msg<P>> {
        let (sender, receiver) = oneshot::channel::<Result<P::Ok, P::Err>>();
        let msg  = Msg { payload, respond_to: sender };
        let message: M = msg.into();
        let _ = self.sender.send(message).await;
    } 

    pub fn blocking_emit<P>(&self, payload: P) -> () where P: Payload, M: From<Msg<P>> {
        let (sender, receiver) = oneshot::channel::<Result<P::Ok, P::Err>>();
        let msg  = Msg { payload, respond_to: sender };
        let message: M = msg.into();
        let _ = self.sender.blocking_send(message);
    } 
}

impl<M> Clone for Handle<M> {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone() }
    }
} 

// pub async fn run<M, T>(actor: T, receiver: mpsc::Receiver<M>, cancel: CancellationToken) where T: Actor<Message=M>, M: Send + 'static {
//     tokio::select! {
//         _ = cancel.cancelled() => {
//             // Log termination
//         }
//         _ = work(actor, receiver) => {
//             // Work Terminated
//         }
//     }

//     async fn work<M, T>(mut actor: T, mut receiver: mpsc::Receiver<M>) where T: Actor<Message=M>, M: Send + 'static {
//         while let Some(message) = receiver.recv().await {
//             actor.receive(message);
//         }    
//     }
// }




// pub struct Msg<P> where P: Payload {
//     pub payload: P,
//     pub respond_to: oneshot::Sender<Result<P::Ok, P::Err>>
// }

// pub struct Handle<M> {
//     pub sender: mpsc::Sender<M>,
//     pub token: CancellationToken
// }

// impl<P> Handle<Msg<P>> where P: Payload + Into<Msg<P>> {
//     // send: Sends Message and doesn't return until it has received a response back
//     pub async fn send(&self, payload: P) -> Result<P::Ok, P::Err> {
//         let (sender, receiver) = oneshot::channel::<Result<P::Ok, P::Err>>();
//         let msg  = Msg { payload, respond_to: sender };
//         let message = msg.into();
//         let _ = self.sender.send(message).await;
//         receiver.await.expect("Actor has been killed")
//     }

//     // emit: Sends Message and immediately returns (Fire and Forget)
//     pub async fn emit(&self, payload: P) -> () {
//         let message = payload.into();
//         let _ = self.sender.send(message).await;
//     } 
// }

// impl<P> Handle<sync::Msg<P>> where P: sync::Payload + Into<sync::Msg<P>> {
//     // send: Sends Message and doesn't return until it has received a response back
//     pub fn send(&self, payload: P) -> Result<P::Ok, P::Err> {
//         let (sender, receiver) = oneshot::channel::<Result<P::Ok, P::Err>>();
//         let msg  = Msg { payload, respond_to: sender };
//         let message = msg.into();
//         let _ = self.sender.blocking_send(message);
//         receiver.blocking_recv().expect("Actor has been killed")
//     }

//     // emit: Sends Message and immediately returns (Fire and Forget)
//     pub fn emit(&self, payload: P) -> () {
//         let message = payload.into();
//         let _ = self.sender.blocking_send(message);
//     } 
// }

// impl<M> Clone for Handle<M> {
//     fn clone(&self) -> Self {
//         Self { sender: self.sender.clone(), token: self.token.clone() }
//     }
// } 