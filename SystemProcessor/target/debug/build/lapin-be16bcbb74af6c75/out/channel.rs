pub mod options {
  use super::*;

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ChannelFlowOptions {
    pub active: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ChannelFlowOkOptions {
    pub active: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct AccessRequestOptions {
    pub exclusive: Boolean,
    pub passive: Boolean,
    pub active: Boolean,
    pub write: Boolean,
    pub read: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ExchangeDeclareOptions {
    pub passive: Boolean,
    pub durable: Boolean,
    pub auto_delete: Boolean,
    pub internal: Boolean,
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ExchangeDeleteOptions {
    pub if_unused: Boolean,
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ExchangeBindOptions {
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ExchangeUnbindOptions {
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct QueueDeclareOptions {
    pub passive: Boolean,
    pub durable: Boolean,
    pub exclusive: Boolean,
    pub auto_delete: Boolean,
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct QueueBindOptions {
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct QueuePurgeOptions {
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct QueueDeleteOptions {
    pub if_unused: Boolean,
    pub if_empty: Boolean,
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicQosOptions {
    pub global: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicConsumeOptions {
    pub no_local: Boolean,
    pub no_ack: Boolean,
    pub exclusive: Boolean,
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicCancelOptions {
    pub nowait: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicPublishOptions {
    pub mandatory: Boolean,
    pub immediate: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicDeliverOptions {
    pub redelivered: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicGetOptions {
    pub no_ack: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicGetOkOptions {
    pub redelivered: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicAckOptions {
    pub multiple: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicRejectOptions {
    pub requeue: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicRecoverAsyncOptions {
    pub requeue: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicRecoverOptions {
    pub requeue: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct BasicNackOptions {
    pub multiple: Boolean,
    pub requeue: Boolean,
    }

  #[derive(Clone, Debug, Default, PartialEq)]
  pub struct ConfirmSelectOptions {
    pub nowait: Boolean,
    }

  }

use options::*;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum Reply {
  ConnectionOpenOk(WaitHandle<()>),
  ConnectionCloseOk(WaitHandle<()>),
  ConnectionUpdateSecretOk(WaitHandle<()>),
  ChannelOpenOk(WaitHandle<Channel>),
  ChannelFlowOk(WaitHandle<Boolean>),
  ChannelCloseOk(WaitHandle<()>),
  AccessRequestOk(WaitHandle<()>),
  ExchangeDeclareOk(WaitHandle<()>),
  ExchangeDeleteOk(WaitHandle<()>),
  ExchangeBindOk(WaitHandle<()>),
  ExchangeUnbindOk(WaitHandle<()>),
  QueueDeclareOk(WaitHandle<Queue>),
  QueueBindOk(WaitHandle<()>),
  QueuePurgeOk(WaitHandle<LongUInt>),
  QueueDeleteOk(WaitHandle<LongUInt>, ShortString),
  QueueUnbindOk(WaitHandle<()>),
  BasicQosOk(WaitHandle<()>),
  BasicConsumeOk(WaitHandle<Consumer>, ShortString),
  BasicCancelOk(WaitHandle<()>),
  BasicGetOk(WaitHandle<Option<BasicGetMessage>>, ShortString),
  BasicRecoverOk(WaitHandle<()>),
  TxSelectOk(WaitHandle<()>),
  TxCommitOk(WaitHandle<()>),
  TxRollbackOk(WaitHandle<()>),
  ConfirmSelectOk(WaitHandle<()>),
  }

impl Channel {
  pub(crate) fn receive_method(&self, method: AMQPClass) -> Result<()> {
    match method {
      AMQPClass::Connection(protocol::connection::AMQPMethod::Start(m)) => self.receive_connection_start(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::Secure(m)) => self.receive_connection_secure(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::Tune(m)) => self.receive_connection_tune(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::OpenOk(m)) => self.receive_connection_open_ok(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::Close(m)) => self.receive_connection_close(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::CloseOk(m)) => self.receive_connection_close_ok(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::Blocked(m)) => self.receive_connection_blocked(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::Unblocked(m)) => self.receive_connection_unblocked(m),
      AMQPClass::Connection(protocol::connection::AMQPMethod::UpdateSecretOk(m)) => self.receive_connection_update_secret_ok(m),
      AMQPClass::Channel(protocol::channel::AMQPMethod::OpenOk(m)) => self.receive_channel_open_ok(m),
      AMQPClass::Channel(protocol::channel::AMQPMethod::Flow(m)) => self.receive_channel_flow(m),
      AMQPClass::Channel(protocol::channel::AMQPMethod::FlowOk(m)) => self.receive_channel_flow_ok(m),
      AMQPClass::Channel(protocol::channel::AMQPMethod::Close(m)) => self.receive_channel_close(m),
      AMQPClass::Channel(protocol::channel::AMQPMethod::CloseOk(m)) => self.receive_channel_close_ok(m),
      AMQPClass::Access(protocol::access::AMQPMethod::RequestOk(m)) => self.receive_access_request_ok(m),
      AMQPClass::Exchange(protocol::exchange::AMQPMethod::DeclareOk(m)) => self.receive_exchange_declare_ok(m),
      AMQPClass::Exchange(protocol::exchange::AMQPMethod::DeleteOk(m)) => self.receive_exchange_delete_ok(m),
      AMQPClass::Exchange(protocol::exchange::AMQPMethod::BindOk(m)) => self.receive_exchange_bind_ok(m),
      AMQPClass::Exchange(protocol::exchange::AMQPMethod::UnbindOk(m)) => self.receive_exchange_unbind_ok(m),
      AMQPClass::Queue(protocol::queue::AMQPMethod::DeclareOk(m)) => self.receive_queue_declare_ok(m),
      AMQPClass::Queue(protocol::queue::AMQPMethod::BindOk(m)) => self.receive_queue_bind_ok(m),
      AMQPClass::Queue(protocol::queue::AMQPMethod::PurgeOk(m)) => self.receive_queue_purge_ok(m),
      AMQPClass::Queue(protocol::queue::AMQPMethod::DeleteOk(m)) => self.receive_queue_delete_ok(m),
      AMQPClass::Queue(protocol::queue::AMQPMethod::UnbindOk(m)) => self.receive_queue_unbind_ok(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::QosOk(m)) => self.receive_basic_qos_ok(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::ConsumeOk(m)) => self.receive_basic_consume_ok(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::Cancel(m)) => self.receive_basic_cancel(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::CancelOk(m)) => self.receive_basic_cancel_ok(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::Return(m)) => self.receive_basic_return(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::Deliver(m)) => self.receive_basic_deliver(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::GetOk(m)) => self.receive_basic_get_ok(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::GetEmpty(m)) => self.receive_basic_get_empty(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::Ack(m)) => self.receive_basic_ack(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::RecoverOk(m)) => self.receive_basic_recover_ok(m),
      AMQPClass::Basic(protocol::basic::AMQPMethod::Nack(m)) => self.receive_basic_nack(m),
      AMQPClass::Tx(protocol::tx::AMQPMethod::SelectOk(m)) => self.receive_tx_select_ok(m),
      AMQPClass::Tx(protocol::tx::AMQPMethod::CommitOk(m)) => self.receive_tx_commit_ok(m),
      AMQPClass::Tx(protocol::tx::AMQPMethod::RollbackOk(m)) => self.receive_tx_rollback_ok(m),
      AMQPClass::Confirm(protocol::confirm::AMQPMethod::SelectOk(m)) => self.receive_confirm_select_ok(m),
      m => {
        error!("the client should not receive this method: {:?}", m);
        Err(Error::InvalidMethod(m))
      }
    }
  }

  
  fn receive_connection_start(&self, method: protocol::connection::Start) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_connection_start_received(method)
  }
  fn connection_start_ok(&self, client_properties: FieldTable, mechanism: &str, response: &str, locale: &str, wait_handle: WaitHandle<Connection>, credentials: Credentials) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::StartOk (protocol::connection::StartOk {
      client_properties: client_properties,
      mechanism: mechanism.into(),
      response: response.into(),
      locale: locale.into(),
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_connection_start_ok_sent(wait_handle, credentials);
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_connection_secure(&self, method: protocol::connection::Secure) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_connection_secure_received(method)
  }
  fn connection_secure_ok(&self, response: &str) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::SecureOk (protocol::connection::SecureOk {
      response: response.into(),
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_connection_tune(&self, method: protocol::connection::Tune) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_connection_tune_received(method)
  }
  fn connection_tune_ok(&self, channel_max: ShortUInt, frame_max: LongUInt, heartbeat: ShortUInt) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::TuneOk (protocol::connection::TuneOk {
      channel_max: channel_max,
      frame_max: frame_max,
      heartbeat: heartbeat,
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  pub (crate) fn connection_open(&self, virtual_host: &str, wait_handle: WaitHandle<Connection>) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::Open (protocol::connection::Open {
      virtual_host: virtual_host.into(),
      
      }));

    let (wait, _wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ConnectionOpenOk(_wait_handle.clone()), Box::new(_wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_connection_open_sent(wait_handle);
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_connection_open_ok(&self, method: protocol::connection::OpenOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ConnectionOpenOk(wait_handle)) => {
        let res = self.on_connection_open_ok_received(method);
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub (crate) fn connection_close(&self, reply_code: ShortUInt, reply_text: &str, class_id: ShortUInt, method_id: ShortUInt) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::Close (protocol::connection::Close {
      reply_code: reply_code,
      reply_text: reply_text.into(),
      class_id: class_id,
      method_id: method_id,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ConnectionCloseOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_connection_close_sent();
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  
  fn receive_connection_close(&self, method: protocol::connection::Close) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_connection_close_received(method)
  }
  fn connection_close_ok(&self) -> Confirmation<()> {
    
    if !self.status.is_closing() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::CloseOk (protocol::connection::CloseOk {
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_connection_close_ok_sent();
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  fn receive_connection_close_ok(&self, _: protocol::connection::CloseOk) -> Result<()> {
    
    if !self.status.is_closing() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ConnectionCloseOk(wait_handle)) => {
        
        let res = self.on_connection_close_ok_received();
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub (crate) fn connection_blocked(&self, reason: &str) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::Blocked (protocol::connection::Blocked {
      reason: reason.into(),
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_connection_blocked(&self, method: protocol::connection::Blocked) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_connection_blocked_received(method)
  }
  pub (crate) fn connection_unblocked(&self) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::Unblocked (protocol::connection::Unblocked {
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_connection_unblocked(&self, method: protocol::connection::Unblocked) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_connection_unblocked_received(method)
  }
  pub (crate) fn connection_update_secret(&self, new_secret: &str, reason: &str) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Connection(protocol::connection::AMQPMethod::UpdateSecret (protocol::connection::UpdateSecret {
      new_secret: new_secret.into(),
      reason: reason.into(),
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ConnectionUpdateSecretOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_connection_update_secret_ok(&self, _: protocol::connection::UpdateSecretOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ConnectionUpdateSecretOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub (crate) fn channel_open(&self) -> Confirmation<Channel> {
    if !self.status.is_initializing() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Channel(protocol::channel::AMQPMethod::Open (protocol::channel::Open {
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ChannelOpenOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_channel_open_ok(&self, method: protocol::channel::OpenOk) -> Result<()> {
    if !self.status.is_initializing() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ChannelOpenOk(wait_handle)) => {
        let res = self.on_channel_open_ok_received(method, wait_handle);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn channel_flow(&self, options: ChannelFlowOptions) -> Confirmation<Boolean> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ChannelFlowOptions {
      active,
      } = options;
    let method = AMQPClass::Channel(protocol::channel::AMQPMethod::Flow (protocol::channel::Flow {
      
      active,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ChannelFlowOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  
  fn receive_channel_flow(&self, method: protocol::channel::Flow) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_channel_flow_received(method)
  }
  fn channel_flow_ok(&self, options: ChannelFlowOkOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ChannelFlowOkOptions {
      active,
      } = options;
    let method = AMQPClass::Channel(protocol::channel::AMQPMethod::FlowOk (protocol::channel::FlowOk {
      
      active,
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  fn receive_channel_flow_ok(&self, method: protocol::channel::FlowOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ChannelFlowOk(wait_handle)) => {
        let res = self.on_channel_flow_ok_received(method, wait_handle);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  fn do_channel_close(&self, reply_code: ShortUInt, reply_text: &str, class_id: ShortUInt, method_id: ShortUInt) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Channel(protocol::channel::AMQPMethod::Close (protocol::channel::Close {
      reply_code: reply_code,
      reply_text: reply_text.into(),
      class_id: class_id,
      method_id: method_id,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ChannelCloseOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_channel_close_sent();
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  
  fn receive_channel_close(&self, method: protocol::channel::Close) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_channel_close_received(method)
  }
  fn channel_close_ok(&self) -> Confirmation<()> {
    
    if !self.status.is_closing() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Channel(protocol::channel::AMQPMethod::CloseOk (protocol::channel::CloseOk {
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_channel_close_ok_sent();
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  fn receive_channel_close_ok(&self, _: protocol::channel::CloseOk) -> Result<()> {
    
    if !self.status.is_closing() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ChannelCloseOk(wait_handle)) => {
        
        let res = self.on_channel_close_ok_received();
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn access_request(&self, realm: &str, options: AccessRequestOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let AccessRequestOptions {
      exclusive,
      passive,
      active,
      write,
      read,
      } = options;
    let method = AMQPClass::Access(protocol::access::AMQPMethod::Request (protocol::access::Request {
      realm: realm.into(),
      
      exclusive,
      passive,
      active,
      write,
      read,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::AccessRequestOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_access_request_ok(&self, method: protocol::access::RequestOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::AccessRequestOk(wait_handle)) => {
        let res = self.on_access_request_ok_received(method);
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  fn do_exchange_declare(&self, exchange: &str, kind: &str, options: ExchangeDeclareOptions, arguments: FieldTable) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ExchangeDeclareOptions {
      passive,
      durable,
      auto_delete,
      internal,
      nowait,
      } = options;
    let method = AMQPClass::Exchange(protocol::exchange::AMQPMethod::Declare (protocol::exchange::Declare {
      exchange: exchange.into(),
      kind: kind.into(),
      
      passive,
      durable,
      auto_delete,
      internal,
      nowait,
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ExchangeDeclareOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_exchange_declare_ok(&self, _: protocol::exchange::DeclareOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ExchangeDeclareOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn exchange_delete(&self, exchange: &str, options: ExchangeDeleteOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ExchangeDeleteOptions {
      if_unused,
      nowait,
      } = options;
    let method = AMQPClass::Exchange(protocol::exchange::AMQPMethod::Delete (protocol::exchange::Delete {
      exchange: exchange.into(),
      
      if_unused,
      nowait,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ExchangeDeleteOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_exchange_delete_ok(&self, _: protocol::exchange::DeleteOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ExchangeDeleteOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn exchange_bind(&self, destination: &str, source: &str, routing_key: &str, options: ExchangeBindOptions, arguments: FieldTable) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ExchangeBindOptions {
      nowait,
      } = options;
    let method = AMQPClass::Exchange(protocol::exchange::AMQPMethod::Bind (protocol::exchange::Bind {
      destination: destination.into(),
      source: source.into(),
      routing_key: routing_key.into(),
      
      nowait,
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ExchangeBindOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_exchange_bind_ok(&self, _: protocol::exchange::BindOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ExchangeBindOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn exchange_unbind(&self, destination: &str, source: &str, routing_key: &str, options: ExchangeUnbindOptions, arguments: FieldTable) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ExchangeUnbindOptions {
      nowait,
      } = options;
    let method = AMQPClass::Exchange(protocol::exchange::AMQPMethod::Unbind (protocol::exchange::Unbind {
      destination: destination.into(),
      source: source.into(),
      routing_key: routing_key.into(),
      
      nowait,
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ExchangeUnbindOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_exchange_unbind_ok(&self, _: protocol::exchange::UnbindOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ExchangeUnbindOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn queue_declare(&self, queue: &str, options: QueueDeclareOptions, arguments: FieldTable) -> Confirmation<Queue> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let QueueDeclareOptions {
      passive,
      durable,
      exclusive,
      auto_delete,
      nowait,
      } = options;
    let method = AMQPClass::Queue(protocol::queue::AMQPMethod::Declare (protocol::queue::Declare {
      queue: queue.into(),
      
      passive,
      durable,
      exclusive,
      auto_delete,
      nowait,
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::QueueDeclareOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      if let Err(err) = self.receive_queue_declare_ok(protocol::queue::DeclareOk { queue: queue.into(), ..Default::default()}) {
        return Confirmation::new_error(err);
      }
      }
    Confirmation::new(wait)
    }
  fn receive_queue_declare_ok(&self, method: protocol::queue::DeclareOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::QueueDeclareOk(wait_handle)) => {
        let res = self.on_queue_declare_ok_received(method, wait_handle);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn queue_bind(&self, queue: &str, exchange: &str, routing_key: &str, options: QueueBindOptions, arguments: FieldTable) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let QueueBindOptions {
      nowait,
      } = options;
    let method = AMQPClass::Queue(protocol::queue::AMQPMethod::Bind (protocol::queue::Bind {
      queue: queue.into(),
      exchange: exchange.into(),
      routing_key: routing_key.into(),
      
      nowait,
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::QueueBindOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_queue_bind_ok(&self, _: protocol::queue::BindOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::QueueBindOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn queue_purge(&self, queue: &str, options: QueuePurgeOptions) -> Confirmation<LongUInt> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let QueuePurgeOptions {
      nowait,
      } = options;
    let method = AMQPClass::Queue(protocol::queue::AMQPMethod::Purge (protocol::queue::Purge {
      queue: queue.into(),
      
      nowait,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::QueuePurgeOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_queue_purge_ok(&self, method: protocol::queue::PurgeOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::QueuePurgeOk(wait_handle)) => {
        let res = self.on_queue_purge_ok_received(method, wait_handle);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn queue_delete(&self, queue: &str, options: QueueDeleteOptions) -> Confirmation<LongUInt> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let QueueDeleteOptions {
      if_unused,
      if_empty,
      nowait,
      } = options;
    let method = AMQPClass::Queue(protocol::queue::AMQPMethod::Delete (protocol::queue::Delete {
      queue: queue.into(),
      
      if_unused,
      if_empty,
      nowait,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::QueueDeleteOk(wait_handle.clone(), queue.into()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      if let Err(err) = self.receive_queue_delete_ok(protocol::queue::DeleteOk { ..Default::default()}) {
        return Confirmation::new_error(err);
      }
      }
    Confirmation::new(wait)
    }
  fn receive_queue_delete_ok(&self, method: protocol::queue::DeleteOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::QueueDeleteOk(wait_handle, queue)) => {
        let res = self.on_queue_delete_ok_received(method, wait_handle, queue);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn queue_unbind(&self, queue: &str, exchange: &str, routing_key: &str, arguments: FieldTable) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Queue(protocol::queue::AMQPMethod::Unbind (protocol::queue::Unbind {
      queue: queue.into(),
      exchange: exchange.into(),
      routing_key: routing_key.into(),
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::QueueUnbindOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_queue_unbind_ok(&self, _: protocol::queue::UnbindOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::QueueUnbindOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn basic_qos(&self, prefetch_count: ShortUInt, options: BasicQosOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicQosOptions {
      global,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Qos (protocol::basic::Qos {
      prefetch_count: prefetch_count,
      
      global,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::BasicQosOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_basic_qos_ok(&self, _: protocol::basic::QosOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::BasicQosOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  fn do_basic_consume(&self, queue: &str, consumer_tag: &str, options: BasicConsumeOptions, arguments: FieldTable) -> Confirmation<Consumer> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicConsumeOptions {
      no_local,
      no_ack,
      exclusive,
      nowait,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Consume (protocol::basic::Consume {
      queue: queue.into(),
      consumer_tag: consumer_tag.into(),
      
      no_local,
      no_ack,
      exclusive,
      nowait,
      arguments: arguments,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::BasicConsumeOk(wait_handle.clone(), queue.into()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      if let Err(err) = self.receive_basic_consume_ok(protocol::basic::ConsumeOk { consumer_tag: consumer_tag.into(), }) {
        return Confirmation::new_error(err);
      }
      }
    Confirmation::new(wait)
    }
  fn receive_basic_consume_ok(&self, method: protocol::basic::ConsumeOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::BasicConsumeOk(wait_handle, queue)) => {
        let res = self.on_basic_consume_ok_received(method, wait_handle, queue);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn basic_cancel(&self, consumer_tag: &str, options: BasicCancelOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicCancelOptions {
      nowait,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Cancel (protocol::basic::Cancel {
      consumer_tag: consumer_tag.into(),
      
      nowait,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::BasicCancelOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      if let Err(err) = self.receive_basic_cancel_ok(protocol::basic::CancelOk { consumer_tag: consumer_tag.into(), }) {
        return Confirmation::new_error(err);
      }
      }
    Confirmation::new(wait)
    }
  
  fn receive_basic_cancel(&self, method: protocol::basic::Cancel) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_basic_cancel_received(method)
  }
  fn basic_cancel_ok(&self, consumer_tag: &str) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::CancelOk (protocol::basic::CancelOk {
      consumer_tag: consumer_tag.into(),
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  fn receive_basic_cancel_ok(&self, method: protocol::basic::CancelOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::BasicCancelOk(wait_handle)) => {
        let res = self.on_basic_cancel_ok_received(method);
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn basic_publish(&self, exchange: &str, routing_key: &str, options: BasicPublishOptions, payload: Vec<u8>, properties: BasicProperties) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    self.before_basic_publish();
    let BasicPublishOptions {
      mandatory,
      immediate,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Publish (protocol::basic::Publish {
      exchange: exchange.into(),
      routing_key: routing_key.into(),
      
      mandatory,
      immediate,
      }));

    let send_res = self.send_method_frame_with_body(method, payload, properties);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_basic_return(&self, method: protocol::basic::Return) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_basic_return_received(method)
  }
  
  fn receive_basic_deliver(&self, method: protocol::basic::Deliver) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_basic_deliver_received(method)
  }
  pub fn basic_get(&self, queue: &str, options: BasicGetOptions) -> Confirmation<Option<BasicGetMessage>> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicGetOptions {
      no_ack,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Get (protocol::basic::Get {
      queue: queue.into(),
      
      no_ack,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::BasicGetOk(wait_handle.clone(), queue.into()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_basic_get_ok(&self, method: protocol::basic::GetOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::BasicGetOk(wait_handle, queue)) => {
        let res = self.on_basic_get_ok_received(method, wait_handle, queue);
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  
  fn receive_basic_get_empty(&self, method: protocol::basic::GetEmpty) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_basic_get_empty_received(method)
  }
  pub fn basic_ack(&self, delivery_tag: LongLongUInt, options: BasicAckOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicAckOptions {
      multiple,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Ack (protocol::basic::Ack {
      delivery_tag: delivery_tag,
      
      multiple,
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_basic_ack_sent(multiple, delivery_tag);
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_basic_ack(&self, method: protocol::basic::Ack) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_basic_ack_received(method)
  }
  pub fn basic_reject(&self, delivery_tag: LongLongUInt, options: BasicRejectOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicRejectOptions {
      requeue,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Reject (protocol::basic::Reject {
      delivery_tag: delivery_tag,
      
      requeue,
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  pub fn basic_recover_async(&self, options: BasicRecoverAsyncOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicRecoverAsyncOptions {
      requeue,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::RecoverAsync (protocol::basic::RecoverAsync {
      
      requeue,
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_basic_recover_async_sent();
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  pub fn basic_recover(&self, options: BasicRecoverOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicRecoverOptions {
      requeue,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Recover (protocol::basic::Recover {
      
      requeue,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::BasicRecoverOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_basic_recover_ok(&self, _: protocol::basic::RecoverOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::BasicRecoverOk(wait_handle)) => {
        
        let res = self.on_basic_recover_ok_received();
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn basic_nack(&self, delivery_tag: LongLongUInt, options: BasicNackOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let BasicNackOptions {
      multiple,
      requeue,
      } = options;
    let method = AMQPClass::Basic(protocol::basic::AMQPMethod::Nack (protocol::basic::Nack {
      delivery_tag: delivery_tag,
      
      multiple,
      requeue,
      }));

    
    let send_res = self.send_method_frame(method, None);
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    let end_hook_res = self.on_basic_nack_sent(multiple, delivery_tag);
    if let Err(err) = end_hook_res {
      return Confirmation::new_error(err);
    }
    
    Confirmation::new(send_res.unwrap())
    }
  
  fn receive_basic_nack(&self, method: protocol::basic::Nack) -> Result<()> {
    if !self.status.is_connected() {
      return Err(Error::NotConnected);
    }
    self.on_basic_nack_received(method)
  }
  pub fn tx_select(&self) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Tx(protocol::tx::AMQPMethod::Select (protocol::tx::Select {
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::TxSelectOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_tx_select_ok(&self, _: protocol::tx::SelectOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::TxSelectOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn tx_commit(&self) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Tx(protocol::tx::AMQPMethod::Commit (protocol::tx::Commit {
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::TxCommitOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_tx_commit_ok(&self, _: protocol::tx::CommitOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::TxCommitOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn tx_rollback(&self) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let method = AMQPClass::Tx(protocol::tx::AMQPMethod::Rollback (protocol::tx::Rollback {
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::TxRollbackOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    Confirmation::new(wait)
    }
  fn receive_tx_rollback_ok(&self, _: protocol::tx::RollbackOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::TxRollbackOk(wait_handle)) => {
        
        
        let res = Ok(());
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  pub fn confirm_select(&self, options: ConfirmSelectOptions) -> Confirmation<()> {
    
    
    if !self.status.is_connected() {
    return Confirmation::new_error(Error::NotConnected);
    }

    let ConfirmSelectOptions {
      nowait,
      } = options;
    let method = AMQPClass::Confirm(protocol::confirm::AMQPMethod::Select (protocol::confirm::Select {
      
      nowait,
      }));

    let (wait, wait_handle) = Wait::new();
    
    let send_res = self.send_method_frame(method, Some((Reply::ConfirmSelectOk(wait_handle.clone()), Box::new(wait_handle))));
    if let Err(err) = send_res {
      return Confirmation::new_error(err);
    }
    if nowait {
      }
    Confirmation::new(wait)
    }
  fn receive_confirm_select_ok(&self, _: protocol::confirm::SelectOk) -> Result<()> {
    
    
    if !self.status.is_connected() {
    return Err(Error::NotConnected);
    }

    match self.connection.next_expected_reply(self.id) {
      Some(Reply::ConfirmSelectOk(wait_handle)) => {
        
        let res = self.on_confirm_select_ok_received();
        wait_handle.finish(());
        res
      },
      _ => {
        self.set_error()?;
        Err(Error::UnexpectedReply)
      },
    }
  }
  }

