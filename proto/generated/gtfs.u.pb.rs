const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__FeedMessage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FeedMessage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FeedMessage>
}

impl ::protobuf::Message for FeedMessage {
  type MessageView<'msg> = FeedMessageView<'msg>;
  type MessageMut<'msg> = FeedMessageMut<'msg>;
}

impl ::std::default::Default for FeedMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FeedMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FeedMessage` is `Sync` because it does not implement interior mutability.
//    Neither does `FeedMessageMut`.
unsafe impl ::std::marker::Sync for FeedMessage {}

// SAFETY:
// - `FeedMessage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FeedMessage {}

impl ::protobuf::Proxied for FeedMessage {
  type View<'msg> = FeedMessageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FeedMessage {}

impl ::protobuf::MutProxied for FeedMessage {
  type Mut<'msg> = FeedMessageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FeedMessageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FeedMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeedMessageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FeedMessageView<'msg> {
  type Message = FeedMessage;
}

impl ::std::fmt::Debug for FeedMessageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FeedMessageView<'_> {
  fn default() -> FeedMessageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FeedMessage>> for FeedMessageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FeedMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FeedMessageView<'msg> {

  pub fn to_owned(&self) -> FeedMessage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // header: optional message transit_realtime.FeedHeader
  pub fn has_header(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn header_opt(self) -> ::std::option::Option<super::FeedHeaderView<'msg>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(self) -> super::FeedHeaderView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FeedHeaderView::default())
  }

  // entity: repeated message transit_realtime.FeedEntity
  pub fn entity(self) -> ::protobuf::RepeatedView<'msg, super::FeedEntity> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FeedEntity>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `FeedMessageView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FeedMessageView<'_> {}

// SAFETY:
// - `FeedMessageView` is `Send` because while its alive a `FeedMessageMut` cannot.
// - `FeedMessageView` does not use thread-local data.
unsafe impl ::std::marker::Send for FeedMessageView<'_> {}

impl<'msg> ::protobuf::AsView for FeedMessageView<'msg> {
  type Proxied = FeedMessage;
  fn as_view(&self) -> ::protobuf::View<'msg, FeedMessage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeedMessageView<'msg> {
  fn into_view<'shorter>(self) -> FeedMessageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FeedMessage> for FeedMessageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FeedMessage {
    let mut dst = FeedMessage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FeedMessage> for FeedMessageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FeedMessage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FeedMessage {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FeedMessageView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FeedMessageMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeedMessageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedMessage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeedMessageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FeedMessageMut<'msg> {
  type Message = FeedMessage;
}

impl ::std::fmt::Debug for FeedMessageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FeedMessage>> for FeedMessageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedMessage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FeedMessageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedMessage> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FeedMessage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // header: optional message transit_realtime.FeedHeader
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<super::FeedHeaderView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> super::FeedHeaderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FeedHeaderView::default())
  }
  pub fn header_mut(&mut self) -> super::FeedHeaderMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::FeedHeader>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // entity: repeated message transit_realtime.FeedEntity
  pub fn entity(&self) -> ::protobuf::RepeatedView<'_, super::FeedEntity> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FeedEntity>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entity_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FeedEntity> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_entity(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FeedEntity>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `FeedMessageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FeedMessageMut<'_> {}

// SAFETY:
// - `FeedMessageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FeedMessageMut<'_> {}

impl<'msg> ::protobuf::AsView for FeedMessageMut<'msg> {
  type Proxied = FeedMessage;
  fn as_view(&self) -> ::protobuf::View<'_, FeedMessage> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeedMessageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FeedMessage>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FeedMessageMut<'msg> {
  type MutProxied = FeedMessage;
  fn as_mut(&mut self) -> FeedMessageMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FeedMessageMut<'msg> {
  fn into_mut<'shorter>(self) -> FeedMessageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FeedMessage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FeedMessage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FeedMessageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FeedMessageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // header: optional message transit_realtime.FeedHeader
  pub fn has_header(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_header(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn header_opt(&self) -> ::std::option::Option<super::FeedHeaderView<'_>> {
    self.has_header().then(|| self.header())
  }
  pub fn header(&self) -> super::FeedHeaderView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::FeedHeaderView::default())
  }
  pub fn header_mut(&mut self) -> super::FeedHeaderMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_header(&mut self,
    val: impl ::protobuf::IntoProxied<super::FeedHeader>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // entity: repeated message transit_realtime.FeedEntity
  pub fn entity(&self) -> ::protobuf::RepeatedView<'_, super::FeedEntity> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::FeedEntity>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn entity_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::FeedEntity> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_entity(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::FeedEntity>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl FeedMessage

impl ::std::ops::Drop for FeedMessage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FeedMessage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FeedMessage {
  type Proxied = Self;
  fn as_view(&self) -> FeedMessageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FeedMessage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FeedMessageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeedMessage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__FeedMessage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P3NG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__FeedMessage_msg_init.0, &[<super::FeedHeader as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::FeedEntity as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__FeedMessage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeedMessage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeedMessage {
  type Msg = FeedMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedMessage {
  type Msg = FeedMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeedMessageMut<'_> {
  type Msg = FeedMessage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedMessage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedMessageMut<'_> {
  type Msg = FeedMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedMessage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedMessageView<'_> {
  type Msg = FeedMessage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedMessage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeedMessageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__FeedHeader_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FeedHeader {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FeedHeader>
}

impl ::protobuf::Message for FeedHeader {
  type MessageView<'msg> = FeedHeaderView<'msg>;
  type MessageMut<'msg> = FeedHeaderMut<'msg>;
}

impl ::std::default::Default for FeedHeader {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FeedHeader {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FeedHeader` is `Sync` because it does not implement interior mutability.
//    Neither does `FeedHeaderMut`.
unsafe impl ::std::marker::Sync for FeedHeader {}

// SAFETY:
// - `FeedHeader` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FeedHeader {}

impl ::protobuf::Proxied for FeedHeader {
  type View<'msg> = FeedHeaderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FeedHeader {}

impl ::protobuf::MutProxied for FeedHeader {
  type Mut<'msg> = FeedHeaderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FeedHeaderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FeedHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeedHeaderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FeedHeaderView<'msg> {
  type Message = FeedHeader;
}

impl ::std::fmt::Debug for FeedHeaderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FeedHeaderView<'_> {
  fn default() -> FeedHeaderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FeedHeader>> for FeedHeaderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FeedHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FeedHeaderView<'msg> {

  pub fn to_owned(&self) -> FeedHeader {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // gtfs_realtime_version: optional string
  pub fn has_gtfs_realtime_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn gtfs_realtime_version_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_gtfs_realtime_version().then(|| self.gtfs_realtime_version())
  }
  pub fn gtfs_realtime_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // incrementality: optional enum transit_realtime.FeedHeader.Incrementality
  pub fn has_incrementality(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn incrementality_opt(self) -> ::std::option::Option<super::feed_header::Incrementality> {
    self.has_incrementality().then(|| self.incrementality())
  }
  pub fn incrementality(self) -> super::feed_header::Incrementality {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::feed_header::Incrementality::FullDataset).into()
      ).try_into().unwrap()
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn timestamp_opt(self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // feed_version: optional string
  pub fn has_feed_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn feed_version_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_feed_version().then(|| self.feed_version())
  }
  pub fn feed_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `FeedHeaderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FeedHeaderView<'_> {}

// SAFETY:
// - `FeedHeaderView` is `Send` because while its alive a `FeedHeaderMut` cannot.
// - `FeedHeaderView` does not use thread-local data.
unsafe impl ::std::marker::Send for FeedHeaderView<'_> {}

impl<'msg> ::protobuf::AsView for FeedHeaderView<'msg> {
  type Proxied = FeedHeader;
  fn as_view(&self) -> ::protobuf::View<'msg, FeedHeader> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeedHeaderView<'msg> {
  fn into_view<'shorter>(self) -> FeedHeaderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FeedHeader> for FeedHeaderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FeedHeader {
    let mut dst = FeedHeader::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FeedHeader> for FeedHeaderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FeedHeader {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FeedHeader {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FeedHeaderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FeedHeaderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeedHeaderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeedHeaderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FeedHeaderMut<'msg> {
  type Message = FeedHeader;
}

impl ::std::fmt::Debug for FeedHeaderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FeedHeader>> for FeedHeaderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FeedHeaderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedHeader> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FeedHeader {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // gtfs_realtime_version: optional string
  pub fn has_gtfs_realtime_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_gtfs_realtime_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn gtfs_realtime_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_gtfs_realtime_version().then(|| self.gtfs_realtime_version())
  }
  pub fn gtfs_realtime_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_gtfs_realtime_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // incrementality: optional enum transit_realtime.FeedHeader.Incrementality
  pub fn has_incrementality(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_incrementality(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn incrementality_opt(&self) -> ::std::option::Option<super::feed_header::Incrementality> {
    self.has_incrementality().then(|| self.incrementality())
  }
  pub fn incrementality(&self) -> super::feed_header::Incrementality {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::feed_header::Incrementality::FullDataset).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_incrementality(&mut self, val: super::feed_header::Incrementality) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // feed_version: optional string
  pub fn has_feed_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_feed_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn feed_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_feed_version().then(|| self.feed_version())
  }
  pub fn feed_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_feed_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `FeedHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FeedHeaderMut<'_> {}

// SAFETY:
// - `FeedHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FeedHeaderMut<'_> {}

impl<'msg> ::protobuf::AsView for FeedHeaderMut<'msg> {
  type Proxied = FeedHeader;
  fn as_view(&self) -> ::protobuf::View<'_, FeedHeader> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeedHeaderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FeedHeader>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FeedHeaderMut<'msg> {
  type MutProxied = FeedHeader;
  fn as_mut(&mut self) -> FeedHeaderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FeedHeaderMut<'msg> {
  fn into_mut<'shorter>(self) -> FeedHeaderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FeedHeader {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FeedHeader> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FeedHeaderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FeedHeaderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // gtfs_realtime_version: optional string
  pub fn has_gtfs_realtime_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_gtfs_realtime_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn gtfs_realtime_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_gtfs_realtime_version().then(|| self.gtfs_realtime_version())
  }
  pub fn gtfs_realtime_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_gtfs_realtime_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // incrementality: optional enum transit_realtime.FeedHeader.Incrementality
  pub fn has_incrementality(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_incrementality(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn incrementality_opt(&self) -> ::std::option::Option<super::feed_header::Incrementality> {
    self.has_incrementality().then(|| self.incrementality())
  }
  pub fn incrementality(&self) -> super::feed_header::Incrementality {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        1, (super::feed_header::Incrementality::FullDataset).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_incrementality(&mut self, val: super::feed_header::Incrementality) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        1, val.into()
      )
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        2, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        2, val.into()
      )
    }
  }

  // feed_version: optional string
  pub fn has_feed_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_feed_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn feed_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_feed_version().then(|| self.feed_version())
  }
  pub fn feed_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_feed_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl FeedHeader

impl ::std::ops::Drop for FeedHeader {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FeedHeader {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FeedHeader {
  type Proxied = Self;
  fn as_view(&self) -> FeedHeaderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FeedHeader {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FeedHeaderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeedHeader {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__FeedHeader_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1N4,1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__FeedHeader_msg_init.0, &[], &[<super::feed_header::Incrementality as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__FeedHeader_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeedHeader {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeedHeader {
  type Msg = FeedHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedHeader {
  type Msg = FeedHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeedHeaderMut<'_> {
  type Msg = FeedHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedHeaderMut<'_> {
  type Msg = FeedHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedHeaderView<'_> {
  type Msg = FeedHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedHeader> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeedHeaderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod feed_header {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Incrementality(i32);

#[allow(non_upper_case_globals)]
impl Incrementality {
  pub const FullDataset: Incrementality = Incrementality(0);
  pub const Differential: Incrementality = Incrementality(1);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "FullDataset",
      1 => "Differential",
      _ => return None
    })
  }
}

impl ::std::convert::From<Incrementality> for i32 {
  fn from(val: Incrementality) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for Incrementality {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<Incrementality, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for Incrementality {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for Incrementality {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Incrementality::{}", constant_name)
    } else {
      write!(f, "Incrementality::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Incrementality {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Incrementality {}

impl ::protobuf::Proxied for Incrementality {
  type View<'a> = Incrementality;
}

impl ::protobuf::AsView for Incrementality {
  type Proxied = Incrementality;

  fn as_view(&self) -> Incrementality {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Incrementality {
  fn into_view<'shorter>(self) -> Incrementality where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Incrementality {
  const NAME: &'static str = "Incrementality";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1)
  }
}

impl ::protobuf::__internal::EntityType for Incrementality {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for Incrementality {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!$"))
    }).0
  }
}

}  // pub mod feed_header


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__FeedEntity_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct FeedEntity {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<FeedEntity>
}

impl ::protobuf::Message for FeedEntity {
  type MessageView<'msg> = FeedEntityView<'msg>;
  type MessageMut<'msg> = FeedEntityMut<'msg>;
}

impl ::std::default::Default for FeedEntity {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for FeedEntity {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `FeedEntity` is `Sync` because it does not implement interior mutability.
//    Neither does `FeedEntityMut`.
unsafe impl ::std::marker::Sync for FeedEntity {}

// SAFETY:
// - `FeedEntity` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for FeedEntity {}

impl ::protobuf::Proxied for FeedEntity {
  type View<'msg> = FeedEntityView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for FeedEntity {}

impl ::protobuf::MutProxied for FeedEntity {
  type Mut<'msg> = FeedEntityMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct FeedEntityView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FeedEntity>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeedEntityView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for FeedEntityView<'msg> {
  type Message = FeedEntity;
}

impl ::std::fmt::Debug for FeedEntityView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for FeedEntityView<'_> {
  fn default() -> FeedEntityView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, FeedEntity>> for FeedEntityView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, FeedEntity>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FeedEntityView<'msg> {

  pub fn to_owned(&self) -> FeedEntity {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn has_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // is_deleted: optional bool
  pub fn has_is_deleted(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn is_deleted_opt(self) -> ::std::option::Option<bool> {
    self.has_is_deleted().then(|| self.is_deleted())
  }
  pub fn is_deleted(self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }

  // trip_update: optional message transit_realtime.TripUpdate
  pub fn has_trip_update(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn trip_update_opt(self) -> ::std::option::Option<super::TripUpdateView<'msg>> {
    self.has_trip_update().then(|| self.trip_update())
  }
  pub fn trip_update(self) -> super::TripUpdateView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripUpdateView::default())
  }

  // vehicle: optional message transit_realtime.VehiclePosition
  pub fn has_vehicle(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn vehicle_opt(self) -> ::std::option::Option<super::VehiclePositionView<'msg>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(self) -> super::VehiclePositionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehiclePositionView::default())
  }

  // alert: optional message transit_realtime.Alert
  pub fn has_alert(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn alert_opt(self) -> ::std::option::Option<super::AlertView<'msg>> {
    self.has_alert().then(|| self.alert())
  }
  pub fn alert(self) -> super::AlertView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AlertView::default())
  }

  // shape: optional message transit_realtime.Shape
  pub fn has_shape(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn shape_opt(self) -> ::std::option::Option<super::ShapeView<'msg>> {
    self.has_shape().then(|| self.shape())
  }
  pub fn shape(self) -> super::ShapeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ShapeView::default())
  }

  // stop: optional message transit_realtime.Stop
  pub fn has_stop(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn stop_opt(self) -> ::std::option::Option<super::StopView<'msg>> {
    self.has_stop().then(|| self.stop())
  }
  pub fn stop(self) -> super::StopView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StopView::default())
  }

  // trip_modifications: optional message transit_realtime.TripModifications
  pub fn has_trip_modifications(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn trip_modifications_opt(self) -> ::std::option::Option<super::TripModificationsView<'msg>> {
    self.has_trip_modifications().then(|| self.trip_modifications())
  }
  pub fn trip_modifications(self) -> super::TripModificationsView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripModificationsView::default())
  }

}

// SAFETY:
// - `FeedEntityView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for FeedEntityView<'_> {}

// SAFETY:
// - `FeedEntityView` is `Send` because while its alive a `FeedEntityMut` cannot.
// - `FeedEntityView` does not use thread-local data.
unsafe impl ::std::marker::Send for FeedEntityView<'_> {}

impl<'msg> ::protobuf::AsView for FeedEntityView<'msg> {
  type Proxied = FeedEntity;
  fn as_view(&self) -> ::protobuf::View<'msg, FeedEntity> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeedEntityView<'msg> {
  fn into_view<'shorter>(self) -> FeedEntityView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<FeedEntity> for FeedEntityView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FeedEntity {
    let mut dst = FeedEntity::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<FeedEntity> for FeedEntityMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> FeedEntity {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for FeedEntity {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FeedEntityView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for FeedEntityMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct FeedEntityMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedEntity>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for FeedEntityMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for FeedEntityMut<'msg> {
  type Message = FeedEntity;
}

impl ::std::fmt::Debug for FeedEntityMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, FeedEntity>> for FeedEntityMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedEntity>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> FeedEntityMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, FeedEntity> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> FeedEntity {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // is_deleted: optional bool
  pub fn has_is_deleted(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_is_deleted(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn is_deleted_opt(&self) -> ::std::option::Option<bool> {
    self.has_is_deleted().then(|| self.is_deleted())
  }
  pub fn is_deleted(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_deleted(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // trip_update: optional message transit_realtime.TripUpdate
  pub fn has_trip_update(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_trip_update(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn trip_update_opt(&self) -> ::std::option::Option<super::TripUpdateView<'_>> {
    self.has_trip_update().then(|| self.trip_update())
  }
  pub fn trip_update(&self) -> super::TripUpdateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripUpdateView::default())
  }
  pub fn trip_update_mut(&mut self) -> super::TripUpdateMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip_update(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripUpdate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // vehicle: optional message transit_realtime.VehiclePosition
  pub fn has_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn vehicle_opt(&self) -> ::std::option::Option<super::VehiclePositionView<'_>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(&self) -> super::VehiclePositionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehiclePositionView::default())
  }
  pub fn vehicle_mut(&mut self) -> super::VehiclePositionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::VehiclePosition>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // alert: optional message transit_realtime.Alert
  pub fn has_alert(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_alert(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn alert_opt(&self) -> ::std::option::Option<super::AlertView<'_>> {
    self.has_alert().then(|| self.alert())
  }
  pub fn alert(&self) -> super::AlertView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AlertView::default())
  }
  pub fn alert_mut(&mut self) -> super::AlertMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_alert(&mut self,
    val: impl ::protobuf::IntoProxied<super::Alert>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // shape: optional message transit_realtime.Shape
  pub fn has_shape(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_shape(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn shape_opt(&self) -> ::std::option::Option<super::ShapeView<'_>> {
    self.has_shape().then(|| self.shape())
  }
  pub fn shape(&self) -> super::ShapeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ShapeView::default())
  }
  pub fn shape_mut(&mut self) -> super::ShapeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_shape(&mut self,
    val: impl ::protobuf::IntoProxied<super::Shape>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // stop: optional message transit_realtime.Stop
  pub fn has_stop(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stop(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stop_opt(&self) -> ::std::option::Option<super::StopView<'_>> {
    self.has_stop().then(|| self.stop())
  }
  pub fn stop(&self) -> super::StopView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StopView::default())
  }
  pub fn stop_mut(&mut self) -> super::StopMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop(&mut self,
    val: impl ::protobuf::IntoProxied<super::Stop>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // trip_modifications: optional message transit_realtime.TripModifications
  pub fn has_trip_modifications(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_trip_modifications(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn trip_modifications_opt(&self) -> ::std::option::Option<super::TripModificationsView<'_>> {
    self.has_trip_modifications().then(|| self.trip_modifications())
  }
  pub fn trip_modifications(&self) -> super::TripModificationsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripModificationsView::default())
  }
  pub fn trip_modifications_mut(&mut self) -> super::TripModificationsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip_modifications(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripModifications>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}

// SAFETY:
// - `FeedEntityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for FeedEntityMut<'_> {}

// SAFETY:
// - `FeedEntityMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for FeedEntityMut<'_> {}

impl<'msg> ::protobuf::AsView for FeedEntityMut<'msg> {
  type Proxied = FeedEntity;
  fn as_view(&self) -> ::protobuf::View<'_, FeedEntity> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for FeedEntityMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, FeedEntity>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for FeedEntityMut<'msg> {
  type MutProxied = FeedEntity;
  fn as_mut(&mut self) -> FeedEntityMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for FeedEntityMut<'msg> {
  fn into_mut<'shorter>(self) -> FeedEntityMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl FeedEntity {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, FeedEntity> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> FeedEntityView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> FeedEntityMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // is_deleted: optional bool
  pub fn has_is_deleted(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_is_deleted(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn is_deleted_opt(&self) -> ::std::option::Option<bool> {
    self.has_is_deleted().then(|| self.is_deleted())
  }
  pub fn is_deleted(&self) -> bool {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_bool_at_index(
        1, (false).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_is_deleted(&mut self, val: bool) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_bool_at_index(
        1, val.into()
      )
    }
  }

  // trip_update: optional message transit_realtime.TripUpdate
  pub fn has_trip_update(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_trip_update(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn trip_update_opt(&self) -> ::std::option::Option<super::TripUpdateView<'_>> {
    self.has_trip_update().then(|| self.trip_update())
  }
  pub fn trip_update(&self) -> super::TripUpdateView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripUpdateView::default())
  }
  pub fn trip_update_mut(&mut self) -> super::TripUpdateMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip_update(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripUpdate>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // vehicle: optional message transit_realtime.VehiclePosition
  pub fn has_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn vehicle_opt(&self) -> ::std::option::Option<super::VehiclePositionView<'_>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(&self) -> super::VehiclePositionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehiclePositionView::default())
  }
  pub fn vehicle_mut(&mut self) -> super::VehiclePositionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::VehiclePosition>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // alert: optional message transit_realtime.Alert
  pub fn has_alert(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_alert(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn alert_opt(&self) -> ::std::option::Option<super::AlertView<'_>> {
    self.has_alert().then(|| self.alert())
  }
  pub fn alert(&self) -> super::AlertView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::AlertView::default())
  }
  pub fn alert_mut(&mut self) -> super::AlertMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_alert(&mut self,
    val: impl ::protobuf::IntoProxied<super::Alert>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // shape: optional message transit_realtime.Shape
  pub fn has_shape(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_shape(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn shape_opt(&self) -> ::std::option::Option<super::ShapeView<'_>> {
    self.has_shape().then(|| self.shape())
  }
  pub fn shape(&self) -> super::ShapeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::ShapeView::default())
  }
  pub fn shape_mut(&mut self) -> super::ShapeMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_shape(&mut self,
    val: impl ::protobuf::IntoProxied<super::Shape>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // stop: optional message transit_realtime.Stop
  pub fn has_stop(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stop(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stop_opt(&self) -> ::std::option::Option<super::StopView<'_>> {
    self.has_stop().then(|| self.stop())
  }
  pub fn stop(&self) -> super::StopView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::StopView::default())
  }
  pub fn stop_mut(&mut self) -> super::StopMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop(&mut self,
    val: impl ::protobuf::IntoProxied<super::Stop>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // trip_modifications: optional message transit_realtime.TripModifications
  pub fn has_trip_modifications(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_trip_modifications(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn trip_modifications_opt(&self) -> ::std::option::Option<super::TripModificationsView<'_>> {
    self.has_trip_modifications().then(|| self.trip_modifications())
  }
  pub fn trip_modifications(&self) -> super::TripModificationsView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripModificationsView::default())
  }
  pub fn trip_modifications_mut(&mut self) -> super::TripModificationsMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip_modifications(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripModifications>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

}  // impl FeedEntity

impl ::std::ops::Drop for FeedEntity {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for FeedEntity {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for FeedEntity {
  type Proxied = Self;
  fn as_view(&self) -> FeedEntityView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for FeedEntity {
  type MutProxied = Self;
  fn as_mut(&mut self) -> FeedEntityMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for FeedEntity {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__FeedEntity_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1N/333333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__FeedEntity_msg_init.0, &[<super::TripUpdate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::VehiclePosition as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Alert as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Shape as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Stop as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TripModifications as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__FeedEntity_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeedEntity {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeedEntity {
  type Msg = FeedEntity;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedEntity> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedEntity {
  type Msg = FeedEntity;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedEntity> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for FeedEntityMut<'_> {
  type Msg = FeedEntity;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedEntity> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedEntityMut<'_> {
  type Msg = FeedEntity;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedEntity> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for FeedEntityView<'_> {
  type Msg = FeedEntity;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<FeedEntity> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for FeedEntityMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripUpdate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TripUpdate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TripUpdate>
}

impl ::protobuf::Message for TripUpdate {
  type MessageView<'msg> = TripUpdateView<'msg>;
  type MessageMut<'msg> = TripUpdateMut<'msg>;
}

impl ::std::default::Default for TripUpdate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TripUpdate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TripUpdate` is `Sync` because it does not implement interior mutability.
//    Neither does `TripUpdateMut`.
unsafe impl ::std::marker::Sync for TripUpdate {}

// SAFETY:
// - `TripUpdate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TripUpdate {}

impl ::protobuf::Proxied for TripUpdate {
  type View<'msg> = TripUpdateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TripUpdate {}

impl ::protobuf::MutProxied for TripUpdate {
  type Mut<'msg> = TripUpdateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TripUpdateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripUpdateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TripUpdateView<'msg> {
  type Message = TripUpdate;
}

impl ::std::fmt::Debug for TripUpdateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TripUpdateView<'_> {
  fn default() -> TripUpdateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TripUpdate>> for TripUpdateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripUpdateView<'msg> {

  pub fn to_owned(&self) -> TripUpdate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn trip_opt(self) -> ::std::option::Option<super::TripDescriptorView<'msg>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(self) -> super::TripDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }

  // vehicle: optional message transit_realtime.VehicleDescriptor
  pub fn has_vehicle(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn vehicle_opt(self) -> ::std::option::Option<super::VehicleDescriptorView<'msg>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(self) -> super::VehicleDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleDescriptorView::default())
  }

  // stop_time_update: repeated message transit_realtime.TripUpdate.StopTimeUpdate
  pub fn stop_time_update(self) -> ::protobuf::RepeatedView<'msg, super::trip_update::StopTimeUpdate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_update::StopTimeUpdate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // timestamp: optional uint64
  pub fn has_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn timestamp_opt(self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // delay: optional int32
  pub fn has_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn delay_opt(self) -> ::std::option::Option<i32> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // trip_properties: optional message transit_realtime.TripUpdate.TripProperties
  pub fn has_trip_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn trip_properties_opt(self) -> ::std::option::Option<super::trip_update::TripPropertiesView<'msg>> {
    self.has_trip_properties().then(|| self.trip_properties())
  }
  pub fn trip_properties(self) -> super::trip_update::TripPropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::trip_update::TripPropertiesView::default())
  }

}

// SAFETY:
// - `TripUpdateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TripUpdateView<'_> {}

// SAFETY:
// - `TripUpdateView` is `Send` because while its alive a `TripUpdateMut` cannot.
// - `TripUpdateView` does not use thread-local data.
unsafe impl ::std::marker::Send for TripUpdateView<'_> {}

impl<'msg> ::protobuf::AsView for TripUpdateView<'msg> {
  type Proxied = TripUpdate;
  fn as_view(&self) -> ::protobuf::View<'msg, TripUpdate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripUpdateView<'msg> {
  fn into_view<'shorter>(self) -> TripUpdateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TripUpdate> for TripUpdateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripUpdate {
    let mut dst = TripUpdate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TripUpdate> for TripUpdateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripUpdate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TripUpdate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripUpdateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripUpdateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TripUpdateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripUpdateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TripUpdateMut<'msg> {
  type Message = TripUpdate;
}

impl ::std::fmt::Debug for TripUpdateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TripUpdate>> for TripUpdateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripUpdateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TripUpdate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TripUpdate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_opt(&self) -> ::std::option::Option<super::TripDescriptorView<'_>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(&self) -> super::TripDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }
  pub fn trip_mut(&mut self) -> super::TripDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // vehicle: optional message transit_realtime.VehicleDescriptor
  pub fn has_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn vehicle_opt(&self) -> ::std::option::Option<super::VehicleDescriptorView<'_>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(&self) -> super::VehicleDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleDescriptorView::default())
  }
  pub fn vehicle_mut(&mut self) -> super::VehicleDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::VehicleDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // stop_time_update: repeated message transit_realtime.TripUpdate.StopTimeUpdate
  pub fn stop_time_update(&self) -> ::protobuf::RepeatedView<'_, super::trip_update::StopTimeUpdate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_update::StopTimeUpdate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn stop_time_update_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::trip_update::StopTimeUpdate> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_stop_time_update(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::trip_update::StopTimeUpdate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // delay: optional int32
  pub fn has_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn delay_opt(&self) -> ::std::option::Option<i32> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delay(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // trip_properties: optional message transit_realtime.TripUpdate.TripProperties
  pub fn has_trip_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_trip_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn trip_properties_opt(&self) -> ::std::option::Option<super::trip_update::TripPropertiesView<'_>> {
    self.has_trip_properties().then(|| self.trip_properties())
  }
  pub fn trip_properties(&self) -> super::trip_update::TripPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::trip_update::TripPropertiesView::default())
  }
  pub fn trip_properties_mut(&mut self) -> super::trip_update::TripPropertiesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::trip_update::TripProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}

// SAFETY:
// - `TripUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TripUpdateMut<'_> {}

// SAFETY:
// - `TripUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TripUpdateMut<'_> {}

impl<'msg> ::protobuf::AsView for TripUpdateMut<'msg> {
  type Proxied = TripUpdate;
  fn as_view(&self) -> ::protobuf::View<'_, TripUpdate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripUpdateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TripUpdate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TripUpdateMut<'msg> {
  type MutProxied = TripUpdate;
  fn as_mut(&mut self) -> TripUpdateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TripUpdateMut<'msg> {
  fn into_mut<'shorter>(self) -> TripUpdateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TripUpdate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TripUpdate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TripUpdateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TripUpdateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_opt(&self) -> ::std::option::Option<super::TripDescriptorView<'_>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(&self) -> super::TripDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }
  pub fn trip_mut(&mut self) -> super::TripDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // vehicle: optional message transit_realtime.VehicleDescriptor
  pub fn has_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn vehicle_opt(&self) -> ::std::option::Option<super::VehicleDescriptorView<'_>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(&self) -> super::VehicleDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleDescriptorView::default())
  }
  pub fn vehicle_mut(&mut self) -> super::VehicleDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::VehicleDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // stop_time_update: repeated message transit_realtime.TripUpdate.StopTimeUpdate
  pub fn stop_time_update(&self) -> ::protobuf::RepeatedView<'_, super::trip_update::StopTimeUpdate> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_update::StopTimeUpdate>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn stop_time_update_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::trip_update::StopTimeUpdate> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_stop_time_update(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::trip_update::StopTimeUpdate>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        3, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        3, val.into()
      )
    }
  }

  // delay: optional int32
  pub fn has_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn delay_opt(&self) -> ::std::option::Option<i32> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delay(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // trip_properties: optional message transit_realtime.TripUpdate.TripProperties
  pub fn has_trip_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_trip_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn trip_properties_opt(&self) -> ::std::option::Option<super::trip_update::TripPropertiesView<'_>> {
    self.has_trip_properties().then(|| self.trip_properties())
  }
  pub fn trip_properties(&self) -> super::trip_update::TripPropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::trip_update::TripPropertiesView::default())
  }
  pub fn trip_properties_mut(&mut self) -> super::trip_update::TripPropertiesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::trip_update::TripProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl TripUpdate

impl ::std::ops::Drop for TripUpdate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TripUpdate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TripUpdate {
  type Proxied = Self;
  fn as_view(&self) -> TripUpdateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TripUpdate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TripUpdateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TripUpdate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TripUpdate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P3NG3,(3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TripUpdate_msg_init.0, &[<super::TripDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::trip_update::StopTimeUpdate as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::VehicleDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::trip_update::TripProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TripUpdate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripUpdate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripUpdate {
  type Msg = TripUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripUpdate {
  type Msg = TripUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripUpdateMut<'_> {
  type Msg = TripUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripUpdateMut<'_> {
  type Msg = TripUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripUpdateView<'_> {
  type Msg = TripUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripUpdate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripUpdateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod trip_update {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripUpdate__StopTimeEvent_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StopTimeEvent {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StopTimeEvent>
}

impl ::protobuf::Message for StopTimeEvent {
  type MessageView<'msg> = StopTimeEventView<'msg>;
  type MessageMut<'msg> = StopTimeEventMut<'msg>;
}

impl ::std::default::Default for StopTimeEvent {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StopTimeEvent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StopTimeEvent` is `Sync` because it does not implement interior mutability.
//    Neither does `StopTimeEventMut`.
unsafe impl ::std::marker::Sync for StopTimeEvent {}

// SAFETY:
// - `StopTimeEvent` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StopTimeEvent {}

impl ::protobuf::Proxied for StopTimeEvent {
  type View<'msg> = StopTimeEventView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StopTimeEvent {}

impl ::protobuf::MutProxied for StopTimeEvent {
  type Mut<'msg> = StopTimeEventMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StopTimeEventView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeEvent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopTimeEventView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StopTimeEventView<'msg> {
  type Message = StopTimeEvent;
}

impl ::std::fmt::Debug for StopTimeEventView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StopTimeEventView<'_> {
  fn default() -> StopTimeEventView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeEvent>> for StopTimeEventView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeEvent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopTimeEventView<'msg> {

  pub fn to_owned(&self) -> StopTimeEvent {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // delay: optional int32
  pub fn has_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn delay_opt(self) -> ::std::option::Option<i32> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // time: optional int64
  pub fn has_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn time_opt(self) -> ::std::option::Option<i64> {
    self.has_time().then(|| self.time())
  }
  pub fn time(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }

  // uncertainty: optional int32
  pub fn has_uncertainty(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn uncertainty_opt(self) -> ::std::option::Option<i32> {
    self.has_uncertainty().then(|| self.uncertainty())
  }
  pub fn uncertainty(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // scheduled_time: optional int64
  pub fn has_scheduled_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn scheduled_time_opt(self) -> ::std::option::Option<i64> {
    self.has_scheduled_time().then(|| self.scheduled_time())
  }
  pub fn scheduled_time(self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `StopTimeEventView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StopTimeEventView<'_> {}

// SAFETY:
// - `StopTimeEventView` is `Send` because while its alive a `StopTimeEventMut` cannot.
// - `StopTimeEventView` does not use thread-local data.
unsafe impl ::std::marker::Send for StopTimeEventView<'_> {}

impl<'msg> ::protobuf::AsView for StopTimeEventView<'msg> {
  type Proxied = StopTimeEvent;
  fn as_view(&self) -> ::protobuf::View<'msg, StopTimeEvent> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopTimeEventView<'msg> {
  fn into_view<'shorter>(self) -> StopTimeEventView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StopTimeEvent> for StopTimeEventView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopTimeEvent {
    let mut dst = StopTimeEvent::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StopTimeEvent> for StopTimeEventMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopTimeEvent {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StopTimeEvent {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopTimeEventView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopTimeEventMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StopTimeEventMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeEvent>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopTimeEventMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StopTimeEventMut<'msg> {
  type Message = StopTimeEvent;
}

impl ::std::fmt::Debug for StopTimeEventMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeEvent>> for StopTimeEventMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeEvent>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopTimeEventMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeEvent> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StopTimeEvent {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // delay: optional int32
  pub fn has_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn delay_opt(&self) -> ::std::option::Option<i32> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delay(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // time: optional int64
  pub fn has_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn time_opt(&self) -> ::std::option::Option<i64> {
    self.has_time().then(|| self.time())
  }
  pub fn time(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_time(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // uncertainty: optional int32
  pub fn has_uncertainty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_uncertainty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn uncertainty_opt(&self) -> ::std::option::Option<i32> {
    self.has_uncertainty().then(|| self.uncertainty())
  }
  pub fn uncertainty(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uncertainty(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // scheduled_time: optional int64
  pub fn has_scheduled_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_scheduled_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn scheduled_time_opt(&self) -> ::std::option::Option<i64> {
    self.has_scheduled_time().then(|| self.scheduled_time())
  }
  pub fn scheduled_time(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scheduled_time(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `StopTimeEventMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StopTimeEventMut<'_> {}

// SAFETY:
// - `StopTimeEventMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StopTimeEventMut<'_> {}

impl<'msg> ::protobuf::AsView for StopTimeEventMut<'msg> {
  type Proxied = StopTimeEvent;
  fn as_view(&self) -> ::protobuf::View<'_, StopTimeEvent> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopTimeEventMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StopTimeEvent>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StopTimeEventMut<'msg> {
  type MutProxied = StopTimeEvent;
  fn as_mut(&mut self) -> StopTimeEventMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StopTimeEventMut<'msg> {
  fn into_mut<'shorter>(self) -> StopTimeEventMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StopTimeEvent {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StopTimeEvent> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StopTimeEventView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StopTimeEventMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // delay: optional int32
  pub fn has_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn delay_opt(&self) -> ::std::option::Option<i32> {
    self.has_delay().then(|| self.delay())
  }
  pub fn delay(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_delay(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // time: optional int64
  pub fn has_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn time_opt(&self) -> ::std::option::Option<i64> {
    self.has_time().then(|| self.time())
  }
  pub fn time(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        1, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_time(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        1, val.into()
      )
    }
  }

  // uncertainty: optional int32
  pub fn has_uncertainty(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_uncertainty(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn uncertainty_opt(&self) -> ::std::option::Option<i32> {
    self.has_uncertainty().then(|| self.uncertainty())
  }
  pub fn uncertainty(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_uncertainty(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // scheduled_time: optional int64
  pub fn has_scheduled_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_scheduled_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn scheduled_time_opt(&self) -> ::std::option::Option<i64> {
    self.has_scheduled_time().then(|| self.scheduled_time())
  }
  pub fn scheduled_time(&self) -> i64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i64_at_index(
        3, (0i64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_scheduled_time(&mut self, val: i64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i64_at_index(
        3, val.into()
      )
    }
  }

}  // impl StopTimeEvent

impl ::std::ops::Drop for StopTimeEvent {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StopTimeEvent {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StopTimeEvent {
  type Proxied = Self;
  fn as_view(&self) -> StopTimeEventView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StopTimeEvent {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StopTimeEventMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StopTimeEvent {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::trip_update::transit_0realtime__TripUpdate__StopTimeEvent_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P(+(+");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::trip_update::transit_0realtime__TripUpdate__StopTimeEvent_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::trip_update::transit_0realtime__TripUpdate__StopTimeEvent_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopTimeEvent {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopTimeEvent {
  type Msg = StopTimeEvent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeEvent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeEvent {
  type Msg = StopTimeEvent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeEvent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopTimeEventMut<'_> {
  type Msg = StopTimeEvent;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeEvent> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeEventMut<'_> {
  type Msg = StopTimeEvent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeEvent> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeEventView<'_> {
  type Msg = StopTimeEvent;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeEvent> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopTimeEventMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripUpdate__StopTimeUpdate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StopTimeUpdate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StopTimeUpdate>
}

impl ::protobuf::Message for StopTimeUpdate {
  type MessageView<'msg> = StopTimeUpdateView<'msg>;
  type MessageMut<'msg> = StopTimeUpdateMut<'msg>;
}

impl ::std::default::Default for StopTimeUpdate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StopTimeUpdate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StopTimeUpdate` is `Sync` because it does not implement interior mutability.
//    Neither does `StopTimeUpdateMut`.
unsafe impl ::std::marker::Sync for StopTimeUpdate {}

// SAFETY:
// - `StopTimeUpdate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StopTimeUpdate {}

impl ::protobuf::Proxied for StopTimeUpdate {
  type View<'msg> = StopTimeUpdateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StopTimeUpdate {}

impl ::protobuf::MutProxied for StopTimeUpdate {
  type Mut<'msg> = StopTimeUpdateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StopTimeUpdateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopTimeUpdateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StopTimeUpdateView<'msg> {
  type Message = StopTimeUpdate;
}

impl ::std::fmt::Debug for StopTimeUpdateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StopTimeUpdateView<'_> {
  fn default() -> StopTimeUpdateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeUpdate>> for StopTimeUpdateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopTimeUpdateView<'msg> {

  pub fn to_owned(&self) -> StopTimeUpdate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // stop_sequence: optional uint32
  pub fn has_stop_sequence(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn stop_sequence_opt(self) -> ::std::option::Option<u32> {
    self.has_stop_sequence().then(|| self.stop_sequence())
  }
  pub fn stop_sequence(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // arrival: optional message transit_realtime.TripUpdate.StopTimeEvent
  pub fn has_arrival(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn arrival_opt(self) -> ::std::option::Option<super::super::trip_update::StopTimeEventView<'msg>> {
    self.has_arrival().then(|| self.arrival())
  }
  pub fn arrival(self) -> super::super::trip_update::StopTimeEventView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::StopTimeEventView::default())
  }

  // departure: optional message transit_realtime.TripUpdate.StopTimeEvent
  pub fn has_departure(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn departure_opt(self) -> ::std::option::Option<super::super::trip_update::StopTimeEventView<'msg>> {
    self.has_departure().then(|| self.departure())
  }
  pub fn departure(self) -> super::super::trip_update::StopTimeEventView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::StopTimeEventView::default())
  }

  // departure_occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_departure_occupancy_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn departure_occupancy_status_opt(self) -> ::std::option::Option<super::super::vehicle_position::OccupancyStatus> {
    self.has_departure_occupancy_status().then(|| self.departure_occupancy_status())
  }
  pub fn departure_occupancy_status(self) -> super::super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (super::super::vehicle_position::OccupancyStatus::Empty).into()
      ).try_into().unwrap()
    }
  }

  // schedule_relationship: optional enum transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship
  pub fn has_schedule_relationship(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn schedule_relationship_opt(self) -> ::std::option::Option<super::super::trip_update::stop_time_update::ScheduleRelationship> {
    self.has_schedule_relationship().then(|| self.schedule_relationship())
  }
  pub fn schedule_relationship(self) -> super::super::trip_update::stop_time_update::ScheduleRelationship {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::trip_update::stop_time_update::ScheduleRelationship::Scheduled).into()
      ).try_into().unwrap()
    }
  }

  // stop_time_properties: optional message transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties
  pub fn has_stop_time_properties(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn stop_time_properties_opt(self) -> ::std::option::Option<super::super::trip_update::stop_time_update::StopTimePropertiesView<'msg>> {
    self.has_stop_time_properties().then(|| self.stop_time_properties())
  }
  pub fn stop_time_properties(self) -> super::super::trip_update::stop_time_update::StopTimePropertiesView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::stop_time_update::StopTimePropertiesView::default())
  }

}

// SAFETY:
// - `StopTimeUpdateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StopTimeUpdateView<'_> {}

// SAFETY:
// - `StopTimeUpdateView` is `Send` because while its alive a `StopTimeUpdateMut` cannot.
// - `StopTimeUpdateView` does not use thread-local data.
unsafe impl ::std::marker::Send for StopTimeUpdateView<'_> {}

impl<'msg> ::protobuf::AsView for StopTimeUpdateView<'msg> {
  type Proxied = StopTimeUpdate;
  fn as_view(&self) -> ::protobuf::View<'msg, StopTimeUpdate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopTimeUpdateView<'msg> {
  fn into_view<'shorter>(self) -> StopTimeUpdateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StopTimeUpdate> for StopTimeUpdateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopTimeUpdate {
    let mut dst = StopTimeUpdate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StopTimeUpdate> for StopTimeUpdateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopTimeUpdate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StopTimeUpdate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopTimeUpdateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopTimeUpdateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StopTimeUpdateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopTimeUpdateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StopTimeUpdateMut<'msg> {
  type Message = StopTimeUpdate;
}

impl ::std::fmt::Debug for StopTimeUpdateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeUpdate>> for StopTimeUpdateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopTimeUpdateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeUpdate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StopTimeUpdate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // stop_sequence: optional uint32
  pub fn has_stop_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stop_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stop_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_stop_sequence().then(|| self.stop_sequence())
  }
  pub fn stop_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // arrival: optional message transit_realtime.TripUpdate.StopTimeEvent
  pub fn has_arrival(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_arrival(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn arrival_opt(&self) -> ::std::option::Option<super::super::trip_update::StopTimeEventView<'_>> {
    self.has_arrival().then(|| self.arrival())
  }
  pub fn arrival(&self) -> super::super::trip_update::StopTimeEventView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::StopTimeEventView::default())
  }
  pub fn arrival_mut(&mut self) -> super::super::trip_update::StopTimeEventMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_arrival(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::trip_update::StopTimeEvent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // departure: optional message transit_realtime.TripUpdate.StopTimeEvent
  pub fn has_departure(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_departure(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn departure_opt(&self) -> ::std::option::Option<super::super::trip_update::StopTimeEventView<'_>> {
    self.has_departure().then(|| self.departure())
  }
  pub fn departure(&self) -> super::super::trip_update::StopTimeEventView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::StopTimeEventView::default())
  }
  pub fn departure_mut(&mut self) -> super::super::trip_update::StopTimeEventMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_departure(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::trip_update::StopTimeEvent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // departure_occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_departure_occupancy_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_departure_occupancy_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn departure_occupancy_status_opt(&self) -> ::std::option::Option<super::super::vehicle_position::OccupancyStatus> {
    self.has_departure_occupancy_status().then(|| self.departure_occupancy_status())
  }
  pub fn departure_occupancy_status(&self) -> super::super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (super::super::vehicle_position::OccupancyStatus::Empty).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_departure_occupancy_status(&mut self, val: super::super::vehicle_position::OccupancyStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        6, val.into()
      )
    }
  }

  // schedule_relationship: optional enum transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship
  pub fn has_schedule_relationship(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_schedule_relationship(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn schedule_relationship_opt(&self) -> ::std::option::Option<super::super::trip_update::stop_time_update::ScheduleRelationship> {
    self.has_schedule_relationship().then(|| self.schedule_relationship())
  }
  pub fn schedule_relationship(&self) -> super::super::trip_update::stop_time_update::ScheduleRelationship {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::trip_update::stop_time_update::ScheduleRelationship::Scheduled).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schedule_relationship(&mut self, val: super::super::trip_update::stop_time_update::ScheduleRelationship) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // stop_time_properties: optional message transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties
  pub fn has_stop_time_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_stop_time_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn stop_time_properties_opt(&self) -> ::std::option::Option<super::super::trip_update::stop_time_update::StopTimePropertiesView<'_>> {
    self.has_stop_time_properties().then(|| self.stop_time_properties())
  }
  pub fn stop_time_properties(&self) -> super::super::trip_update::stop_time_update::StopTimePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::stop_time_update::StopTimePropertiesView::default())
  }
  pub fn stop_time_properties_mut(&mut self) -> super::super::trip_update::stop_time_update::StopTimePropertiesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_time_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::trip_update::stop_time_update::StopTimeProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}

// SAFETY:
// - `StopTimeUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StopTimeUpdateMut<'_> {}

// SAFETY:
// - `StopTimeUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StopTimeUpdateMut<'_> {}

impl<'msg> ::protobuf::AsView for StopTimeUpdateMut<'msg> {
  type Proxied = StopTimeUpdate;
  fn as_view(&self) -> ::protobuf::View<'_, StopTimeUpdate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopTimeUpdateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StopTimeUpdate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StopTimeUpdateMut<'msg> {
  type MutProxied = StopTimeUpdate;
  fn as_mut(&mut self) -> StopTimeUpdateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StopTimeUpdateMut<'msg> {
  fn into_mut<'shorter>(self) -> StopTimeUpdateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StopTimeUpdate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StopTimeUpdate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StopTimeUpdateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StopTimeUpdateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // stop_sequence: optional uint32
  pub fn has_stop_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stop_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stop_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_stop_sequence().then(|| self.stop_sequence())
  }
  pub fn stop_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // arrival: optional message transit_realtime.TripUpdate.StopTimeEvent
  pub fn has_arrival(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_arrival(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn arrival_opt(&self) -> ::std::option::Option<super::super::trip_update::StopTimeEventView<'_>> {
    self.has_arrival().then(|| self.arrival())
  }
  pub fn arrival(&self) -> super::super::trip_update::StopTimeEventView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::StopTimeEventView::default())
  }
  pub fn arrival_mut(&mut self) -> super::super::trip_update::StopTimeEventMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_arrival(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::trip_update::StopTimeEvent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // departure: optional message transit_realtime.TripUpdate.StopTimeEvent
  pub fn has_departure(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_departure(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn departure_opt(&self) -> ::std::option::Option<super::super::trip_update::StopTimeEventView<'_>> {
    self.has_departure().then(|| self.departure())
  }
  pub fn departure(&self) -> super::super::trip_update::StopTimeEventView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::StopTimeEventView::default())
  }
  pub fn departure_mut(&mut self) -> super::super::trip_update::StopTimeEventMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_departure(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::trip_update::StopTimeEvent>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // departure_occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_departure_occupancy_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_departure_occupancy_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn departure_occupancy_status_opt(&self) -> ::std::option::Option<super::super::vehicle_position::OccupancyStatus> {
    self.has_departure_occupancy_status().then(|| self.departure_occupancy_status())
  }
  pub fn departure_occupancy_status(&self) -> super::super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        6, (super::super::vehicle_position::OccupancyStatus::Empty).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_departure_occupancy_status(&mut self, val: super::super::vehicle_position::OccupancyStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        6, val.into()
      )
    }
  }

  // schedule_relationship: optional enum transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship
  pub fn has_schedule_relationship(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_schedule_relationship(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn schedule_relationship_opt(&self) -> ::std::option::Option<super::super::trip_update::stop_time_update::ScheduleRelationship> {
    self.has_schedule_relationship().then(|| self.schedule_relationship())
  }
  pub fn schedule_relationship(&self) -> super::super::trip_update::stop_time_update::ScheduleRelationship {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        4, (super::super::trip_update::stop_time_update::ScheduleRelationship::Scheduled).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schedule_relationship(&mut self, val: super::super::trip_update::stop_time_update::ScheduleRelationship) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        4, val.into()
      )
    }
  }

  // stop_time_properties: optional message transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties
  pub fn has_stop_time_properties(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_stop_time_properties(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn stop_time_properties_opt(&self) -> ::std::option::Option<super::super::trip_update::stop_time_update::StopTimePropertiesView<'_>> {
    self.has_stop_time_properties().then(|| self.stop_time_properties())
  }
  pub fn stop_time_properties(&self) -> super::super::trip_update::stop_time_update::StopTimePropertiesView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::trip_update::stop_time_update::StopTimePropertiesView::default())
  }
  pub fn stop_time_properties_mut(&mut self) -> super::super::trip_update::stop_time_update::StopTimePropertiesMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_time_properties(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::trip_update::stop_time_update::StopTimeProperties>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

}  // impl StopTimeUpdate

impl ::std::ops::Drop for StopTimeUpdate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StopTimeUpdate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StopTimeUpdate {
  type Proxied = Self;
  fn as_view(&self) -> StopTimeUpdateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StopTimeUpdate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StopTimeUpdateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StopTimeUpdate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::trip_update::transit_0realtime__TripUpdate__StopTimeUpdate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P)331434");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::trip_update::transit_0realtime__TripUpdate__StopTimeUpdate_msg_init.0, &[<super::super::trip_update::StopTimeEvent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::trip_update::StopTimeEvent as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::trip_update::stop_time_update::StopTimeProperties as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[<super::super::trip_update::stop_time_update::ScheduleRelationship as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            <super::super::vehicle_position::OccupancyStatus as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::trip_update::transit_0realtime__TripUpdate__StopTimeUpdate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopTimeUpdate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopTimeUpdate {
  type Msg = StopTimeUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeUpdate {
  type Msg = StopTimeUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopTimeUpdateMut<'_> {
  type Msg = StopTimeUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeUpdateMut<'_> {
  type Msg = StopTimeUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeUpdateView<'_> {
  type Msg = StopTimeUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeUpdate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopTimeUpdateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod stop_time_update {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripUpdate__StopTimeUpdate__StopTimeProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StopTimeProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StopTimeProperties>
}

impl ::protobuf::Message for StopTimeProperties {
  type MessageView<'msg> = StopTimePropertiesView<'msg>;
  type MessageMut<'msg> = StopTimePropertiesMut<'msg>;
}

impl ::std::default::Default for StopTimeProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StopTimeProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StopTimeProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `StopTimePropertiesMut`.
unsafe impl ::std::marker::Sync for StopTimeProperties {}

// SAFETY:
// - `StopTimeProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StopTimeProperties {}

impl ::protobuf::Proxied for StopTimeProperties {
  type View<'msg> = StopTimePropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StopTimeProperties {}

impl ::protobuf::MutProxied for StopTimeProperties {
  type Mut<'msg> = StopTimePropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StopTimePropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopTimePropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StopTimePropertiesView<'msg> {
  type Message = StopTimeProperties;
}

impl ::std::fmt::Debug for StopTimePropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StopTimePropertiesView<'_> {
  fn default() -> StopTimePropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeProperties>> for StopTimePropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopTimeProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopTimePropertiesView<'msg> {

  pub fn to_owned(&self) -> StopTimeProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // assigned_stop_id: optional string
  pub fn has_assigned_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn assigned_stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_assigned_stop_id().then(|| self.assigned_stop_id())
  }
  pub fn assigned_stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stop_headsign: optional string
  pub fn has_stop_headsign(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn stop_headsign_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_headsign().then(|| self.stop_headsign())
  }
  pub fn stop_headsign(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // pickup_type: optional enum transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.DropOffPickupType
  pub fn has_pickup_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn pickup_type_opt(self) -> ::std::option::Option<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType> {
    self.has_pickup_type().then(|| self.pickup_type())
  }
  pub fn pickup_type(self) -> super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType::Regular).into()
      ).try_into().unwrap()
    }
  }

  // drop_off_type: optional enum transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.DropOffPickupType
  pub fn has_drop_off_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn drop_off_type_opt(self) -> ::std::option::Option<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType> {
    self.has_drop_off_type().then(|| self.drop_off_type())
  }
  pub fn drop_off_type(self) -> super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType::Regular).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `StopTimePropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StopTimePropertiesView<'_> {}

// SAFETY:
// - `StopTimePropertiesView` is `Send` because while its alive a `StopTimePropertiesMut` cannot.
// - `StopTimePropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for StopTimePropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for StopTimePropertiesView<'msg> {
  type Proxied = StopTimeProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, StopTimeProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopTimePropertiesView<'msg> {
  fn into_view<'shorter>(self) -> StopTimePropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StopTimeProperties> for StopTimePropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopTimeProperties {
    let mut dst = StopTimeProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StopTimeProperties> for StopTimePropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopTimeProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StopTimeProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopTimePropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopTimePropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StopTimePropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopTimePropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StopTimePropertiesMut<'msg> {
  type Message = StopTimeProperties;
}

impl ::std::fmt::Debug for StopTimePropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeProperties>> for StopTimePropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopTimePropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StopTimeProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StopTimeProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // assigned_stop_id: optional string
  pub fn has_assigned_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_assigned_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn assigned_stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_assigned_stop_id().then(|| self.assigned_stop_id())
  }
  pub fn assigned_stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_assigned_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stop_headsign: optional string
  pub fn has_stop_headsign(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_headsign(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_headsign_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_headsign().then(|| self.stop_headsign())
  }
  pub fn stop_headsign(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_headsign(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // pickup_type: optional enum transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.DropOffPickupType
  pub fn has_pickup_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_pickup_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn pickup_type_opt(&self) -> ::std::option::Option<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType> {
    self.has_pickup_type().then(|| self.pickup_type())
  }
  pub fn pickup_type(&self) -> super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType::Regular).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_pickup_type(&mut self, val: super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // drop_off_type: optional enum transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.DropOffPickupType
  pub fn has_drop_off_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_drop_off_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn drop_off_type_opt(&self) -> ::std::option::Option<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType> {
    self.has_drop_off_type().then(|| self.drop_off_type())
  }
  pub fn drop_off_type(&self) -> super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType::Regular).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_drop_off_type(&mut self, val: super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `StopTimePropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StopTimePropertiesMut<'_> {}

// SAFETY:
// - `StopTimePropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StopTimePropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for StopTimePropertiesMut<'msg> {
  type Proxied = StopTimeProperties;
  fn as_view(&self) -> ::protobuf::View<'_, StopTimeProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopTimePropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StopTimeProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StopTimePropertiesMut<'msg> {
  type MutProxied = StopTimeProperties;
  fn as_mut(&mut self) -> StopTimePropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StopTimePropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> StopTimePropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StopTimeProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StopTimeProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StopTimePropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StopTimePropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // assigned_stop_id: optional string
  pub fn has_assigned_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_assigned_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn assigned_stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_assigned_stop_id().then(|| self.assigned_stop_id())
  }
  pub fn assigned_stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_assigned_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stop_headsign: optional string
  pub fn has_stop_headsign(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_headsign(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_headsign_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_headsign().then(|| self.stop_headsign())
  }
  pub fn stop_headsign(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_headsign(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // pickup_type: optional enum transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.DropOffPickupType
  pub fn has_pickup_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_pickup_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn pickup_type_opt(&self) -> ::std::option::Option<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType> {
    self.has_pickup_type().then(|| self.pickup_type())
  }
  pub fn pickup_type(&self) -> super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType::Regular).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_pickup_type(&mut self, val: super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // drop_off_type: optional enum transit_realtime.TripUpdate.StopTimeUpdate.StopTimeProperties.DropOffPickupType
  pub fn has_drop_off_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_drop_off_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn drop_off_type_opt(&self) -> ::std::option::Option<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType> {
    self.has_drop_off_type().then(|| self.drop_off_type())
  }
  pub fn drop_off_type(&self) -> super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType::Regular).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_drop_off_type(&mut self, val: super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}  // impl StopTimeProperties

impl ::std::ops::Drop for StopTimeProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StopTimeProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StopTimeProperties {
  type Proxied = Self;
  fn as_view(&self) -> StopTimePropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StopTimeProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StopTimePropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StopTimeProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::super::trip_update::stop_time_update::transit_0realtime__TripUpdate__StopTimeUpdate__StopTimeProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1144");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::super::trip_update::stop_time_update::transit_0realtime__TripUpdate__StopTimeUpdate__StopTimeProperties_msg_init.0, &[], &[<super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            <super::super::super::trip_update::stop_time_update::stop_time_properties::DropOffPickupType as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::super::trip_update::stop_time_update::transit_0realtime__TripUpdate__StopTimeUpdate__StopTimeProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopTimeProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopTimeProperties {
  type Msg = StopTimeProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimeProperties {
  type Msg = StopTimeProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopTimePropertiesMut<'_> {
  type Msg = StopTimeProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimePropertiesMut<'_> {
  type Msg = StopTimeProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopTimePropertiesView<'_> {
  type Msg = StopTimeProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopTimeProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopTimePropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod stop_time_properties {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DropOffPickupType(i32);

#[allow(non_upper_case_globals)]
impl DropOffPickupType {
  pub const Regular: DropOffPickupType = DropOffPickupType(0);
  pub const None: DropOffPickupType = DropOffPickupType(1);
  pub const PhoneAgency: DropOffPickupType = DropOffPickupType(2);
  pub const CoordinateWithDriver: DropOffPickupType = DropOffPickupType(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Regular",
      1 => "None",
      2 => "PhoneAgency",
      3 => "CoordinateWithDriver",
      _ => return None
    })
  }
}

impl ::std::convert::From<DropOffPickupType> for i32 {
  fn from(val: DropOffPickupType) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for DropOffPickupType {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<DropOffPickupType, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for DropOffPickupType {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for DropOffPickupType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "DropOffPickupType::{}", constant_name)
    } else {
      write!(f, "DropOffPickupType::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for DropOffPickupType {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for DropOffPickupType {}

impl ::protobuf::Proxied for DropOffPickupType {
  type View<'a> = DropOffPickupType;
}

impl ::protobuf::AsView for DropOffPickupType {
  type Proxied = DropOffPickupType;

  fn as_view(&self) -> DropOffPickupType {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for DropOffPickupType {
  fn into_view<'shorter>(self) -> DropOffPickupType where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for DropOffPickupType {
  const NAME: &'static str = "DropOffPickupType";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for DropOffPickupType {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for DropOffPickupType {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!1"))
    }).0
  }
}

}  // pub mod stop_time_properties

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScheduleRelationship(i32);

#[allow(non_upper_case_globals)]
impl ScheduleRelationship {
  pub const Scheduled: ScheduleRelationship = ScheduleRelationship(0);
  pub const Skipped: ScheduleRelationship = ScheduleRelationship(1);
  pub const NoData: ScheduleRelationship = ScheduleRelationship(2);
  pub const Unscheduled: ScheduleRelationship = ScheduleRelationship(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Scheduled",
      1 => "Skipped",
      2 => "NoData",
      3 => "Unscheduled",
      _ => return None
    })
  }
}

impl ::std::convert::From<ScheduleRelationship> for i32 {
  fn from(val: ScheduleRelationship) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for ScheduleRelationship {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<ScheduleRelationship, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for ScheduleRelationship {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ScheduleRelationship {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ScheduleRelationship::{}", constant_name)
    } else {
      write!(f, "ScheduleRelationship::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ScheduleRelationship {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ScheduleRelationship {}

impl ::protobuf::Proxied for ScheduleRelationship {
  type View<'a> = ScheduleRelationship;
}

impl ::protobuf::AsView for ScheduleRelationship {
  type Proxied = ScheduleRelationship;

  fn as_view(&self) -> ScheduleRelationship {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScheduleRelationship {
  fn into_view<'shorter>(self) -> ScheduleRelationship where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ScheduleRelationship {
  const NAME: &'static str = "ScheduleRelationship";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for ScheduleRelationship {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for ScheduleRelationship {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!1"))
    }).0
  }
}

}  // pub mod stop_time_update

// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripUpdate__TripProperties_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TripProperties {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TripProperties>
}

impl ::protobuf::Message for TripProperties {
  type MessageView<'msg> = TripPropertiesView<'msg>;
  type MessageMut<'msg> = TripPropertiesMut<'msg>;
}

impl ::std::default::Default for TripProperties {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TripProperties {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TripProperties` is `Sync` because it does not implement interior mutability.
//    Neither does `TripPropertiesMut`.
unsafe impl ::std::marker::Sync for TripProperties {}

// SAFETY:
// - `TripProperties` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TripProperties {}

impl ::protobuf::Proxied for TripProperties {
  type View<'msg> = TripPropertiesView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TripProperties {}

impl ::protobuf::MutProxied for TripProperties {
  type Mut<'msg> = TripPropertiesMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TripPropertiesView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripPropertiesView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TripPropertiesView<'msg> {
  type Message = TripProperties;
}

impl ::std::fmt::Debug for TripPropertiesView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TripPropertiesView<'_> {
  fn default() -> TripPropertiesView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TripProperties>> for TripPropertiesView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripPropertiesView<'msg> {

  pub fn to_owned(&self) -> TripProperties {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trip_id: optional string
  pub fn has_trip_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn trip_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_trip_id().then(|| self.trip_id())
  }
  pub fn trip_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // start_date: optional string
  pub fn has_start_date(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn start_date_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // start_time: optional string
  pub fn has_start_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn start_time_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // shape_id: optional string
  pub fn has_shape_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn shape_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // trip_headsign: optional string
  pub fn has_trip_headsign(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn trip_headsign_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_trip_headsign().then(|| self.trip_headsign())
  }
  pub fn trip_headsign(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // trip_short_name: optional string
  pub fn has_trip_short_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn trip_short_name_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_trip_short_name().then(|| self.trip_short_name())
  }
  pub fn trip_short_name(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `TripPropertiesView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TripPropertiesView<'_> {}

// SAFETY:
// - `TripPropertiesView` is `Send` because while its alive a `TripPropertiesMut` cannot.
// - `TripPropertiesView` does not use thread-local data.
unsafe impl ::std::marker::Send for TripPropertiesView<'_> {}

impl<'msg> ::protobuf::AsView for TripPropertiesView<'msg> {
  type Proxied = TripProperties;
  fn as_view(&self) -> ::protobuf::View<'msg, TripProperties> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripPropertiesView<'msg> {
  fn into_view<'shorter>(self) -> TripPropertiesView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TripProperties> for TripPropertiesView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripProperties {
    let mut dst = TripProperties::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TripProperties> for TripPropertiesMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripProperties {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TripProperties {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripPropertiesView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripPropertiesMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TripPropertiesMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripProperties>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripPropertiesMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TripPropertiesMut<'msg> {
  type Message = TripProperties;
}

impl ::std::fmt::Debug for TripPropertiesMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TripProperties>> for TripPropertiesMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripProperties>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripPropertiesMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TripProperties> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TripProperties {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trip_id: optional string
  pub fn has_trip_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_id().then(|| self.trip_id())
  }
  pub fn trip_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // start_date: optional string
  pub fn has_start_date(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_start_date(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn start_date_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // start_time: optional string
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // shape_id: optional string
  pub fn has_shape_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_shape_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn shape_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shape_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // trip_headsign: optional string
  pub fn has_trip_headsign(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_trip_headsign(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn trip_headsign_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_headsign().then(|| self.trip_headsign())
  }
  pub fn trip_headsign(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_headsign(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // trip_short_name: optional string
  pub fn has_trip_short_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_trip_short_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn trip_short_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_short_name().then(|| self.trip_short_name())
  }
  pub fn trip_short_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_short_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

}

// SAFETY:
// - `TripPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TripPropertiesMut<'_> {}

// SAFETY:
// - `TripPropertiesMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TripPropertiesMut<'_> {}

impl<'msg> ::protobuf::AsView for TripPropertiesMut<'msg> {
  type Proxied = TripProperties;
  fn as_view(&self) -> ::protobuf::View<'_, TripProperties> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripPropertiesMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TripProperties>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TripPropertiesMut<'msg> {
  type MutProxied = TripProperties;
  fn as_mut(&mut self) -> TripPropertiesMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TripPropertiesMut<'msg> {
  fn into_mut<'shorter>(self) -> TripPropertiesMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TripProperties {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TripProperties> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TripPropertiesView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TripPropertiesMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trip_id: optional string
  pub fn has_trip_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_id().then(|| self.trip_id())
  }
  pub fn trip_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // start_date: optional string
  pub fn has_start_date(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_start_date(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn start_date_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // start_time: optional string
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // shape_id: optional string
  pub fn has_shape_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_shape_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn shape_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shape_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

  // trip_headsign: optional string
  pub fn has_trip_headsign(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_trip_headsign(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn trip_headsign_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_headsign().then(|| self.trip_headsign())
  }
  pub fn trip_headsign(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_headsign(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // trip_short_name: optional string
  pub fn has_trip_short_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_trip_short_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn trip_short_name_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_short_name().then(|| self.trip_short_name())
  }
  pub fn trip_short_name(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        5, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_short_name(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val);
    }
  }

}  // impl TripProperties

impl ::std::ops::Drop for TripProperties {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TripProperties {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TripProperties {
  type Proxied = Self;
  fn as_view(&self) -> TripPropertiesView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TripProperties {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TripPropertiesMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TripProperties {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::trip_update::transit_0realtime__TripUpdate__TripProperties_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P111111");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::trip_update::transit_0realtime__TripUpdate__TripProperties_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::trip_update::transit_0realtime__TripUpdate__TripProperties_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripProperties {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripProperties {
  type Msg = TripProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripProperties {
  type Msg = TripProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripPropertiesMut<'_> {
  type Msg = TripProperties;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripProperties> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripPropertiesMut<'_> {
  type Msg = TripProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripProperties> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripPropertiesView<'_> {
  type Msg = TripProperties;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripProperties> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripPropertiesMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod trip_update


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__VehiclePosition_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct VehiclePosition {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<VehiclePosition>
}

impl ::protobuf::Message for VehiclePosition {
  type MessageView<'msg> = VehiclePositionView<'msg>;
  type MessageMut<'msg> = VehiclePositionMut<'msg>;
}

impl ::std::default::Default for VehiclePosition {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for VehiclePosition {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `VehiclePosition` is `Sync` because it does not implement interior mutability.
//    Neither does `VehiclePositionMut`.
unsafe impl ::std::marker::Sync for VehiclePosition {}

// SAFETY:
// - `VehiclePosition` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for VehiclePosition {}

impl ::protobuf::Proxied for VehiclePosition {
  type View<'msg> = VehiclePositionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for VehiclePosition {}

impl ::protobuf::MutProxied for VehiclePosition {
  type Mut<'msg> = VehiclePositionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VehiclePositionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, VehiclePosition>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VehiclePositionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VehiclePositionView<'msg> {
  type Message = VehiclePosition;
}

impl ::std::fmt::Debug for VehiclePositionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for VehiclePositionView<'_> {
  fn default() -> VehiclePositionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, VehiclePosition>> for VehiclePositionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, VehiclePosition>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VehiclePositionView<'msg> {

  pub fn to_owned(&self) -> VehiclePosition {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn trip_opt(self) -> ::std::option::Option<super::TripDescriptorView<'msg>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(self) -> super::TripDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }

  // vehicle: optional message transit_realtime.VehicleDescriptor
  pub fn has_vehicle(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn vehicle_opt(self) -> ::std::option::Option<super::VehicleDescriptorView<'msg>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(self) -> super::VehicleDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleDescriptorView::default())
  }

  // position: optional message transit_realtime.Position
  pub fn has_position(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn position_opt(self) -> ::std::option::Option<super::PositionView<'msg>> {
    self.has_position().then(|| self.position())
  }
  pub fn position(self) -> super::PositionView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PositionView::default())
  }

  // current_stop_sequence: optional uint32
  pub fn has_current_stop_sequence(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn current_stop_sequence_opt(self) -> ::std::option::Option<u32> {
    self.has_current_stop_sequence().then(|| self.current_stop_sequence())
  }
  pub fn current_stop_sequence(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // current_status: optional enum transit_realtime.VehiclePosition.VehicleStopStatus
  pub fn has_current_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn current_status_opt(self) -> ::std::option::Option<super::vehicle_position::VehicleStopStatus> {
    self.has_current_status().then(|| self.current_status())
  }
  pub fn current_status(self) -> super::vehicle_position::VehicleStopStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::vehicle_position::VehicleStopStatus::InTransitTo).into()
      ).try_into().unwrap()
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn timestamp_opt(self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // congestion_level: optional enum transit_realtime.VehiclePosition.CongestionLevel
  pub fn has_congestion_level(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn congestion_level_opt(self) -> ::std::option::Option<super::vehicle_position::CongestionLevel> {
    self.has_congestion_level().then(|| self.congestion_level())
  }
  pub fn congestion_level(self) -> super::vehicle_position::CongestionLevel {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::vehicle_position::CongestionLevel::UnknownCongestionLevel).into()
      ).try_into().unwrap()
    }
  }

  // occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_occupancy_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn occupancy_status_opt(self) -> ::std::option::Option<super::vehicle_position::OccupancyStatus> {
    self.has_occupancy_status().then(|| self.occupancy_status())
  }
  pub fn occupancy_status(self) -> super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        8, (super::vehicle_position::OccupancyStatus::Empty).into()
      ).try_into().unwrap()
    }
  }

  // occupancy_percentage: optional uint32
  pub fn has_occupancy_percentage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn occupancy_percentage_opt(self) -> ::std::option::Option<u32> {
    self.has_occupancy_percentage().then(|| self.occupancy_percentage())
  }
  pub fn occupancy_percentage(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        9, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // multi_carriage_details: repeated message transit_realtime.VehiclePosition.CarriageDetails
  pub fn multi_carriage_details(self) -> ::protobuf::RepeatedView<'msg, super::vehicle_position::CarriageDetails> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::vehicle_position::CarriageDetails>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `VehiclePositionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for VehiclePositionView<'_> {}

// SAFETY:
// - `VehiclePositionView` is `Send` because while its alive a `VehiclePositionMut` cannot.
// - `VehiclePositionView` does not use thread-local data.
unsafe impl ::std::marker::Send for VehiclePositionView<'_> {}

impl<'msg> ::protobuf::AsView for VehiclePositionView<'msg> {
  type Proxied = VehiclePosition;
  fn as_view(&self) -> ::protobuf::View<'msg, VehiclePosition> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehiclePositionView<'msg> {
  fn into_view<'shorter>(self) -> VehiclePositionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<VehiclePosition> for VehiclePositionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VehiclePosition {
    let mut dst = VehiclePosition::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<VehiclePosition> for VehiclePositionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VehiclePosition {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for VehiclePosition {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VehiclePositionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VehiclePositionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VehiclePositionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, VehiclePosition>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VehiclePositionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VehiclePositionMut<'msg> {
  type Message = VehiclePosition;
}

impl ::std::fmt::Debug for VehiclePositionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, VehiclePosition>> for VehiclePositionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, VehiclePosition>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VehiclePositionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, VehiclePosition> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> VehiclePosition {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_opt(&self) -> ::std::option::Option<super::TripDescriptorView<'_>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(&self) -> super::TripDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }
  pub fn trip_mut(&mut self) -> super::TripDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // vehicle: optional message transit_realtime.VehicleDescriptor
  pub fn has_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn vehicle_opt(&self) -> ::std::option::Option<super::VehicleDescriptorView<'_>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(&self) -> super::VehicleDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleDescriptorView::default())
  }
  pub fn vehicle_mut(&mut self) -> super::VehicleDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::VehicleDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // position: optional message transit_realtime.Position
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn position_opt(&self) -> ::std::option::Option<super::PositionView<'_>> {
    self.has_position().then(|| self.position())
  }
  pub fn position(&self) -> super::PositionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PositionView::default())
  }
  pub fn position_mut(&mut self) -> super::PositionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Position>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // current_stop_sequence: optional uint32
  pub fn has_current_stop_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_current_stop_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn current_stop_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_current_stop_sequence().then(|| self.current_stop_sequence())
  }
  pub fn current_stop_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_current_stop_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // current_status: optional enum transit_realtime.VehiclePosition.VehicleStopStatus
  pub fn has_current_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_current_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn current_status_opt(&self) -> ::std::option::Option<super::vehicle_position::VehicleStopStatus> {
    self.has_current_status().then(|| self.current_status())
  }
  pub fn current_status(&self) -> super::vehicle_position::VehicleStopStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::vehicle_position::VehicleStopStatus::InTransitTo).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_current_status(&mut self, val: super::vehicle_position::VehicleStopStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // congestion_level: optional enum transit_realtime.VehiclePosition.CongestionLevel
  pub fn has_congestion_level(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_congestion_level(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn congestion_level_opt(&self) -> ::std::option::Option<super::vehicle_position::CongestionLevel> {
    self.has_congestion_level().then(|| self.congestion_level())
  }
  pub fn congestion_level(&self) -> super::vehicle_position::CongestionLevel {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::vehicle_position::CongestionLevel::UnknownCongestionLevel).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_congestion_level(&mut self, val: super::vehicle_position::CongestionLevel) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  // occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_occupancy_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_occupancy_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn occupancy_status_opt(&self) -> ::std::option::Option<super::vehicle_position::OccupancyStatus> {
    self.has_occupancy_status().then(|| self.occupancy_status())
  }
  pub fn occupancy_status(&self) -> super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        8, (super::vehicle_position::OccupancyStatus::Empty).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_status(&mut self, val: super::vehicle_position::OccupancyStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        8, val.into()
      )
    }
  }

  // occupancy_percentage: optional uint32
  pub fn has_occupancy_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_occupancy_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn occupancy_percentage_opt(&self) -> ::std::option::Option<u32> {
    self.has_occupancy_percentage().then(|| self.occupancy_percentage())
  }
  pub fn occupancy_percentage(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        9, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_percentage(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        9, val.into()
      )
    }
  }

  // multi_carriage_details: repeated message transit_realtime.VehiclePosition.CarriageDetails
  pub fn multi_carriage_details(&self) -> ::protobuf::RepeatedView<'_, super::vehicle_position::CarriageDetails> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::vehicle_position::CarriageDetails>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn multi_carriage_details_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::vehicle_position::CarriageDetails> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_multi_carriage_details(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::vehicle_position::CarriageDetails>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

}

// SAFETY:
// - `VehiclePositionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for VehiclePositionMut<'_> {}

// SAFETY:
// - `VehiclePositionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for VehiclePositionMut<'_> {}

impl<'msg> ::protobuf::AsView for VehiclePositionMut<'msg> {
  type Proxied = VehiclePosition;
  fn as_view(&self) -> ::protobuf::View<'_, VehiclePosition> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehiclePositionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, VehiclePosition>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for VehiclePositionMut<'msg> {
  type MutProxied = VehiclePosition;
  fn as_mut(&mut self) -> VehiclePositionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VehiclePositionMut<'msg> {
  fn into_mut<'shorter>(self) -> VehiclePositionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl VehiclePosition {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, VehiclePosition> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> VehiclePositionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> VehiclePositionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_opt(&self) -> ::std::option::Option<super::TripDescriptorView<'_>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(&self) -> super::TripDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }
  pub fn trip_mut(&mut self) -> super::TripDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // vehicle: optional message transit_realtime.VehicleDescriptor
  pub fn has_vehicle(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_vehicle(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn vehicle_opt(&self) -> ::std::option::Option<super::VehicleDescriptorView<'_>> {
    self.has_vehicle().then(|| self.vehicle())
  }
  pub fn vehicle(&self) -> super::VehicleDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::VehicleDescriptorView::default())
  }
  pub fn vehicle_mut(&mut self) -> super::VehicleDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_vehicle(&mut self,
    val: impl ::protobuf::IntoProxied<super::VehicleDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // position: optional message transit_realtime.Position
  pub fn has_position(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_position(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn position_opt(&self) -> ::std::option::Option<super::PositionView<'_>> {
    self.has_position().then(|| self.position())
  }
  pub fn position(&self) -> super::PositionView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::PositionView::default())
  }
  pub fn position_mut(&mut self) -> super::PositionMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_position(&mut self,
    val: impl ::protobuf::IntoProxied<super::Position>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // current_stop_sequence: optional uint32
  pub fn has_current_stop_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_current_stop_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn current_stop_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_current_stop_sequence().then(|| self.current_stop_sequence())
  }
  pub fn current_stop_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        2, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_current_stop_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        2, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        6, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val);
    }
  }

  // current_status: optional enum transit_realtime.VehiclePosition.VehicleStopStatus
  pub fn has_current_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_current_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn current_status_opt(&self) -> ::std::option::Option<super::vehicle_position::VehicleStopStatus> {
    self.has_current_status().then(|| self.current_status())
  }
  pub fn current_status(&self) -> super::vehicle_position::VehicleStopStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::vehicle_position::VehicleStopStatus::InTransitTo).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_current_status(&mut self, val: super::vehicle_position::VehicleStopStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // timestamp: optional uint64
  pub fn has_timestamp(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_timestamp(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn timestamp_opt(&self) -> ::std::option::Option<u64> {
    self.has_timestamp().then(|| self.timestamp())
  }
  pub fn timestamp(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        4, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_timestamp(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        4, val.into()
      )
    }
  }

  // congestion_level: optional enum transit_realtime.VehiclePosition.CongestionLevel
  pub fn has_congestion_level(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_congestion_level(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn congestion_level_opt(&self) -> ::std::option::Option<super::vehicle_position::CongestionLevel> {
    self.has_congestion_level().then(|| self.congestion_level())
  }
  pub fn congestion_level(&self) -> super::vehicle_position::CongestionLevel {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        5, (super::vehicle_position::CongestionLevel::UnknownCongestionLevel).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_congestion_level(&mut self, val: super::vehicle_position::CongestionLevel) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        5, val.into()
      )
    }
  }

  // occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_occupancy_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_occupancy_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn occupancy_status_opt(&self) -> ::std::option::Option<super::vehicle_position::OccupancyStatus> {
    self.has_occupancy_status().then(|| self.occupancy_status())
  }
  pub fn occupancy_status(&self) -> super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        8, (super::vehicle_position::OccupancyStatus::Empty).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_status(&mut self, val: super::vehicle_position::OccupancyStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        8, val.into()
      )
    }
  }

  // occupancy_percentage: optional uint32
  pub fn has_occupancy_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_occupancy_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn occupancy_percentage_opt(&self) -> ::std::option::Option<u32> {
    self.has_occupancy_percentage().then(|| self.occupancy_percentage())
  }
  pub fn occupancy_percentage(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        9, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_percentage(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        9, val.into()
      )
    }
  }

  // multi_carriage_details: repeated message transit_realtime.VehiclePosition.CarriageDetails
  pub fn multi_carriage_details(&self) -> ::protobuf::RepeatedView<'_, super::vehicle_position::CarriageDetails> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        10
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::vehicle_position::CarriageDetails>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn multi_carriage_details_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::vehicle_position::CarriageDetails> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        10,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_multi_carriage_details(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::vehicle_position::CarriageDetails>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        src);
    }
  }

}  // impl VehiclePosition

impl ::std::ops::Drop for VehiclePosition {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for VehiclePosition {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for VehiclePosition {
  type Proxied = Self;
  fn as_view(&self) -> VehiclePositionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for VehiclePosition {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VehiclePositionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for VehiclePosition {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__VehiclePosition_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P33)4,4134)G");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__VehiclePosition_msg_init.0, &[<super::TripDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::Position as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::VehicleDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::vehicle_position::CarriageDetails as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[<super::vehicle_position::VehicleStopStatus as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            <super::vehicle_position::CongestionLevel as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            <super::vehicle_position::OccupancyStatus as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__VehiclePosition_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VehiclePosition {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VehiclePosition {
  type Msg = VehiclePosition;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehiclePosition> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehiclePosition {
  type Msg = VehiclePosition;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehiclePosition> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VehiclePositionMut<'_> {
  type Msg = VehiclePosition;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehiclePosition> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehiclePositionMut<'_> {
  type Msg = VehiclePosition;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehiclePosition> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehiclePositionView<'_> {
  type Msg = VehiclePosition;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehiclePosition> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VehiclePositionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod vehicle_position {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__VehiclePosition__CarriageDetails_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct CarriageDetails {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<CarriageDetails>
}

impl ::protobuf::Message for CarriageDetails {
  type MessageView<'msg> = CarriageDetailsView<'msg>;
  type MessageMut<'msg> = CarriageDetailsMut<'msg>;
}

impl ::std::default::Default for CarriageDetails {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for CarriageDetails {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `CarriageDetails` is `Sync` because it does not implement interior mutability.
//    Neither does `CarriageDetailsMut`.
unsafe impl ::std::marker::Sync for CarriageDetails {}

// SAFETY:
// - `CarriageDetails` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for CarriageDetails {}

impl ::protobuf::Proxied for CarriageDetails {
  type View<'msg> = CarriageDetailsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for CarriageDetails {}

impl ::protobuf::MutProxied for CarriageDetails {
  type Mut<'msg> = CarriageDetailsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct CarriageDetailsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CarriageDetails>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CarriageDetailsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for CarriageDetailsView<'msg> {
  type Message = CarriageDetails;
}

impl ::std::fmt::Debug for CarriageDetailsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for CarriageDetailsView<'_> {
  fn default() -> CarriageDetailsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, CarriageDetails>> for CarriageDetailsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, CarriageDetails>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CarriageDetailsView<'msg> {

  pub fn to_owned(&self) -> CarriageDetails {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn has_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // label: optional string
  pub fn has_label(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn label_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_label().then(|| self.label())
  }
  pub fn label(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_occupancy_status(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn occupancy_status_opt(self) -> ::std::option::Option<super::super::vehicle_position::OccupancyStatus> {
    self.has_occupancy_status().then(|| self.occupancy_status())
  }
  pub fn occupancy_status(self) -> super::super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::super::vehicle_position::OccupancyStatus::NoDataAvailable).into()
      ).try_into().unwrap()
    }
  }

  // occupancy_percentage: optional int32
  pub fn has_occupancy_percentage(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn occupancy_percentage_opt(self) -> ::std::option::Option<i32> {
    self.has_occupancy_percentage().then(|| self.occupancy_percentage())
  }
  pub fn occupancy_percentage(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (-1i32).into()
      ).try_into().unwrap()
    }
  }

  // carriage_sequence: optional uint32
  pub fn has_carriage_sequence(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn carriage_sequence_opt(self) -> ::std::option::Option<u32> {
    self.has_carriage_sequence().then(|| self.carriage_sequence())
  }
  pub fn carriage_sequence(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `CarriageDetailsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for CarriageDetailsView<'_> {}

// SAFETY:
// - `CarriageDetailsView` is `Send` because while its alive a `CarriageDetailsMut` cannot.
// - `CarriageDetailsView` does not use thread-local data.
unsafe impl ::std::marker::Send for CarriageDetailsView<'_> {}

impl<'msg> ::protobuf::AsView for CarriageDetailsView<'msg> {
  type Proxied = CarriageDetails;
  fn as_view(&self) -> ::protobuf::View<'msg, CarriageDetails> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CarriageDetailsView<'msg> {
  fn into_view<'shorter>(self) -> CarriageDetailsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<CarriageDetails> for CarriageDetailsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CarriageDetails {
    let mut dst = CarriageDetails::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<CarriageDetails> for CarriageDetailsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> CarriageDetails {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for CarriageDetails {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CarriageDetailsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for CarriageDetailsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct CarriageDetailsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CarriageDetails>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for CarriageDetailsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for CarriageDetailsMut<'msg> {
  type Message = CarriageDetails;
}

impl ::std::fmt::Debug for CarriageDetailsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, CarriageDetails>> for CarriageDetailsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, CarriageDetails>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> CarriageDetailsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, CarriageDetails> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> CarriageDetails {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // label: optional string
  pub fn has_label(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_label(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn label_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_label().then(|| self.label())
  }
  pub fn label(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_label(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_occupancy_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_occupancy_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn occupancy_status_opt(&self) -> ::std::option::Option<super::super::vehicle_position::OccupancyStatus> {
    self.has_occupancy_status().then(|| self.occupancy_status())
  }
  pub fn occupancy_status(&self) -> super::super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::super::vehicle_position::OccupancyStatus::NoDataAvailable).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_status(&mut self, val: super::super::vehicle_position::OccupancyStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // occupancy_percentage: optional int32
  pub fn has_occupancy_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_occupancy_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn occupancy_percentage_opt(&self) -> ::std::option::Option<i32> {
    self.has_occupancy_percentage().then(|| self.occupancy_percentage())
  }
  pub fn occupancy_percentage(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (-1i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_percentage(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // carriage_sequence: optional uint32
  pub fn has_carriage_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_carriage_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn carriage_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_carriage_sequence().then(|| self.carriage_sequence())
  }
  pub fn carriage_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_carriage_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `CarriageDetailsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for CarriageDetailsMut<'_> {}

// SAFETY:
// - `CarriageDetailsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for CarriageDetailsMut<'_> {}

impl<'msg> ::protobuf::AsView for CarriageDetailsMut<'msg> {
  type Proxied = CarriageDetails;
  fn as_view(&self) -> ::protobuf::View<'_, CarriageDetails> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CarriageDetailsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, CarriageDetails>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for CarriageDetailsMut<'msg> {
  type MutProxied = CarriageDetails;
  fn as_mut(&mut self) -> CarriageDetailsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for CarriageDetailsMut<'msg> {
  fn into_mut<'shorter>(self) -> CarriageDetailsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl CarriageDetails {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, CarriageDetails> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> CarriageDetailsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> CarriageDetailsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // label: optional string
  pub fn has_label(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_label(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn label_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_label().then(|| self.label())
  }
  pub fn label(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_label(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // occupancy_status: optional enum transit_realtime.VehiclePosition.OccupancyStatus
  pub fn has_occupancy_status(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_occupancy_status(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn occupancy_status_opt(&self) -> ::std::option::Option<super::super::vehicle_position::OccupancyStatus> {
    self.has_occupancy_status().then(|| self.occupancy_status())
  }
  pub fn occupancy_status(&self) -> super::super::vehicle_position::OccupancyStatus {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::super::vehicle_position::OccupancyStatus::NoDataAvailable).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_status(&mut self, val: super::super::vehicle_position::OccupancyStatus) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // occupancy_percentage: optional int32
  pub fn has_occupancy_percentage(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_occupancy_percentage(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn occupancy_percentage_opt(&self) -> ::std::option::Option<i32> {
    self.has_occupancy_percentage().then(|| self.occupancy_percentage())
  }
  pub fn occupancy_percentage(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (-1i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_occupancy_percentage(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // carriage_sequence: optional uint32
  pub fn has_carriage_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_carriage_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn carriage_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_carriage_sequence().then(|| self.carriage_sequence())
  }
  pub fn carriage_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        4, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_carriage_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        4, val.into()
      )
    }
  }

}  // impl CarriageDetails

impl ::std::ops::Drop for CarriageDetails {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for CarriageDetails {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for CarriageDetails {
  type Proxied = Self;
  fn as_view(&self) -> CarriageDetailsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for CarriageDetails {
  type MutProxied = Self;
  fn as_mut(&mut self) -> CarriageDetailsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for CarriageDetails {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::vehicle_position::transit_0realtime__VehiclePosition__CarriageDetails_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P114()");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::vehicle_position::transit_0realtime__VehiclePosition__CarriageDetails_msg_init.0, &[], &[<super::super::vehicle_position::OccupancyStatus as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::vehicle_position::transit_0realtime__VehiclePosition__CarriageDetails_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CarriageDetails {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CarriageDetails {
  type Msg = CarriageDetails;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CarriageDetails> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CarriageDetails {
  type Msg = CarriageDetails;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CarriageDetails> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for CarriageDetailsMut<'_> {
  type Msg = CarriageDetails;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CarriageDetails> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CarriageDetailsMut<'_> {
  type Msg = CarriageDetails;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CarriageDetails> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for CarriageDetailsView<'_> {
  type Msg = CarriageDetails;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<CarriageDetails> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for CarriageDetailsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VehicleStopStatus(i32);

#[allow(non_upper_case_globals)]
impl VehicleStopStatus {
  pub const IncomingAt: VehicleStopStatus = VehicleStopStatus(0);
  pub const StoppedAt: VehicleStopStatus = VehicleStopStatus(1);
  pub const InTransitTo: VehicleStopStatus = VehicleStopStatus(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "IncomingAt",
      1 => "StoppedAt",
      2 => "InTransitTo",
      _ => return None
    })
  }
}

impl ::std::convert::From<VehicleStopStatus> for i32 {
  fn from(val: VehicleStopStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for VehicleStopStatus {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<VehicleStopStatus, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for VehicleStopStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for VehicleStopStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "VehicleStopStatus::{}", constant_name)
    } else {
      write!(f, "VehicleStopStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for VehicleStopStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for VehicleStopStatus {}

impl ::protobuf::Proxied for VehicleStopStatus {
  type View<'a> = VehicleStopStatus;
}

impl ::protobuf::AsView for VehicleStopStatus {
  type Proxied = VehicleStopStatus;

  fn as_view(&self) -> VehicleStopStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehicleStopStatus {
  fn into_view<'shorter>(self) -> VehicleStopStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for VehicleStopStatus {
  const NAME: &'static str = "VehicleStopStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for VehicleStopStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for VehicleStopStatus {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!)"))
    }).0
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CongestionLevel(i32);

#[allow(non_upper_case_globals)]
impl CongestionLevel {
  pub const UnknownCongestionLevel: CongestionLevel = CongestionLevel(0);
  pub const RunningSmoothly: CongestionLevel = CongestionLevel(1);
  pub const StopAndGo: CongestionLevel = CongestionLevel(2);
  pub const Congestion: CongestionLevel = CongestionLevel(3);
  pub const SevereCongestion: CongestionLevel = CongestionLevel(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "UnknownCongestionLevel",
      1 => "RunningSmoothly",
      2 => "StopAndGo",
      3 => "Congestion",
      4 => "SevereCongestion",
      _ => return None
    })
  }
}

impl ::std::convert::From<CongestionLevel> for i32 {
  fn from(val: CongestionLevel) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for CongestionLevel {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<CongestionLevel, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for CongestionLevel {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for CongestionLevel {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "CongestionLevel::{}", constant_name)
    } else {
      write!(f, "CongestionLevel::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for CongestionLevel {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for CongestionLevel {}

impl ::protobuf::Proxied for CongestionLevel {
  type View<'a> = CongestionLevel;
}

impl ::protobuf::AsView for CongestionLevel {
  type Proxied = CongestionLevel;

  fn as_view(&self) -> CongestionLevel {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for CongestionLevel {
  fn into_view<'shorter>(self) -> CongestionLevel where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for CongestionLevel {
  const NAME: &'static str = "CongestionLevel";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for CongestionLevel {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for CongestionLevel {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!A"))
    }).0
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OccupancyStatus(i32);

#[allow(non_upper_case_globals)]
impl OccupancyStatus {
  pub const Empty: OccupancyStatus = OccupancyStatus(0);
  pub const ManySeatsAvailable: OccupancyStatus = OccupancyStatus(1);
  pub const FewSeatsAvailable: OccupancyStatus = OccupancyStatus(2);
  pub const StandingRoomOnly: OccupancyStatus = OccupancyStatus(3);
  pub const CrushedStandingRoomOnly: OccupancyStatus = OccupancyStatus(4);
  pub const Full: OccupancyStatus = OccupancyStatus(5);
  pub const NotAcceptingPassengers: OccupancyStatus = OccupancyStatus(6);
  pub const NoDataAvailable: OccupancyStatus = OccupancyStatus(7);
  pub const NotBoardable: OccupancyStatus = OccupancyStatus(8);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Empty",
      1 => "ManySeatsAvailable",
      2 => "FewSeatsAvailable",
      3 => "StandingRoomOnly",
      4 => "CrushedStandingRoomOnly",
      5 => "Full",
      6 => "NotAcceptingPassengers",
      7 => "NoDataAvailable",
      8 => "NotBoardable",
      _ => return None
    })
  }
}

impl ::std::convert::From<OccupancyStatus> for i32 {
  fn from(val: OccupancyStatus) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for OccupancyStatus {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<OccupancyStatus, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for OccupancyStatus {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for OccupancyStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "OccupancyStatus::{}", constant_name)
    } else {
      write!(f, "OccupancyStatus::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for OccupancyStatus {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for OccupancyStatus {}

impl ::protobuf::Proxied for OccupancyStatus {
  type View<'a> = OccupancyStatus;
}

impl ::protobuf::AsView for OccupancyStatus {
  type Proxied = OccupancyStatus;

  fn as_view(&self) -> OccupancyStatus {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for OccupancyStatus {
  fn into_view<'shorter>(self) -> OccupancyStatus where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for OccupancyStatus {
  const NAME: &'static str = "OccupancyStatus";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|4|5|6|7|8)
  }
}

impl ::protobuf::__internal::EntityType for OccupancyStatus {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for OccupancyStatus {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!A1"))
    }).0
  }
}

}  // pub mod vehicle_position


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__Alert_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Alert {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Alert>
}

impl ::protobuf::Message for Alert {
  type MessageView<'msg> = AlertView<'msg>;
  type MessageMut<'msg> = AlertMut<'msg>;
}

impl ::std::default::Default for Alert {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Alert {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Alert` is `Sync` because it does not implement interior mutability.
//    Neither does `AlertMut`.
unsafe impl ::std::marker::Sync for Alert {}

// SAFETY:
// - `Alert` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Alert {}

impl ::protobuf::Proxied for Alert {
  type View<'msg> = AlertView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Alert {}

impl ::protobuf::MutProxied for Alert {
  type Mut<'msg> = AlertMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct AlertView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Alert>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AlertView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for AlertView<'msg> {
  type Message = Alert;
}

impl ::std::fmt::Debug for AlertView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for AlertView<'_> {
  fn default() -> AlertView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Alert>> for AlertView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Alert>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AlertView<'msg> {

  pub fn to_owned(&self) -> Alert {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // active_period: repeated message transit_realtime.TimeRange
  pub fn active_period(self) -> ::protobuf::RepeatedView<'msg, super::TimeRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TimeRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // informed_entity: repeated message transit_realtime.EntitySelector
  pub fn informed_entity(self) -> ::protobuf::RepeatedView<'msg, super::EntitySelector> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EntitySelector>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // cause: optional enum transit_realtime.Alert.Cause
  pub fn has_cause(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn cause_opt(self) -> ::std::option::Option<super::alert::Cause> {
    self.has_cause().then(|| self.cause())
  }
  pub fn cause(self) -> super::alert::Cause {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::alert::Cause::UnknownCause).into()
      ).try_into().unwrap()
    }
  }

  // effect: optional enum transit_realtime.Alert.Effect
  pub fn has_effect(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn effect_opt(self) -> ::std::option::Option<super::alert::Effect> {
    self.has_effect().then(|| self.effect())
  }
  pub fn effect(self) -> super::alert::Effect {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::alert::Effect::UnknownEffect).into()
      ).try_into().unwrap()
    }
  }

  // url: optional message transit_realtime.TranslatedString
  pub fn has_url(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn url_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_url().then(|| self.url())
  }
  pub fn url(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // header_text: optional message transit_realtime.TranslatedString
  pub fn has_header_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn header_text_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_header_text().then(|| self.header_text())
  }
  pub fn header_text(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // description_text: optional message transit_realtime.TranslatedString
  pub fn has_description_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn description_text_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_description_text().then(|| self.description_text())
  }
  pub fn description_text(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // tts_header_text: optional message transit_realtime.TranslatedString
  pub fn has_tts_header_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn tts_header_text_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_tts_header_text().then(|| self.tts_header_text())
  }
  pub fn tts_header_text(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // tts_description_text: optional message transit_realtime.TranslatedString
  pub fn has_tts_description_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn tts_description_text_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_tts_description_text().then(|| self.tts_description_text())
  }
  pub fn tts_description_text(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // severity_level: optional enum transit_realtime.Alert.SeverityLevel
  pub fn has_severity_level(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn severity_level_opt(self) -> ::std::option::Option<super::alert::SeverityLevel> {
    self.has_severity_level().then(|| self.severity_level())
  }
  pub fn severity_level(self) -> super::alert::SeverityLevel {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        9, (super::alert::SeverityLevel::UnknownSeverity).into()
      ).try_into().unwrap()
    }
  }

  // image: optional message transit_realtime.TranslatedImage
  pub fn has_image(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn image_opt(self) -> ::std::option::Option<super::TranslatedImageView<'msg>> {
    self.has_image().then(|| self.image())
  }
  pub fn image(self) -> super::TranslatedImageView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedImageView::default())
  }

  // image_alternative_text: optional message transit_realtime.TranslatedString
  pub fn has_image_alternative_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn image_alternative_text_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_image_alternative_text().then(|| self.image_alternative_text())
  }
  pub fn image_alternative_text(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // cause_detail: optional message transit_realtime.TranslatedString
  pub fn has_cause_detail(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn cause_detail_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_cause_detail().then(|| self.cause_detail())
  }
  pub fn cause_detail(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // effect_detail: optional message transit_realtime.TranslatedString
  pub fn has_effect_detail(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn effect_detail_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_effect_detail().then(|| self.effect_detail())
  }
  pub fn effect_detail(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

}

// SAFETY:
// - `AlertView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for AlertView<'_> {}

// SAFETY:
// - `AlertView` is `Send` because while its alive a `AlertMut` cannot.
// - `AlertView` does not use thread-local data.
unsafe impl ::std::marker::Send for AlertView<'_> {}

impl<'msg> ::protobuf::AsView for AlertView<'msg> {
  type Proxied = Alert;
  fn as_view(&self) -> ::protobuf::View<'msg, Alert> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AlertView<'msg> {
  fn into_view<'shorter>(self) -> AlertView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Alert> for AlertView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Alert {
    let mut dst = Alert::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Alert> for AlertMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Alert {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Alert {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AlertView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for AlertMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct AlertMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Alert>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for AlertMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for AlertMut<'msg> {
  type Message = Alert;
}

impl ::std::fmt::Debug for AlertMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Alert>> for AlertMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Alert>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> AlertMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Alert> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Alert {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // active_period: repeated message transit_realtime.TimeRange
  pub fn active_period(&self) -> ::protobuf::RepeatedView<'_, super::TimeRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TimeRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn active_period_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TimeRange> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_active_period(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TimeRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // informed_entity: repeated message transit_realtime.EntitySelector
  pub fn informed_entity(&self) -> ::protobuf::RepeatedView<'_, super::EntitySelector> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EntitySelector>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn informed_entity_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EntitySelector> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_informed_entity(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EntitySelector>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // cause: optional enum transit_realtime.Alert.Cause
  pub fn has_cause(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cause(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cause_opt(&self) -> ::std::option::Option<super::alert::Cause> {
    self.has_cause().then(|| self.cause())
  }
  pub fn cause(&self) -> super::alert::Cause {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::alert::Cause::UnknownCause).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_cause(&mut self, val: super::alert::Cause) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // effect: optional enum transit_realtime.Alert.Effect
  pub fn has_effect(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_effect(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn effect_opt(&self) -> ::std::option::Option<super::alert::Effect> {
    self.has_effect().then(|| self.effect())
  }
  pub fn effect(&self) -> super::alert::Effect {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::alert::Effect::UnknownEffect).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_effect(&mut self, val: super::alert::Effect) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // url: optional message transit_realtime.TranslatedString
  pub fn has_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn url_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_url().then(|| self.url())
  }
  pub fn url(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn url_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_url(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // header_text: optional message transit_realtime.TranslatedString
  pub fn has_header_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_header_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn header_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_header_text().then(|| self.header_text())
  }
  pub fn header_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn header_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_header_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // description_text: optional message transit_realtime.TranslatedString
  pub fn has_description_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_description_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn description_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_description_text().then(|| self.description_text())
  }
  pub fn description_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn description_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_description_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // tts_header_text: optional message transit_realtime.TranslatedString
  pub fn has_tts_header_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_tts_header_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn tts_header_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_tts_header_text().then(|| self.tts_header_text())
  }
  pub fn tts_header_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn tts_header_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tts_header_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // tts_description_text: optional message transit_realtime.TranslatedString
  pub fn has_tts_description_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_tts_description_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn tts_description_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_tts_description_text().then(|| self.tts_description_text())
  }
  pub fn tts_description_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn tts_description_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tts_description_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // severity_level: optional enum transit_realtime.Alert.SeverityLevel
  pub fn has_severity_level(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_severity_level(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn severity_level_opt(&self) -> ::std::option::Option<super::alert::SeverityLevel> {
    self.has_severity_level().then(|| self.severity_level())
  }
  pub fn severity_level(&self) -> super::alert::SeverityLevel {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        9, (super::alert::SeverityLevel::UnknownSeverity).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_severity_level(&mut self, val: super::alert::SeverityLevel) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        9, val.into()
      )
    }
  }

  // image: optional message transit_realtime.TranslatedImage
  pub fn has_image(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_image(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn image_opt(&self) -> ::std::option::Option<super::TranslatedImageView<'_>> {
    self.has_image().then(|| self.image())
  }
  pub fn image(&self) -> super::TranslatedImageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedImageView::default())
  }
  pub fn image_mut(&mut self) -> super::TranslatedImageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_image(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedImage>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // image_alternative_text: optional message transit_realtime.TranslatedString
  pub fn has_image_alternative_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_image_alternative_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn image_alternative_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_image_alternative_text().then(|| self.image_alternative_text())
  }
  pub fn image_alternative_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn image_alternative_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_image_alternative_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // cause_detail: optional message transit_realtime.TranslatedString
  pub fn has_cause_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_cause_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn cause_detail_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_cause_detail().then(|| self.cause_detail())
  }
  pub fn cause_detail(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn cause_detail_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cause_detail(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // effect_detail: optional message transit_realtime.TranslatedString
  pub fn has_effect_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_effect_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn effect_detail_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_effect_detail().then(|| self.effect_detail())
  }
  pub fn effect_detail(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn effect_detail_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_effect_detail(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

}

// SAFETY:
// - `AlertMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for AlertMut<'_> {}

// SAFETY:
// - `AlertMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for AlertMut<'_> {}

impl<'msg> ::protobuf::AsView for AlertMut<'msg> {
  type Proxied = Alert;
  fn as_view(&self) -> ::protobuf::View<'_, Alert> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for AlertMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Alert>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for AlertMut<'msg> {
  type MutProxied = Alert;
  fn as_mut(&mut self) -> AlertMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for AlertMut<'msg> {
  fn into_mut<'shorter>(self) -> AlertMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Alert {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Alert> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> AlertView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> AlertMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // active_period: repeated message transit_realtime.TimeRange
  pub fn active_period(&self) -> ::protobuf::RepeatedView<'_, super::TimeRange> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TimeRange>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn active_period_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TimeRange> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_active_period(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TimeRange>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // informed_entity: repeated message transit_realtime.EntitySelector
  pub fn informed_entity(&self) -> ::protobuf::RepeatedView<'_, super::EntitySelector> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::EntitySelector>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn informed_entity_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::EntitySelector> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_informed_entity(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::EntitySelector>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // cause: optional enum transit_realtime.Alert.Cause
  pub fn has_cause(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_cause(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn cause_opt(&self) -> ::std::option::Option<super::alert::Cause> {
    self.has_cause().then(|| self.cause())
  }
  pub fn cause(&self) -> super::alert::Cause {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::alert::Cause::UnknownCause).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_cause(&mut self, val: super::alert::Cause) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // effect: optional enum transit_realtime.Alert.Effect
  pub fn has_effect(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_effect(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn effect_opt(&self) -> ::std::option::Option<super::alert::Effect> {
    self.has_effect().then(|| self.effect())
  }
  pub fn effect(&self) -> super::alert::Effect {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::alert::Effect::UnknownEffect).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_effect(&mut self, val: super::alert::Effect) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // url: optional message transit_realtime.TranslatedString
  pub fn has_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn url_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_url().then(|| self.url())
  }
  pub fn url(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn url_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_url(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // header_text: optional message transit_realtime.TranslatedString
  pub fn has_header_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_header_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn header_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_header_text().then(|| self.header_text())
  }
  pub fn header_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(5)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn header_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         5, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_header_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        5,
        val
      );
    }
  }

  // description_text: optional message transit_realtime.TranslatedString
  pub fn has_description_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_description_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn description_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_description_text().then(|| self.description_text())
  }
  pub fn description_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn description_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_description_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

  // tts_header_text: optional message transit_realtime.TranslatedString
  pub fn has_tts_header_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_tts_header_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn tts_header_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_tts_header_text().then(|| self.tts_header_text())
  }
  pub fn tts_header_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(7)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn tts_header_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         7, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tts_header_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val
      );
    }
  }

  // tts_description_text: optional message transit_realtime.TranslatedString
  pub fn has_tts_description_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_tts_description_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn tts_description_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_tts_description_text().then(|| self.tts_description_text())
  }
  pub fn tts_description_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn tts_description_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tts_description_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // severity_level: optional enum transit_realtime.Alert.SeverityLevel
  pub fn has_severity_level(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_severity_level(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn severity_level_opt(&self) -> ::std::option::Option<super::alert::SeverityLevel> {
    self.has_severity_level().then(|| self.severity_level())
  }
  pub fn severity_level(&self) -> super::alert::SeverityLevel {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        9, (super::alert::SeverityLevel::UnknownSeverity).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_severity_level(&mut self, val: super::alert::SeverityLevel) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        9, val.into()
      )
    }
  }

  // image: optional message transit_realtime.TranslatedImage
  pub fn has_image(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_image(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn image_opt(&self) -> ::std::option::Option<super::TranslatedImageView<'_>> {
    self.has_image().then(|| self.image())
  }
  pub fn image(&self) -> super::TranslatedImageView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(10)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedImageView::default())
  }
  pub fn image_mut(&mut self) -> super::TranslatedImageMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         10, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_image(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedImage>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val
      );
    }
  }

  // image_alternative_text: optional message transit_realtime.TranslatedString
  pub fn has_image_alternative_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_image_alternative_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn image_alternative_text_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_image_alternative_text().then(|| self.image_alternative_text())
  }
  pub fn image_alternative_text(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(11)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn image_alternative_text_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         11, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_image_alternative_text(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        11,
        val
      );
    }
  }

  // cause_detail: optional message transit_realtime.TranslatedString
  pub fn has_cause_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_cause_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn cause_detail_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_cause_detail().then(|| self.cause_detail())
  }
  pub fn cause_detail(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(12)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn cause_detail_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         12, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_cause_detail(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val
      );
    }
  }

  // effect_detail: optional message transit_realtime.TranslatedString
  pub fn has_effect_detail(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_effect_detail(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn effect_detail_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_effect_detail().then(|| self.effect_detail())
  }
  pub fn effect_detail(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn effect_detail_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_effect_detail(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

}  // impl Alert

impl ::std::ops::Drop for Alert {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Alert {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Alert {
  type Proxied = Self;
  fn as_view(&self) -> AlertView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Alert {
  type MutProxied = Self;
  fn as_mut(&mut self) -> AlertMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Alert {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__Alert_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$PGcG443a333343333");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__Alert_msg_init.0, &[<super::TimeRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::EntitySelector as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedImage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[<super::alert::Cause as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            <super::alert::Effect as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            <super::alert::SeverityLevel as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__Alert_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Alert {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Alert {
  type Msg = Alert;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Alert> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Alert {
  type Msg = Alert;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Alert> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for AlertMut<'_> {
  type Msg = Alert;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Alert> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlertMut<'_> {
  type Msg = Alert;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Alert> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for AlertView<'_> {
  type Msg = Alert;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Alert> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for AlertMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod alert {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cause(i32);

#[allow(non_upper_case_globals)]
impl Cause {
  pub const UnknownCause: Cause = Cause(1);
  pub const OtherCause: Cause = Cause(2);
  pub const TechnicalProblem: Cause = Cause(3);
  pub const Strike: Cause = Cause(4);
  pub const Demonstration: Cause = Cause(5);
  pub const Accident: Cause = Cause(6);
  pub const Holiday: Cause = Cause(7);
  pub const Weather: Cause = Cause(8);
  pub const Maintenance: Cause = Cause(9);
  pub const Construction: Cause = Cause(10);
  pub const PoliceActivity: Cause = Cause(11);
  pub const MedicalEmergency: Cause = Cause(12);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      1 => "UnknownCause",
      2 => "OtherCause",
      3 => "TechnicalProblem",
      4 => "Strike",
      5 => "Demonstration",
      6 => "Accident",
      7 => "Holiday",
      8 => "Weather",
      9 => "Maintenance",
      10 => "Construction",
      11 => "PoliceActivity",
      12 => "MedicalEmergency",
      _ => return None
    })
  }
}

impl ::std::convert::From<Cause> for i32 {
  fn from(val: Cause) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for Cause {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<Cause, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for Cause {
  fn default() -> Self {
    Self(1)
  }
}

impl ::std::fmt::Debug for Cause {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Cause::{}", constant_name)
    } else {
      write!(f, "Cause::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Cause {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Cause {}

impl ::protobuf::Proxied for Cause {
  type View<'a> = Cause;
}

impl ::protobuf::AsView for Cause {
  type Proxied = Cause;

  fn as_view(&self) -> Cause {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Cause {
  fn into_view<'shorter>(self) -> Cause where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Cause {
  const NAME: &'static str = "Cause";

  fn is_known(value: i32) -> bool {
    matches!(value, 1|2|3|4|5|6|7|8|9|10|11|12)
  }
}

impl ::protobuf::__internal::EntityType for Cause {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for Cause {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!@A)"))
    }).0
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Effect(i32);

#[allow(non_upper_case_globals)]
impl Effect {
  pub const NoService: Effect = Effect(1);
  pub const ReducedService: Effect = Effect(2);
  pub const SignificantDelays: Effect = Effect(3);
  pub const Detour: Effect = Effect(4);
  pub const AdditionalService: Effect = Effect(5);
  pub const ModifiedService: Effect = Effect(6);
  pub const OtherEffect: Effect = Effect(7);
  pub const UnknownEffect: Effect = Effect(8);
  pub const StopMoved: Effect = Effect(9);
  pub const NoEffect: Effect = Effect(10);
  pub const AccessibilityIssue: Effect = Effect(11);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      1 => "NoService",
      2 => "ReducedService",
      3 => "SignificantDelays",
      4 => "Detour",
      5 => "AdditionalService",
      6 => "ModifiedService",
      7 => "OtherEffect",
      8 => "UnknownEffect",
      9 => "StopMoved",
      10 => "NoEffect",
      11 => "AccessibilityIssue",
      _ => return None
    })
  }
}

impl ::std::convert::From<Effect> for i32 {
  fn from(val: Effect) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for Effect {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<Effect, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for Effect {
  fn default() -> Self {
    Self(1)
  }
}

impl ::std::fmt::Debug for Effect {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Effect::{}", constant_name)
    } else {
      write!(f, "Effect::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Effect {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Effect {}

impl ::protobuf::Proxied for Effect {
  type View<'a> = Effect;
}

impl ::protobuf::AsView for Effect {
  type Proxied = Effect;

  fn as_view(&self) -> Effect {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Effect {
  fn into_view<'shorter>(self) -> Effect where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Effect {
  const NAME: &'static str = "Effect";

  fn is_known(value: i32) -> bool {
    matches!(value, 1|2|3|4|5|6|7|8|9|10|11)
  }
}

impl ::protobuf::__internal::EntityType for Effect {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for Effect {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!@A$"))
    }).0
  }
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SeverityLevel(i32);

#[allow(non_upper_case_globals)]
impl SeverityLevel {
  pub const UnknownSeverity: SeverityLevel = SeverityLevel(1);
  pub const Info: SeverityLevel = SeverityLevel(2);
  pub const Warning: SeverityLevel = SeverityLevel(3);
  pub const Severe: SeverityLevel = SeverityLevel(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      1 => "UnknownSeverity",
      2 => "Info",
      3 => "Warning",
      4 => "Severe",
      _ => return None
    })
  }
}

impl ::std::convert::From<SeverityLevel> for i32 {
  fn from(val: SeverityLevel) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for SeverityLevel {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<SeverityLevel, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for SeverityLevel {
  fn default() -> Self {
    Self(1)
  }
}

impl ::std::fmt::Debug for SeverityLevel {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "SeverityLevel::{}", constant_name)
    } else {
      write!(f, "SeverityLevel::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for SeverityLevel {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for SeverityLevel {}

impl ::protobuf::Proxied for SeverityLevel {
  type View<'a> = SeverityLevel;
}

impl ::protobuf::AsView for SeverityLevel {
  type Proxied = SeverityLevel;

  fn as_view(&self) -> SeverityLevel {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SeverityLevel {
  fn into_view<'shorter>(self) -> SeverityLevel where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for SeverityLevel {
  const NAME: &'static str = "SeverityLevel";

  fn is_known(value: i32) -> bool {
    matches!(value, 1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for SeverityLevel {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for SeverityLevel {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!@"))
    }).0
  }
}

}  // pub mod alert


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TimeRange_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TimeRange {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TimeRange>
}

impl ::protobuf::Message for TimeRange {
  type MessageView<'msg> = TimeRangeView<'msg>;
  type MessageMut<'msg> = TimeRangeMut<'msg>;
}

impl ::std::default::Default for TimeRange {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TimeRange {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TimeRange` is `Sync` because it does not implement interior mutability.
//    Neither does `TimeRangeMut`.
unsafe impl ::std::marker::Sync for TimeRange {}

// SAFETY:
// - `TimeRange` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TimeRange {}

impl ::protobuf::Proxied for TimeRange {
  type View<'msg> = TimeRangeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TimeRange {}

impl ::protobuf::MutProxied for TimeRange {
  type Mut<'msg> = TimeRangeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TimeRangeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TimeRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TimeRangeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TimeRangeView<'msg> {
  type Message = TimeRange;
}

impl ::std::fmt::Debug for TimeRangeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TimeRangeView<'_> {
  fn default() -> TimeRangeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TimeRange>> for TimeRangeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TimeRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TimeRangeView<'msg> {

  pub fn to_owned(&self) -> TimeRange {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // start: optional uint64
  pub fn has_start(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn start_opt(self) -> ::std::option::Option<u64> {
    self.has_start().then(|| self.start())
  }
  pub fn start(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }

  // end: optional uint64
  pub fn has_end(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn end_opt(self) -> ::std::option::Option<u64> {
    self.has_end().then(|| self.end())
  }
  pub fn end(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `TimeRangeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TimeRangeView<'_> {}

// SAFETY:
// - `TimeRangeView` is `Send` because while its alive a `TimeRangeMut` cannot.
// - `TimeRangeView` does not use thread-local data.
unsafe impl ::std::marker::Send for TimeRangeView<'_> {}

impl<'msg> ::protobuf::AsView for TimeRangeView<'msg> {
  type Proxied = TimeRange;
  fn as_view(&self) -> ::protobuf::View<'msg, TimeRange> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimeRangeView<'msg> {
  fn into_view<'shorter>(self) -> TimeRangeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TimeRange> for TimeRangeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TimeRange {
    let mut dst = TimeRange::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TimeRange> for TimeRangeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TimeRange {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TimeRange {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TimeRangeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TimeRangeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TimeRangeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TimeRange>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TimeRangeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TimeRangeMut<'msg> {
  type Message = TimeRange;
}

impl ::std::fmt::Debug for TimeRangeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TimeRange>> for TimeRangeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TimeRange>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TimeRangeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TimeRange> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TimeRange {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // start: optional uint64
  pub fn has_start(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_start(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn start_opt(&self) -> ::std::option::Option<u64> {
    self.has_start().then(|| self.start())
  }
  pub fn start(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // end: optional uint64
  pub fn has_end(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_end(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn end_opt(&self) -> ::std::option::Option<u64> {
    self.has_end().then(|| self.end())
  }
  pub fn end(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}

// SAFETY:
// - `TimeRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TimeRangeMut<'_> {}

// SAFETY:
// - `TimeRangeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TimeRangeMut<'_> {}

impl<'msg> ::protobuf::AsView for TimeRangeMut<'msg> {
  type Proxied = TimeRange;
  fn as_view(&self) -> ::protobuf::View<'_, TimeRange> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TimeRangeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TimeRange>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TimeRangeMut<'msg> {
  type MutProxied = TimeRange;
  fn as_mut(&mut self) -> TimeRangeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TimeRangeMut<'msg> {
  fn into_mut<'shorter>(self) -> TimeRangeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TimeRange {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TimeRange> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TimeRangeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TimeRangeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // start: optional uint64
  pub fn has_start(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_start(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn start_opt(&self) -> ::std::option::Option<u64> {
    self.has_start().then(|| self.start())
  }
  pub fn start(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        0, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_start(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        0, val.into()
      )
    }
  }

  // end: optional uint64
  pub fn has_end(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_end(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn end_opt(&self) -> ::std::option::Option<u64> {
    self.has_end().then(|| self.end())
  }
  pub fn end(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        1, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_end(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        1, val.into()
      )
    }
  }

}  // impl TimeRange

impl ::std::ops::Drop for TimeRange {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TimeRange {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TimeRange {
  type Proxied = Self;
  fn as_view(&self) -> TimeRangeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TimeRange {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TimeRangeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TimeRange {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TimeRange_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P,,");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TimeRange_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TimeRange_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TimeRange {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TimeRange {
  type Msg = TimeRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimeRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimeRange {
  type Msg = TimeRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimeRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TimeRangeMut<'_> {
  type Msg = TimeRange;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimeRange> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimeRangeMut<'_> {
  type Msg = TimeRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimeRange> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TimeRangeView<'_> {
  type Msg = TimeRange;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TimeRange> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TimeRangeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__Position_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Position {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Position>
}

impl ::protobuf::Message for Position {
  type MessageView<'msg> = PositionView<'msg>;
  type MessageMut<'msg> = PositionMut<'msg>;
}

impl ::std::default::Default for Position {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Position {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Position` is `Sync` because it does not implement interior mutability.
//    Neither does `PositionMut`.
unsafe impl ::std::marker::Sync for Position {}

// SAFETY:
// - `Position` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Position {}

impl ::protobuf::Proxied for Position {
  type View<'msg> = PositionView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Position {}

impl ::protobuf::MutProxied for Position {
  type Mut<'msg> = PositionMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct PositionView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Position>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PositionView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for PositionView<'msg> {
  type Message = Position;
}

impl ::std::fmt::Debug for PositionView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for PositionView<'_> {
  fn default() -> PositionView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Position>> for PositionView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Position>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PositionView<'msg> {

  pub fn to_owned(&self) -> Position {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // latitude: optional float
  pub fn has_latitude(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn latitude_opt(self) -> ::std::option::Option<f32> {
    self.has_latitude().then(|| self.latitude())
  }
  pub fn latitude(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // longitude: optional float
  pub fn has_longitude(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn longitude_opt(self) -> ::std::option::Option<f32> {
    self.has_longitude().then(|| self.longitude())
  }
  pub fn longitude(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // bearing: optional float
  pub fn has_bearing(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn bearing_opt(self) -> ::std::option::Option<f32> {
    self.has_bearing().then(|| self.bearing())
  }
  pub fn bearing(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // odometer: optional double
  pub fn has_odometer(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn odometer_opt(self) -> ::std::option::Option<f64> {
    self.has_odometer().then(|| self.odometer())
  }
  pub fn odometer(self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }

  // speed: optional float
  pub fn has_speed(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn speed_opt(self) -> ::std::option::Option<f32> {
    self.has_speed().then(|| self.speed())
  }
  pub fn speed(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `PositionView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for PositionView<'_> {}

// SAFETY:
// - `PositionView` is `Send` because while its alive a `PositionMut` cannot.
// - `PositionView` does not use thread-local data.
unsafe impl ::std::marker::Send for PositionView<'_> {}

impl<'msg> ::protobuf::AsView for PositionView<'msg> {
  type Proxied = Position;
  fn as_view(&self) -> ::protobuf::View<'msg, Position> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PositionView<'msg> {
  fn into_view<'shorter>(self) -> PositionView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Position> for PositionView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Position {
    let mut dst = Position::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Position> for PositionMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Position {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Position {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PositionView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for PositionMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PositionMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Position>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for PositionMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for PositionMut<'msg> {
  type Message = Position;
}

impl ::std::fmt::Debug for PositionMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Position>> for PositionMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Position>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> PositionMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Position> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Position {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // latitude: optional float
  pub fn has_latitude(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_latitude(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn latitude_opt(&self) -> ::std::option::Option<f32> {
    self.has_latitude().then(|| self.latitude())
  }
  pub fn latitude(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_latitude(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // longitude: optional float
  pub fn has_longitude(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_longitude(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn longitude_opt(&self) -> ::std::option::Option<f32> {
    self.has_longitude().then(|| self.longitude())
  }
  pub fn longitude(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_longitude(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // bearing: optional float
  pub fn has_bearing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_bearing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn bearing_opt(&self) -> ::std::option::Option<f32> {
    self.has_bearing().then(|| self.bearing())
  }
  pub fn bearing(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bearing(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

  // odometer: optional double
  pub fn has_odometer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_odometer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn odometer_opt(&self) -> ::std::option::Option<f64> {
    self.has_odometer().then(|| self.odometer())
  }
  pub fn odometer(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_odometer(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  // speed: optional float
  pub fn has_speed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_speed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn speed_opt(&self) -> ::std::option::Option<f32> {
    self.has_speed().then(|| self.speed())
  }
  pub fn speed(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_speed(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

}

// SAFETY:
// - `PositionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for PositionMut<'_> {}

// SAFETY:
// - `PositionMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for PositionMut<'_> {}

impl<'msg> ::protobuf::AsView for PositionMut<'msg> {
  type Proxied = Position;
  fn as_view(&self) -> ::protobuf::View<'_, Position> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for PositionMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Position>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for PositionMut<'msg> {
  type MutProxied = Position;
  fn as_mut(&mut self) -> PositionMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for PositionMut<'msg> {
  fn into_mut<'shorter>(self) -> PositionMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Position {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Position> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> PositionView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> PositionMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // latitude: optional float
  pub fn has_latitude(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_latitude(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn latitude_opt(&self) -> ::std::option::Option<f32> {
    self.has_latitude().then(|| self.latitude())
  }
  pub fn latitude(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        0, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_latitude(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        0, val.into()
      )
    }
  }

  // longitude: optional float
  pub fn has_longitude(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_longitude(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn longitude_opt(&self) -> ::std::option::Option<f32> {
    self.has_longitude().then(|| self.longitude())
  }
  pub fn longitude(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        1, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_longitude(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        1, val.into()
      )
    }
  }

  // bearing: optional float
  pub fn has_bearing(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_bearing(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn bearing_opt(&self) -> ::std::option::Option<f32> {
    self.has_bearing().then(|| self.bearing())
  }
  pub fn bearing(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        2, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_bearing(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        2, val.into()
      )
    }
  }

  // odometer: optional double
  pub fn has_odometer(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_odometer(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn odometer_opt(&self) -> ::std::option::Option<f64> {
    self.has_odometer().then(|| self.odometer())
  }
  pub fn odometer(&self) -> f64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f64_at_index(
        3, (0f64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_odometer(&mut self, val: f64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f64_at_index(
        3, val.into()
      )
    }
  }

  // speed: optional float
  pub fn has_speed(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_speed(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn speed_opt(&self) -> ::std::option::Option<f32> {
    self.has_speed().then(|| self.speed())
  }
  pub fn speed(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        4, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_speed(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        4, val.into()
      )
    }
  }

}  // impl Position

impl ::std::ops::Drop for Position {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Position {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Position {
  type Proxied = Self;
  fn as_view(&self) -> PositionView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Position {
  type MutProxied = Self;
  fn as_mut(&mut self) -> PositionMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Position {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__Position_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P!N!N! !");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__Position_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__Position_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Position {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Position {
  type Msg = Position;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Position> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Position {
  type Msg = Position;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Position> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for PositionMut<'_> {
  type Msg = Position;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Position> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PositionMut<'_> {
  type Msg = Position;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Position> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for PositionView<'_> {
  type Msg = Position;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Position> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for PositionMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripDescriptor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TripDescriptor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TripDescriptor>
}

impl ::protobuf::Message for TripDescriptor {
  type MessageView<'msg> = TripDescriptorView<'msg>;
  type MessageMut<'msg> = TripDescriptorMut<'msg>;
}

impl ::std::default::Default for TripDescriptor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TripDescriptor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TripDescriptor` is `Sync` because it does not implement interior mutability.
//    Neither does `TripDescriptorMut`.
unsafe impl ::std::marker::Sync for TripDescriptor {}

// SAFETY:
// - `TripDescriptor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TripDescriptor {}

impl ::protobuf::Proxied for TripDescriptor {
  type View<'msg> = TripDescriptorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TripDescriptor {}

impl ::protobuf::MutProxied for TripDescriptor {
  type Mut<'msg> = TripDescriptorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TripDescriptorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripDescriptorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TripDescriptorView<'msg> {
  type Message = TripDescriptor;
}

impl ::std::fmt::Debug for TripDescriptorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TripDescriptorView<'_> {
  fn default() -> TripDescriptorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TripDescriptor>> for TripDescriptorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripDescriptorView<'msg> {

  pub fn to_owned(&self) -> TripDescriptor {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trip_id: optional string
  pub fn has_trip_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn trip_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_trip_id().then(|| self.trip_id())
  }
  pub fn trip_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_id: optional string
  pub fn has_route_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn route_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // direction_id: optional uint32
  pub fn has_direction_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn direction_id_opt(self) -> ::std::option::Option<u32> {
    self.has_direction_id().then(|| self.direction_id())
  }
  pub fn direction_id(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // start_time: optional string
  pub fn has_start_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn start_time_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // start_date: optional string
  pub fn has_start_date(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn start_date_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // schedule_relationship: optional enum transit_realtime.TripDescriptor.ScheduleRelationship
  pub fn has_schedule_relationship(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn schedule_relationship_opt(self) -> ::std::option::Option<super::trip_descriptor::ScheduleRelationship> {
    self.has_schedule_relationship().then(|| self.schedule_relationship())
  }
  pub fn schedule_relationship(self) -> super::trip_descriptor::ScheduleRelationship {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::trip_descriptor::ScheduleRelationship::Scheduled).into()
      ).try_into().unwrap()
    }
  }

  // modified_trip: optional message transit_realtime.TripDescriptor.ModifiedTripSelector
  pub fn has_modified_trip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn modified_trip_opt(self) -> ::std::option::Option<super::trip_descriptor::ModifiedTripSelectorView<'msg>> {
    self.has_modified_trip().then(|| self.modified_trip())
  }
  pub fn modified_trip(self) -> super::trip_descriptor::ModifiedTripSelectorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::trip_descriptor::ModifiedTripSelectorView::default())
  }

}

// SAFETY:
// - `TripDescriptorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TripDescriptorView<'_> {}

// SAFETY:
// - `TripDescriptorView` is `Send` because while its alive a `TripDescriptorMut` cannot.
// - `TripDescriptorView` does not use thread-local data.
unsafe impl ::std::marker::Send for TripDescriptorView<'_> {}

impl<'msg> ::protobuf::AsView for TripDescriptorView<'msg> {
  type Proxied = TripDescriptor;
  fn as_view(&self) -> ::protobuf::View<'msg, TripDescriptor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripDescriptorView<'msg> {
  fn into_view<'shorter>(self) -> TripDescriptorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TripDescriptor> for TripDescriptorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripDescriptor {
    let mut dst = TripDescriptor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TripDescriptor> for TripDescriptorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripDescriptor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TripDescriptor {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripDescriptorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripDescriptorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TripDescriptorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripDescriptorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TripDescriptorMut<'msg> {
  type Message = TripDescriptor;
}

impl ::std::fmt::Debug for TripDescriptorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TripDescriptor>> for TripDescriptorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripDescriptorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TripDescriptor> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TripDescriptor {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trip_id: optional string
  pub fn has_trip_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_id().then(|| self.trip_id())
  }
  pub fn trip_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // route_id: optional string
  pub fn has_route_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_route_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn route_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // direction_id: optional uint32
  pub fn has_direction_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_direction_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn direction_id_opt(&self) -> ::std::option::Option<u32> {
    self.has_direction_id().then(|| self.direction_id())
  }
  pub fn direction_id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_direction_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

  // start_time: optional string
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // start_date: optional string
  pub fn has_start_date(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start_date(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_date_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // schedule_relationship: optional enum transit_realtime.TripDescriptor.ScheduleRelationship
  pub fn has_schedule_relationship(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_schedule_relationship(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn schedule_relationship_opt(&self) -> ::std::option::Option<super::trip_descriptor::ScheduleRelationship> {
    self.has_schedule_relationship().then(|| self.schedule_relationship())
  }
  pub fn schedule_relationship(&self) -> super::trip_descriptor::ScheduleRelationship {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::trip_descriptor::ScheduleRelationship::Scheduled).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schedule_relationship(&mut self, val: super::trip_descriptor::ScheduleRelationship) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // modified_trip: optional message transit_realtime.TripDescriptor.ModifiedTripSelector
  pub fn has_modified_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_modified_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn modified_trip_opt(&self) -> ::std::option::Option<super::trip_descriptor::ModifiedTripSelectorView<'_>> {
    self.has_modified_trip().then(|| self.modified_trip())
  }
  pub fn modified_trip(&self) -> super::trip_descriptor::ModifiedTripSelectorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::trip_descriptor::ModifiedTripSelectorView::default())
  }
  pub fn modified_trip_mut(&mut self) -> super::trip_descriptor::ModifiedTripSelectorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_modified_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::trip_descriptor::ModifiedTripSelector>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}

// SAFETY:
// - `TripDescriptorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TripDescriptorMut<'_> {}

// SAFETY:
// - `TripDescriptorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TripDescriptorMut<'_> {}

impl<'msg> ::protobuf::AsView for TripDescriptorMut<'msg> {
  type Proxied = TripDescriptor;
  fn as_view(&self) -> ::protobuf::View<'_, TripDescriptor> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripDescriptorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TripDescriptor>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TripDescriptorMut<'msg> {
  type MutProxied = TripDescriptor;
  fn as_mut(&mut self) -> TripDescriptorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TripDescriptorMut<'msg> {
  fn into_mut<'shorter>(self) -> TripDescriptorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TripDescriptor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TripDescriptor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TripDescriptorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TripDescriptorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trip_id: optional string
  pub fn has_trip_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_trip_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn trip_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_trip_id().then(|| self.trip_id())
  }
  pub fn trip_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_trip_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // route_id: optional string
  pub fn has_route_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_route_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn route_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // direction_id: optional uint32
  pub fn has_direction_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_direction_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn direction_id_opt(&self) -> ::std::option::Option<u32> {
    self.has_direction_id().then(|| self.direction_id())
  }
  pub fn direction_id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_direction_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

  // start_time: optional string
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // start_date: optional string
  pub fn has_start_date(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start_date(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_date_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // schedule_relationship: optional enum transit_realtime.TripDescriptor.ScheduleRelationship
  pub fn has_schedule_relationship(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_schedule_relationship(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn schedule_relationship_opt(&self) -> ::std::option::Option<super::trip_descriptor::ScheduleRelationship> {
    self.has_schedule_relationship().then(|| self.schedule_relationship())
  }
  pub fn schedule_relationship(&self) -> super::trip_descriptor::ScheduleRelationship {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::trip_descriptor::ScheduleRelationship::Scheduled).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_schedule_relationship(&mut self, val: super::trip_descriptor::ScheduleRelationship) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

  // modified_trip: optional message transit_realtime.TripDescriptor.ModifiedTripSelector
  pub fn has_modified_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_modified_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn modified_trip_opt(&self) -> ::std::option::Option<super::trip_descriptor::ModifiedTripSelectorView<'_>> {
    self.has_modified_trip().then(|| self.modified_trip())
  }
  pub fn modified_trip(&self) -> super::trip_descriptor::ModifiedTripSelectorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(6)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::trip_descriptor::ModifiedTripSelectorView::default())
  }
  pub fn modified_trip_mut(&mut self) -> super::trip_descriptor::ModifiedTripSelectorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         6, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_modified_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::trip_descriptor::ModifiedTripSelector>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        6,
        val
      );
    }
  }

}  // impl TripDescriptor

impl ::std::ops::Drop for TripDescriptor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TripDescriptor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TripDescriptor {
  type Proxied = Self;
  fn as_view(&self) -> TripDescriptorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TripDescriptor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TripDescriptorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TripDescriptor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TripDescriptor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P11141)3");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TripDescriptor_msg_init.0, &[<super::trip_descriptor::ModifiedTripSelector as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[<super::trip_descriptor::ScheduleRelationship as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TripDescriptor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripDescriptor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripDescriptor {
  type Msg = TripDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripDescriptor {
  type Msg = TripDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripDescriptorMut<'_> {
  type Msg = TripDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripDescriptorMut<'_> {
  type Msg = TripDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripDescriptorView<'_> {
  type Msg = TripDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripDescriptor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripDescriptorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod trip_descriptor {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripDescriptor__ModifiedTripSelector_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ModifiedTripSelector {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ModifiedTripSelector>
}

impl ::protobuf::Message for ModifiedTripSelector {
  type MessageView<'msg> = ModifiedTripSelectorView<'msg>;
  type MessageMut<'msg> = ModifiedTripSelectorMut<'msg>;
}

impl ::std::default::Default for ModifiedTripSelector {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ModifiedTripSelector {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ModifiedTripSelector` is `Sync` because it does not implement interior mutability.
//    Neither does `ModifiedTripSelectorMut`.
unsafe impl ::std::marker::Sync for ModifiedTripSelector {}

// SAFETY:
// - `ModifiedTripSelector` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ModifiedTripSelector {}

impl ::protobuf::Proxied for ModifiedTripSelector {
  type View<'msg> = ModifiedTripSelectorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ModifiedTripSelector {}

impl ::protobuf::MutProxied for ModifiedTripSelector {
  type Mut<'msg> = ModifiedTripSelectorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ModifiedTripSelectorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ModifiedTripSelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModifiedTripSelectorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ModifiedTripSelectorView<'msg> {
  type Message = ModifiedTripSelector;
}

impl ::std::fmt::Debug for ModifiedTripSelectorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ModifiedTripSelectorView<'_> {
  fn default() -> ModifiedTripSelectorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ModifiedTripSelector>> for ModifiedTripSelectorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ModifiedTripSelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModifiedTripSelectorView<'msg> {

  pub fn to_owned(&self) -> ModifiedTripSelector {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // modifications_id: optional string
  pub fn has_modifications_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn modifications_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_modifications_id().then(|| self.modifications_id())
  }
  pub fn modifications_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // affected_trip_id: optional string
  pub fn has_affected_trip_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn affected_trip_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_affected_trip_id().then(|| self.affected_trip_id())
  }
  pub fn affected_trip_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // start_time: optional string
  pub fn has_start_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn start_time_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // start_date: optional string
  pub fn has_start_date(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn start_date_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ModifiedTripSelectorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ModifiedTripSelectorView<'_> {}

// SAFETY:
// - `ModifiedTripSelectorView` is `Send` because while its alive a `ModifiedTripSelectorMut` cannot.
// - `ModifiedTripSelectorView` does not use thread-local data.
unsafe impl ::std::marker::Send for ModifiedTripSelectorView<'_> {}

impl<'msg> ::protobuf::AsView for ModifiedTripSelectorView<'msg> {
  type Proxied = ModifiedTripSelector;
  fn as_view(&self) -> ::protobuf::View<'msg, ModifiedTripSelector> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModifiedTripSelectorView<'msg> {
  fn into_view<'shorter>(self) -> ModifiedTripSelectorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ModifiedTripSelector> for ModifiedTripSelectorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ModifiedTripSelector {
    let mut dst = ModifiedTripSelector::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ModifiedTripSelector> for ModifiedTripSelectorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ModifiedTripSelector {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ModifiedTripSelector {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ModifiedTripSelectorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ModifiedTripSelectorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ModifiedTripSelectorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifiedTripSelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModifiedTripSelectorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ModifiedTripSelectorMut<'msg> {
  type Message = ModifiedTripSelector;
}

impl ::std::fmt::Debug for ModifiedTripSelectorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ModifiedTripSelector>> for ModifiedTripSelectorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifiedTripSelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModifiedTripSelectorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ModifiedTripSelector> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ModifiedTripSelector {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // modifications_id: optional string
  pub fn has_modifications_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_modifications_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn modifications_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_modifications_id().then(|| self.modifications_id())
  }
  pub fn modifications_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_modifications_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // affected_trip_id: optional string
  pub fn has_affected_trip_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_affected_trip_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn affected_trip_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_affected_trip_id().then(|| self.affected_trip_id())
  }
  pub fn affected_trip_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_affected_trip_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // start_time: optional string
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // start_date: optional string
  pub fn has_start_date(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_start_date(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn start_date_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}

// SAFETY:
// - `ModifiedTripSelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ModifiedTripSelectorMut<'_> {}

// SAFETY:
// - `ModifiedTripSelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ModifiedTripSelectorMut<'_> {}

impl<'msg> ::protobuf::AsView for ModifiedTripSelectorMut<'msg> {
  type Proxied = ModifiedTripSelector;
  fn as_view(&self) -> ::protobuf::View<'_, ModifiedTripSelector> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModifiedTripSelectorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ModifiedTripSelector>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ModifiedTripSelectorMut<'msg> {
  type MutProxied = ModifiedTripSelector;
  fn as_mut(&mut self) -> ModifiedTripSelectorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ModifiedTripSelectorMut<'msg> {
  fn into_mut<'shorter>(self) -> ModifiedTripSelectorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ModifiedTripSelector {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ModifiedTripSelector> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ModifiedTripSelectorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ModifiedTripSelectorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // modifications_id: optional string
  pub fn has_modifications_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_modifications_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn modifications_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_modifications_id().then(|| self.modifications_id())
  }
  pub fn modifications_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_modifications_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // affected_trip_id: optional string
  pub fn has_affected_trip_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_affected_trip_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn affected_trip_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_affected_trip_id().then(|| self.affected_trip_id())
  }
  pub fn affected_trip_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_affected_trip_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // start_time: optional string
  pub fn has_start_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_start_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn start_time_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_time().then(|| self.start_time())
  }
  pub fn start_time(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_time(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // start_date: optional string
  pub fn has_start_date(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_start_date(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn start_date_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_start_date().then(|| self.start_date())
  }
  pub fn start_date(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        3, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_start_date(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val);
    }
  }

}  // impl ModifiedTripSelector

impl ::std::ops::Drop for ModifiedTripSelector {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ModifiedTripSelector {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ModifiedTripSelector {
  type Proxied = Self;
  fn as_view(&self) -> ModifiedTripSelectorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ModifiedTripSelector {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ModifiedTripSelectorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ModifiedTripSelector {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::trip_descriptor::transit_0realtime__TripDescriptor__ModifiedTripSelector_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1111");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::trip_descriptor::transit_0realtime__TripDescriptor__ModifiedTripSelector_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::trip_descriptor::transit_0realtime__TripDescriptor__ModifiedTripSelector_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModifiedTripSelector {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModifiedTripSelector {
  type Msg = ModifiedTripSelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifiedTripSelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifiedTripSelector {
  type Msg = ModifiedTripSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifiedTripSelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModifiedTripSelectorMut<'_> {
  type Msg = ModifiedTripSelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifiedTripSelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifiedTripSelectorMut<'_> {
  type Msg = ModifiedTripSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifiedTripSelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModifiedTripSelectorView<'_> {
  type Msg = ModifiedTripSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ModifiedTripSelector> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModifiedTripSelectorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ScheduleRelationship(i32);

#[allow(non_upper_case_globals)]
impl ScheduleRelationship {
  pub const Scheduled: ScheduleRelationship = ScheduleRelationship(0);
  pub const Added: ScheduleRelationship = ScheduleRelationship(1);
  pub const Unscheduled: ScheduleRelationship = ScheduleRelationship(2);
  pub const Canceled: ScheduleRelationship = ScheduleRelationship(3);
  pub const Replacement: ScheduleRelationship = ScheduleRelationship(5);
  pub const Duplicated: ScheduleRelationship = ScheduleRelationship(6);
  pub const Deleted: ScheduleRelationship = ScheduleRelationship(7);
  pub const New: ScheduleRelationship = ScheduleRelationship(8);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Scheduled",
      1 => "Added",
      2 => "Unscheduled",
      3 => "Canceled",
      5 => "Replacement",
      6 => "Duplicated",
      7 => "Deleted",
      8 => "New",
      _ => return None
    })
  }
}

impl ::std::convert::From<ScheduleRelationship> for i32 {
  fn from(val: ScheduleRelationship) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for ScheduleRelationship {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<ScheduleRelationship, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for ScheduleRelationship {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for ScheduleRelationship {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "ScheduleRelationship::{}", constant_name)
    } else {
      write!(f, "ScheduleRelationship::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for ScheduleRelationship {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for ScheduleRelationship {}

impl ::protobuf::Proxied for ScheduleRelationship {
  type View<'a> = ScheduleRelationship;
}

impl ::protobuf::AsView for ScheduleRelationship {
  type Proxied = ScheduleRelationship;

  fn as_view(&self) -> ScheduleRelationship {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ScheduleRelationship {
  fn into_view<'shorter>(self) -> ScheduleRelationship where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for ScheduleRelationship {
  const NAME: &'static str = "ScheduleRelationship";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3|5|6|7|8)
  }
}

impl ::protobuf::__internal::EntityType for ScheduleRelationship {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for ScheduleRelationship {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!11"))
    }).0
  }
}

}  // pub mod trip_descriptor


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__VehicleDescriptor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct VehicleDescriptor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<VehicleDescriptor>
}

impl ::protobuf::Message for VehicleDescriptor {
  type MessageView<'msg> = VehicleDescriptorView<'msg>;
  type MessageMut<'msg> = VehicleDescriptorMut<'msg>;
}

impl ::std::default::Default for VehicleDescriptor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for VehicleDescriptor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `VehicleDescriptor` is `Sync` because it does not implement interior mutability.
//    Neither does `VehicleDescriptorMut`.
unsafe impl ::std::marker::Sync for VehicleDescriptor {}

// SAFETY:
// - `VehicleDescriptor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for VehicleDescriptor {}

impl ::protobuf::Proxied for VehicleDescriptor {
  type View<'msg> = VehicleDescriptorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for VehicleDescriptor {}

impl ::protobuf::MutProxied for VehicleDescriptor {
  type Mut<'msg> = VehicleDescriptorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct VehicleDescriptorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, VehicleDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VehicleDescriptorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for VehicleDescriptorView<'msg> {
  type Message = VehicleDescriptor;
}

impl ::std::fmt::Debug for VehicleDescriptorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for VehicleDescriptorView<'_> {
  fn default() -> VehicleDescriptorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, VehicleDescriptor>> for VehicleDescriptorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, VehicleDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VehicleDescriptorView<'msg> {

  pub fn to_owned(&self) -> VehicleDescriptor {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // id: optional string
  pub fn has_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // label: optional string
  pub fn has_label(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn label_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_label().then(|| self.label())
  }
  pub fn label(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // license_plate: optional string
  pub fn has_license_plate(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn license_plate_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_license_plate().then(|| self.license_plate())
  }
  pub fn license_plate(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // wheelchair_accessible: optional enum transit_realtime.VehicleDescriptor.WheelchairAccessible
  pub fn has_wheelchair_accessible(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn wheelchair_accessible_opt(self) -> ::std::option::Option<super::vehicle_descriptor::WheelchairAccessible> {
    self.has_wheelchair_accessible().then(|| self.wheelchair_accessible())
  }
  pub fn wheelchair_accessible(self) -> super::vehicle_descriptor::WheelchairAccessible {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::vehicle_descriptor::WheelchairAccessible::NoValue).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `VehicleDescriptorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for VehicleDescriptorView<'_> {}

// SAFETY:
// - `VehicleDescriptorView` is `Send` because while its alive a `VehicleDescriptorMut` cannot.
// - `VehicleDescriptorView` does not use thread-local data.
unsafe impl ::std::marker::Send for VehicleDescriptorView<'_> {}

impl<'msg> ::protobuf::AsView for VehicleDescriptorView<'msg> {
  type Proxied = VehicleDescriptor;
  fn as_view(&self) -> ::protobuf::View<'msg, VehicleDescriptor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehicleDescriptorView<'msg> {
  fn into_view<'shorter>(self) -> VehicleDescriptorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<VehicleDescriptor> for VehicleDescriptorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VehicleDescriptor {
    let mut dst = VehicleDescriptor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<VehicleDescriptor> for VehicleDescriptorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> VehicleDescriptor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for VehicleDescriptor {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VehicleDescriptorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for VehicleDescriptorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct VehicleDescriptorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, VehicleDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for VehicleDescriptorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for VehicleDescriptorMut<'msg> {
  type Message = VehicleDescriptor;
}

impl ::std::fmt::Debug for VehicleDescriptorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, VehicleDescriptor>> for VehicleDescriptorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, VehicleDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> VehicleDescriptorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, VehicleDescriptor> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> VehicleDescriptor {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // id: optional string
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // label: optional string
  pub fn has_label(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_label(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn label_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_label().then(|| self.label())
  }
  pub fn label(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_label(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // license_plate: optional string
  pub fn has_license_plate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_license_plate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn license_plate_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_license_plate().then(|| self.license_plate())
  }
  pub fn license_plate(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_license_plate(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // wheelchair_accessible: optional enum transit_realtime.VehicleDescriptor.WheelchairAccessible
  pub fn has_wheelchair_accessible(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_wheelchair_accessible(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn wheelchair_accessible_opt(&self) -> ::std::option::Option<super::vehicle_descriptor::WheelchairAccessible> {
    self.has_wheelchair_accessible().then(|| self.wheelchair_accessible())
  }
  pub fn wheelchair_accessible(&self) -> super::vehicle_descriptor::WheelchairAccessible {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::vehicle_descriptor::WheelchairAccessible::NoValue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_wheelchair_accessible(&mut self, val: super::vehicle_descriptor::WheelchairAccessible) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}

// SAFETY:
// - `VehicleDescriptorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for VehicleDescriptorMut<'_> {}

// SAFETY:
// - `VehicleDescriptorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for VehicleDescriptorMut<'_> {}

impl<'msg> ::protobuf::AsView for VehicleDescriptorMut<'msg> {
  type Proxied = VehicleDescriptor;
  fn as_view(&self) -> ::protobuf::View<'_, VehicleDescriptor> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for VehicleDescriptorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, VehicleDescriptor>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for VehicleDescriptorMut<'msg> {
  type MutProxied = VehicleDescriptor;
  fn as_mut(&mut self) -> VehicleDescriptorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for VehicleDescriptorMut<'msg> {
  fn into_mut<'shorter>(self) -> VehicleDescriptorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl VehicleDescriptor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, VehicleDescriptor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> VehicleDescriptorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> VehicleDescriptorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // id: optional string
  pub fn has_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_id().then(|| self.id())
  }
  pub fn id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // label: optional string
  pub fn has_label(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_label(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn label_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_label().then(|| self.label())
  }
  pub fn label(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_label(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // license_plate: optional string
  pub fn has_license_plate(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_license_plate(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn license_plate_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_license_plate().then(|| self.license_plate())
  }
  pub fn license_plate(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_license_plate(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

  // wheelchair_accessible: optional enum transit_realtime.VehicleDescriptor.WheelchairAccessible
  pub fn has_wheelchair_accessible(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_wheelchair_accessible(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn wheelchair_accessible_opt(&self) -> ::std::option::Option<super::vehicle_descriptor::WheelchairAccessible> {
    self.has_wheelchair_accessible().then(|| self.wheelchair_accessible())
  }
  pub fn wheelchair_accessible(&self) -> super::vehicle_descriptor::WheelchairAccessible {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        3, (super::vehicle_descriptor::WheelchairAccessible::NoValue).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_wheelchair_accessible(&mut self, val: super::vehicle_descriptor::WheelchairAccessible) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        3, val.into()
      )
    }
  }

}  // impl VehicleDescriptor

impl ::std::ops::Drop for VehicleDescriptor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for VehicleDescriptor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for VehicleDescriptor {
  type Proxied = Self;
  fn as_view(&self) -> VehicleDescriptorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for VehicleDescriptor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> VehicleDescriptorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for VehicleDescriptor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__VehicleDescriptor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1114");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__VehicleDescriptor_msg_init.0, &[], &[<super::vehicle_descriptor::WheelchairAccessible as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__VehicleDescriptor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VehicleDescriptor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VehicleDescriptor {
  type Msg = VehicleDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehicleDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehicleDescriptor {
  type Msg = VehicleDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehicleDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for VehicleDescriptorMut<'_> {
  type Msg = VehicleDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehicleDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehicleDescriptorMut<'_> {
  type Msg = VehicleDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehicleDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for VehicleDescriptorView<'_> {
  type Msg = VehicleDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<VehicleDescriptor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for VehicleDescriptorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod vehicle_descriptor {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WheelchairAccessible(i32);

#[allow(non_upper_case_globals)]
impl WheelchairAccessible {
  pub const NoValue: WheelchairAccessible = WheelchairAccessible(0);
  pub const Unknown: WheelchairAccessible = WheelchairAccessible(1);
  pub const WheelchairAccessible: WheelchairAccessible = WheelchairAccessible(2);
  pub const WheelchairInaccessible: WheelchairAccessible = WheelchairAccessible(3);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "NoValue",
      1 => "Unknown",
      2 => "WheelchairAccessible",
      3 => "WheelchairInaccessible",
      _ => return None
    })
  }
}

impl ::std::convert::From<WheelchairAccessible> for i32 {
  fn from(val: WheelchairAccessible) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for WheelchairAccessible {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<WheelchairAccessible, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for WheelchairAccessible {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for WheelchairAccessible {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "WheelchairAccessible::{}", constant_name)
    } else {
      write!(f, "WheelchairAccessible::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for WheelchairAccessible {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for WheelchairAccessible {}

impl ::protobuf::Proxied for WheelchairAccessible {
  type View<'a> = WheelchairAccessible;
}

impl ::protobuf::AsView for WheelchairAccessible {
  type Proxied = WheelchairAccessible;

  fn as_view(&self) -> WheelchairAccessible {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WheelchairAccessible {
  fn into_view<'shorter>(self) -> WheelchairAccessible where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for WheelchairAccessible {
  const NAME: &'static str = "WheelchairAccessible";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2|3)
  }
}

impl ::protobuf::__internal::EntityType for WheelchairAccessible {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for WheelchairAccessible {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!1"))
    }).0
  }
}

}  // pub mod vehicle_descriptor


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__EntitySelector_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct EntitySelector {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<EntitySelector>
}

impl ::protobuf::Message for EntitySelector {
  type MessageView<'msg> = EntitySelectorView<'msg>;
  type MessageMut<'msg> = EntitySelectorMut<'msg>;
}

impl ::std::default::Default for EntitySelector {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for EntitySelector {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `EntitySelector` is `Sync` because it does not implement interior mutability.
//    Neither does `EntitySelectorMut`.
unsafe impl ::std::marker::Sync for EntitySelector {}

// SAFETY:
// - `EntitySelector` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for EntitySelector {}

impl ::protobuf::Proxied for EntitySelector {
  type View<'msg> = EntitySelectorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for EntitySelector {}

impl ::protobuf::MutProxied for EntitySelector {
  type Mut<'msg> = EntitySelectorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct EntitySelectorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EntitySelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntitySelectorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for EntitySelectorView<'msg> {
  type Message = EntitySelector;
}

impl ::std::fmt::Debug for EntitySelectorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for EntitySelectorView<'_> {
  fn default() -> EntitySelectorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, EntitySelector>> for EntitySelectorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, EntitySelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EntitySelectorView<'msg> {

  pub fn to_owned(&self) -> EntitySelector {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // agency_id: optional string
  pub fn has_agency_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn agency_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_agency_id().then(|| self.agency_id())
  }
  pub fn agency_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_id: optional string
  pub fn has_route_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn route_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // route_type: optional int32
  pub fn has_route_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn route_type_opt(self) -> ::std::option::Option<i32> {
    self.has_route_type().then(|| self.route_type())
  }
  pub fn route_type(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn trip_opt(self) -> ::std::option::Option<super::TripDescriptorView<'msg>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(self) -> super::TripDescriptorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }

  // stop_id: optional string
  pub fn has_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // direction_id: optional uint32
  pub fn has_direction_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn direction_id_opt(self) -> ::std::option::Option<u32> {
    self.has_direction_id().then(|| self.direction_id())
  }
  pub fn direction_id(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `EntitySelectorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for EntitySelectorView<'_> {}

// SAFETY:
// - `EntitySelectorView` is `Send` because while its alive a `EntitySelectorMut` cannot.
// - `EntitySelectorView` does not use thread-local data.
unsafe impl ::std::marker::Send for EntitySelectorView<'_> {}

impl<'msg> ::protobuf::AsView for EntitySelectorView<'msg> {
  type Proxied = EntitySelector;
  fn as_view(&self) -> ::protobuf::View<'msg, EntitySelector> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntitySelectorView<'msg> {
  fn into_view<'shorter>(self) -> EntitySelectorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<EntitySelector> for EntitySelectorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EntitySelector {
    let mut dst = EntitySelector::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<EntitySelector> for EntitySelectorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> EntitySelector {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for EntitySelector {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EntitySelectorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for EntitySelectorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct EntitySelectorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EntitySelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for EntitySelectorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for EntitySelectorMut<'msg> {
  type Message = EntitySelector;
}

impl ::std::fmt::Debug for EntitySelectorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, EntitySelector>> for EntitySelectorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, EntitySelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> EntitySelectorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, EntitySelector> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> EntitySelector {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // agency_id: optional string
  pub fn has_agency_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_agency_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn agency_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_agency_id().then(|| self.agency_id())
  }
  pub fn agency_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_agency_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // route_id: optional string
  pub fn has_route_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_route_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn route_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // route_type: optional int32
  pub fn has_route_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_route_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn route_type_opt(&self) -> ::std::option::Option<i32> {
    self.has_route_type().then(|| self.route_type())
  }
  pub fn route_type(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_route_type(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn trip_opt(&self) -> ::std::option::Option<super::TripDescriptorView<'_>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(&self) -> super::TripDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }
  pub fn trip_mut(&mut self) -> super::TripDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // direction_id: optional uint32
  pub fn has_direction_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_direction_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn direction_id_opt(&self) -> ::std::option::Option<u32> {
    self.has_direction_id().then(|| self.direction_id())
  }
  pub fn direction_id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_direction_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `EntitySelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for EntitySelectorMut<'_> {}

// SAFETY:
// - `EntitySelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for EntitySelectorMut<'_> {}

impl<'msg> ::protobuf::AsView for EntitySelectorMut<'msg> {
  type Proxied = EntitySelector;
  fn as_view(&self) -> ::protobuf::View<'_, EntitySelector> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for EntitySelectorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, EntitySelector>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for EntitySelectorMut<'msg> {
  type MutProxied = EntitySelector;
  fn as_mut(&mut self) -> EntitySelectorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for EntitySelectorMut<'msg> {
  fn into_mut<'shorter>(self) -> EntitySelectorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl EntitySelector {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, EntitySelector> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> EntitySelectorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> EntitySelectorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // agency_id: optional string
  pub fn has_agency_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_agency_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn agency_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_agency_id().then(|| self.agency_id())
  }
  pub fn agency_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_agency_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // route_id: optional string
  pub fn has_route_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_route_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn route_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // route_type: optional int32
  pub fn has_route_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_route_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn route_type_opt(&self) -> ::std::option::Option<i32> {
    self.has_route_type().then(|| self.route_type())
  }
  pub fn route_type(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_route_type(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // trip: optional message transit_realtime.TripDescriptor
  pub fn has_trip(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_trip(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn trip_opt(&self) -> ::std::option::Option<super::TripDescriptorView<'_>> {
    self.has_trip().then(|| self.trip())
  }
  pub fn trip(&self) -> super::TripDescriptorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TripDescriptorView::default())
  }
  pub fn trip_mut(&mut self) -> super::TripDescriptorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_trip(&mut self,
    val: impl ::protobuf::IntoProxied<super::TripDescriptor>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // direction_id: optional uint32
  pub fn has_direction_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_direction_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn direction_id_opt(&self) -> ::std::option::Option<u32> {
    self.has_direction_id().then(|| self.direction_id())
  }
  pub fn direction_id(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        5, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_direction_id(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        5, val.into()
      )
    }
  }

}  // impl EntitySelector

impl ::std::ops::Drop for EntitySelector {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for EntitySelector {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for EntitySelector {
  type Proxied = Self;
  fn as_view(&self) -> EntitySelectorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for EntitySelector {
  type MutProxied = Self;
  fn as_mut(&mut self) -> EntitySelectorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for EntitySelector {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__EntitySelector_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P11(31)");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__EntitySelector_msg_init.0, &[<super::TripDescriptor as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__EntitySelector_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EntitySelector {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EntitySelector {
  type Msg = EntitySelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntitySelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntitySelector {
  type Msg = EntitySelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntitySelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for EntitySelectorMut<'_> {
  type Msg = EntitySelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntitySelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntitySelectorMut<'_> {
  type Msg = EntitySelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntitySelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for EntitySelectorView<'_> {
  type Msg = EntitySelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<EntitySelector> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for EntitySelectorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TranslatedString_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TranslatedString {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TranslatedString>
}

impl ::protobuf::Message for TranslatedString {
  type MessageView<'msg> = TranslatedStringView<'msg>;
  type MessageMut<'msg> = TranslatedStringMut<'msg>;
}

impl ::std::default::Default for TranslatedString {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TranslatedString {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TranslatedString` is `Sync` because it does not implement interior mutability.
//    Neither does `TranslatedStringMut`.
unsafe impl ::std::marker::Sync for TranslatedString {}

// SAFETY:
// - `TranslatedString` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TranslatedString {}

impl ::protobuf::Proxied for TranslatedString {
  type View<'msg> = TranslatedStringView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TranslatedString {}

impl ::protobuf::MutProxied for TranslatedString {
  type Mut<'msg> = TranslatedStringMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TranslatedStringView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TranslatedString>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TranslatedStringView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TranslatedStringView<'msg> {
  type Message = TranslatedString;
}

impl ::std::fmt::Debug for TranslatedStringView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TranslatedStringView<'_> {
  fn default() -> TranslatedStringView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TranslatedString>> for TranslatedStringView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TranslatedString>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TranslatedStringView<'msg> {

  pub fn to_owned(&self) -> TranslatedString {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // translation: repeated message transit_realtime.TranslatedString.Translation
  pub fn translation(self) -> ::protobuf::RepeatedView<'msg, super::translated_string::Translation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::translated_string::Translation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TranslatedStringView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TranslatedStringView<'_> {}

// SAFETY:
// - `TranslatedStringView` is `Send` because while its alive a `TranslatedStringMut` cannot.
// - `TranslatedStringView` does not use thread-local data.
unsafe impl ::std::marker::Send for TranslatedStringView<'_> {}

impl<'msg> ::protobuf::AsView for TranslatedStringView<'msg> {
  type Proxied = TranslatedString;
  fn as_view(&self) -> ::protobuf::View<'msg, TranslatedString> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TranslatedStringView<'msg> {
  fn into_view<'shorter>(self) -> TranslatedStringView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TranslatedString> for TranslatedStringView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TranslatedString {
    let mut dst = TranslatedString::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TranslatedString> for TranslatedStringMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TranslatedString {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TranslatedString {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TranslatedStringView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TranslatedStringMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TranslatedStringMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedString>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TranslatedStringMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TranslatedStringMut<'msg> {
  type Message = TranslatedString;
}

impl ::std::fmt::Debug for TranslatedStringMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedString>> for TranslatedStringMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedString>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TranslatedStringMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedString> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TranslatedString {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // translation: repeated message transit_realtime.TranslatedString.Translation
  pub fn translation(&self) -> ::protobuf::RepeatedView<'_, super::translated_string::Translation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::translated_string::Translation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn translation_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::translated_string::Translation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_translation(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::translated_string::Translation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `TranslatedStringMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TranslatedStringMut<'_> {}

// SAFETY:
// - `TranslatedStringMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TranslatedStringMut<'_> {}

impl<'msg> ::protobuf::AsView for TranslatedStringMut<'msg> {
  type Proxied = TranslatedString;
  fn as_view(&self) -> ::protobuf::View<'_, TranslatedString> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TranslatedStringMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TranslatedString>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TranslatedStringMut<'msg> {
  type MutProxied = TranslatedString;
  fn as_mut(&mut self) -> TranslatedStringMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TranslatedStringMut<'msg> {
  fn into_mut<'shorter>(self) -> TranslatedStringMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TranslatedString {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TranslatedString> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TranslatedStringView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TranslatedStringMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // translation: repeated message transit_realtime.TranslatedString.Translation
  pub fn translation(&self) -> ::protobuf::RepeatedView<'_, super::translated_string::Translation> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::translated_string::Translation>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn translation_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::translated_string::Translation> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_translation(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::translated_string::Translation>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl TranslatedString

impl ::std::ops::Drop for TranslatedString {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TranslatedString {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TranslatedString {
  type Proxied = Self;
  fn as_view(&self) -> TranslatedStringView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TranslatedString {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TranslatedStringMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TranslatedString {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TranslatedString_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TranslatedString_msg_init.0, &[<super::translated_string::Translation as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TranslatedString_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TranslatedString {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TranslatedString {
  type Msg = TranslatedString;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedString> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslatedString {
  type Msg = TranslatedString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedString> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TranslatedStringMut<'_> {
  type Msg = TranslatedString;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedString> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslatedStringMut<'_> {
  type Msg = TranslatedString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedString> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslatedStringView<'_> {
  type Msg = TranslatedString;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedString> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TranslatedStringMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod translated_string {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TranslatedString__Translation_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Translation {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Translation>
}

impl ::protobuf::Message for Translation {
  type MessageView<'msg> = TranslationView<'msg>;
  type MessageMut<'msg> = TranslationMut<'msg>;
}

impl ::std::default::Default for Translation {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Translation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Translation` is `Sync` because it does not implement interior mutability.
//    Neither does `TranslationMut`.
unsafe impl ::std::marker::Sync for Translation {}

// SAFETY:
// - `Translation` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Translation {}

impl ::protobuf::Proxied for Translation {
  type View<'msg> = TranslationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Translation {}

impl ::protobuf::MutProxied for Translation {
  type Mut<'msg> = TranslationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TranslationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Translation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TranslationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TranslationView<'msg> {
  type Message = Translation;
}

impl ::std::fmt::Debug for TranslationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TranslationView<'_> {
  fn default() -> TranslationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Translation>> for TranslationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Translation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TranslationView<'msg> {

  pub fn to_owned(&self) -> Translation {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // text: optional string
  pub fn has_text(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn text_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_text().then(|| self.text())
  }
  pub fn text(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // language: optional string
  pub fn has_language(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn language_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_language().then(|| self.language())
  }
  pub fn language(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `TranslationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TranslationView<'_> {}

// SAFETY:
// - `TranslationView` is `Send` because while its alive a `TranslationMut` cannot.
// - `TranslationView` does not use thread-local data.
unsafe impl ::std::marker::Send for TranslationView<'_> {}

impl<'msg> ::protobuf::AsView for TranslationView<'msg> {
  type Proxied = Translation;
  fn as_view(&self) -> ::protobuf::View<'msg, Translation> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TranslationView<'msg> {
  fn into_view<'shorter>(self) -> TranslationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Translation> for TranslationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Translation {
    let mut dst = Translation::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Translation> for TranslationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Translation {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Translation {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TranslationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TranslationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TranslationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Translation>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TranslationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TranslationMut<'msg> {
  type Message = Translation;
}

impl ::std::fmt::Debug for TranslationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Translation>> for TranslationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Translation>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TranslationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Translation> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Translation {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // text: optional string
  pub fn has_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn text_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text().then(|| self.text())
  }
  pub fn text(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // language: optional string
  pub fn has_language(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_language(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn language_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_language().then(|| self.language())
  }
  pub fn language(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_language(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `TranslationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TranslationMut<'_> {}

// SAFETY:
// - `TranslationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TranslationMut<'_> {}

impl<'msg> ::protobuf::AsView for TranslationMut<'msg> {
  type Proxied = Translation;
  fn as_view(&self) -> ::protobuf::View<'_, Translation> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TranslationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Translation>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TranslationMut<'msg> {
  type MutProxied = Translation;
  fn as_mut(&mut self) -> TranslationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TranslationMut<'msg> {
  fn into_mut<'shorter>(self) -> TranslationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Translation {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Translation> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TranslationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TranslationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // text: optional string
  pub fn has_text(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_text(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn text_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_text().then(|| self.text())
  }
  pub fn text(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_text(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // language: optional string
  pub fn has_language(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_language(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn language_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_language().then(|| self.language())
  }
  pub fn language(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_language(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Translation

impl ::std::ops::Drop for Translation {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Translation {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Translation {
  type Proxied = Self;
  fn as_view(&self) -> TranslationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Translation {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TranslationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Translation {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::translated_string::transit_0realtime__TranslatedString__Translation_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1N1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::translated_string::transit_0realtime__TranslatedString__Translation_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::translated_string::transit_0realtime__TranslatedString__Translation_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Translation {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Translation {
  type Msg = Translation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Translation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Translation {
  type Msg = Translation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Translation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TranslationMut<'_> {
  type Msg = Translation;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Translation> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslationMut<'_> {
  type Msg = Translation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Translation> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslationView<'_> {
  type Msg = Translation;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Translation> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TranslationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod translated_string


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TranslatedImage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TranslatedImage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TranslatedImage>
}

impl ::protobuf::Message for TranslatedImage {
  type MessageView<'msg> = TranslatedImageView<'msg>;
  type MessageMut<'msg> = TranslatedImageMut<'msg>;
}

impl ::std::default::Default for TranslatedImage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TranslatedImage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TranslatedImage` is `Sync` because it does not implement interior mutability.
//    Neither does `TranslatedImageMut`.
unsafe impl ::std::marker::Sync for TranslatedImage {}

// SAFETY:
// - `TranslatedImage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TranslatedImage {}

impl ::protobuf::Proxied for TranslatedImage {
  type View<'msg> = TranslatedImageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TranslatedImage {}

impl ::protobuf::MutProxied for TranslatedImage {
  type Mut<'msg> = TranslatedImageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TranslatedImageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TranslatedImage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TranslatedImageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TranslatedImageView<'msg> {
  type Message = TranslatedImage;
}

impl ::std::fmt::Debug for TranslatedImageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TranslatedImageView<'_> {
  fn default() -> TranslatedImageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TranslatedImage>> for TranslatedImageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TranslatedImage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TranslatedImageView<'msg> {

  pub fn to_owned(&self) -> TranslatedImage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // localized_image: repeated message transit_realtime.TranslatedImage.LocalizedImage
  pub fn localized_image(self) -> ::protobuf::RepeatedView<'msg, super::translated_image::LocalizedImage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::translated_image::LocalizedImage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TranslatedImageView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TranslatedImageView<'_> {}

// SAFETY:
// - `TranslatedImageView` is `Send` because while its alive a `TranslatedImageMut` cannot.
// - `TranslatedImageView` does not use thread-local data.
unsafe impl ::std::marker::Send for TranslatedImageView<'_> {}

impl<'msg> ::protobuf::AsView for TranslatedImageView<'msg> {
  type Proxied = TranslatedImage;
  fn as_view(&self) -> ::protobuf::View<'msg, TranslatedImage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TranslatedImageView<'msg> {
  fn into_view<'shorter>(self) -> TranslatedImageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TranslatedImage> for TranslatedImageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TranslatedImage {
    let mut dst = TranslatedImage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TranslatedImage> for TranslatedImageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TranslatedImage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TranslatedImage {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TranslatedImageView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TranslatedImageMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TranslatedImageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedImage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TranslatedImageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TranslatedImageMut<'msg> {
  type Message = TranslatedImage;
}

impl ::std::fmt::Debug for TranslatedImageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedImage>> for TranslatedImageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedImage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TranslatedImageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TranslatedImage> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TranslatedImage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // localized_image: repeated message transit_realtime.TranslatedImage.LocalizedImage
  pub fn localized_image(&self) -> ::protobuf::RepeatedView<'_, super::translated_image::LocalizedImage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::translated_image::LocalizedImage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn localized_image_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::translated_image::LocalizedImage> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_localized_image(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::translated_image::LocalizedImage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}

// SAFETY:
// - `TranslatedImageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TranslatedImageMut<'_> {}

// SAFETY:
// - `TranslatedImageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TranslatedImageMut<'_> {}

impl<'msg> ::protobuf::AsView for TranslatedImageMut<'msg> {
  type Proxied = TranslatedImage;
  fn as_view(&self) -> ::protobuf::View<'_, TranslatedImage> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TranslatedImageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TranslatedImage>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TranslatedImageMut<'msg> {
  type MutProxied = TranslatedImage;
  fn as_mut(&mut self) -> TranslatedImageMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TranslatedImageMut<'msg> {
  fn into_mut<'shorter>(self) -> TranslatedImageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TranslatedImage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TranslatedImage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TranslatedImageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TranslatedImageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // localized_image: repeated message transit_realtime.TranslatedImage.LocalizedImage
  pub fn localized_image(&self) -> ::protobuf::RepeatedView<'_, super::translated_image::LocalizedImage> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::translated_image::LocalizedImage>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn localized_image_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::translated_image::LocalizedImage> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_localized_image(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::translated_image::LocalizedImage>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

}  // impl TranslatedImage

impl ::std::ops::Drop for TranslatedImage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TranslatedImage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TranslatedImage {
  type Proxied = Self;
  fn as_view(&self) -> TranslatedImageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TranslatedImage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TranslatedImageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TranslatedImage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TranslatedImage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$PG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TranslatedImage_msg_init.0, &[<super::translated_image::LocalizedImage as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TranslatedImage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TranslatedImage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TranslatedImage {
  type Msg = TranslatedImage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedImage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslatedImage {
  type Msg = TranslatedImage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedImage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TranslatedImageMut<'_> {
  type Msg = TranslatedImage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedImage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslatedImageMut<'_> {
  type Msg = TranslatedImage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedImage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TranslatedImageView<'_> {
  type Msg = TranslatedImage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TranslatedImage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TranslatedImageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod translated_image {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TranslatedImage__LocalizedImage_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct LocalizedImage {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<LocalizedImage>
}

impl ::protobuf::Message for LocalizedImage {
  type MessageView<'msg> = LocalizedImageView<'msg>;
  type MessageMut<'msg> = LocalizedImageMut<'msg>;
}

impl ::std::default::Default for LocalizedImage {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for LocalizedImage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `LocalizedImage` is `Sync` because it does not implement interior mutability.
//    Neither does `LocalizedImageMut`.
unsafe impl ::std::marker::Sync for LocalizedImage {}

// SAFETY:
// - `LocalizedImage` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for LocalizedImage {}

impl ::protobuf::Proxied for LocalizedImage {
  type View<'msg> = LocalizedImageView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for LocalizedImage {}

impl ::protobuf::MutProxied for LocalizedImage {
  type Mut<'msg> = LocalizedImageMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct LocalizedImageView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalizedImage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalizedImageView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for LocalizedImageView<'msg> {
  type Message = LocalizedImage;
}

impl ::std::fmt::Debug for LocalizedImageView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for LocalizedImageView<'_> {
  fn default() -> LocalizedImageView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, LocalizedImage>> for LocalizedImageView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, LocalizedImage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalizedImageView<'msg> {

  pub fn to_owned(&self) -> LocalizedImage {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // url: optional string
  pub fn has_url(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn url_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_url().then(|| self.url())
  }
  pub fn url(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // media_type: optional string
  pub fn has_media_type(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn media_type_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_media_type().then(|| self.media_type())
  }
  pub fn media_type(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // language: optional string
  pub fn has_language(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn language_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_language().then(|| self.language())
  }
  pub fn language(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `LocalizedImageView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for LocalizedImageView<'_> {}

// SAFETY:
// - `LocalizedImageView` is `Send` because while its alive a `LocalizedImageMut` cannot.
// - `LocalizedImageView` does not use thread-local data.
unsafe impl ::std::marker::Send for LocalizedImageView<'_> {}

impl<'msg> ::protobuf::AsView for LocalizedImageView<'msg> {
  type Proxied = LocalizedImage;
  fn as_view(&self) -> ::protobuf::View<'msg, LocalizedImage> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalizedImageView<'msg> {
  fn into_view<'shorter>(self) -> LocalizedImageView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalizedImage> for LocalizedImageView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalizedImage {
    let mut dst = LocalizedImage::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<LocalizedImage> for LocalizedImageMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> LocalizedImage {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for LocalizedImage {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalizedImageView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for LocalizedImageMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct LocalizedImageMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalizedImage>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for LocalizedImageMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for LocalizedImageMut<'msg> {
  type Message = LocalizedImage;
}

impl ::std::fmt::Debug for LocalizedImageMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, LocalizedImage>> for LocalizedImageMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalizedImage>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> LocalizedImageMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, LocalizedImage> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> LocalizedImage {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // url: optional string
  pub fn has_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn url_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_url().then(|| self.url())
  }
  pub fn url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // media_type: optional string
  pub fn has_media_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_media_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn media_type_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_media_type().then(|| self.media_type())
  }
  pub fn media_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_media_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // language: optional string
  pub fn has_language(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_language(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn language_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_language().then(|| self.language())
  }
  pub fn language(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_language(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}

// SAFETY:
// - `LocalizedImageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for LocalizedImageMut<'_> {}

// SAFETY:
// - `LocalizedImageMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for LocalizedImageMut<'_> {}

impl<'msg> ::protobuf::AsView for LocalizedImageMut<'msg> {
  type Proxied = LocalizedImage;
  fn as_view(&self) -> ::protobuf::View<'_, LocalizedImage> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for LocalizedImageMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, LocalizedImage>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for LocalizedImageMut<'msg> {
  type MutProxied = LocalizedImage;
  fn as_mut(&mut self) -> LocalizedImageMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for LocalizedImageMut<'msg> {
  fn into_mut<'shorter>(self) -> LocalizedImageMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl LocalizedImage {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, LocalizedImage> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> LocalizedImageView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> LocalizedImageMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // url: optional string
  pub fn has_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn url_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_url().then(|| self.url())
  }
  pub fn url(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_url(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // media_type: optional string
  pub fn has_media_type(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_media_type(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn media_type_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_media_type().then(|| self.media_type())
  }
  pub fn media_type(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_media_type(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

  // language: optional string
  pub fn has_language(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_language(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn language_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_language().then(|| self.language())
  }
  pub fn language(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        2, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_language(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val);
    }
  }

}  // impl LocalizedImage

impl ::std::ops::Drop for LocalizedImage {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for LocalizedImage {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for LocalizedImage {
  type Proxied = Self;
  fn as_view(&self) -> LocalizedImageView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for LocalizedImage {
  type MutProxied = Self;
  fn as_mut(&mut self) -> LocalizedImageMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for LocalizedImage {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::translated_image::transit_0realtime__TranslatedImage__LocalizedImage_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P1N1N1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::translated_image::transit_0realtime__TranslatedImage__LocalizedImage_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::translated_image::transit_0realtime__TranslatedImage__LocalizedImage_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalizedImage {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalizedImage {
  type Msg = LocalizedImage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalizedImage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalizedImage {
  type Msg = LocalizedImage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalizedImage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for LocalizedImageMut<'_> {
  type Msg = LocalizedImage;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalizedImage> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalizedImageMut<'_> {
  type Msg = LocalizedImage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalizedImage> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for LocalizedImageView<'_> {
  type Msg = LocalizedImage;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<LocalizedImage> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for LocalizedImageMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod translated_image


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__Shape_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Shape {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Shape>
}

impl ::protobuf::Message for Shape {
  type MessageView<'msg> = ShapeView<'msg>;
  type MessageMut<'msg> = ShapeMut<'msg>;
}

impl ::std::default::Default for Shape {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Shape {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Shape` is `Sync` because it does not implement interior mutability.
//    Neither does `ShapeMut`.
unsafe impl ::std::marker::Sync for Shape {}

// SAFETY:
// - `Shape` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Shape {}

impl ::protobuf::Proxied for Shape {
  type View<'msg> = ShapeView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Shape {}

impl ::protobuf::MutProxied for Shape {
  type Mut<'msg> = ShapeMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ShapeView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Shape>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ShapeView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ShapeView<'msg> {
  type Message = Shape;
}

impl ::std::fmt::Debug for ShapeView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ShapeView<'_> {
  fn default() -> ShapeView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Shape>> for ShapeView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Shape>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ShapeView<'msg> {

  pub fn to_owned(&self) -> Shape {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // shape_id: optional string
  pub fn has_shape_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn shape_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // encoded_polyline: optional string
  pub fn has_encoded_polyline(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn encoded_polyline_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_encoded_polyline().then(|| self.encoded_polyline())
  }
  pub fn encoded_polyline(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ShapeView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ShapeView<'_> {}

// SAFETY:
// - `ShapeView` is `Send` because while its alive a `ShapeMut` cannot.
// - `ShapeView` does not use thread-local data.
unsafe impl ::std::marker::Send for ShapeView<'_> {}

impl<'msg> ::protobuf::AsView for ShapeView<'msg> {
  type Proxied = Shape;
  fn as_view(&self) -> ::protobuf::View<'msg, Shape> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ShapeView<'msg> {
  fn into_view<'shorter>(self) -> ShapeView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Shape> for ShapeView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Shape {
    let mut dst = Shape::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Shape> for ShapeMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Shape {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Shape {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ShapeView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ShapeMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ShapeMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Shape>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ShapeMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ShapeMut<'msg> {
  type Message = Shape;
}

impl ::std::fmt::Debug for ShapeMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Shape>> for ShapeMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Shape>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ShapeMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Shape> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Shape {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // shape_id: optional string
  pub fn has_shape_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_shape_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn shape_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shape_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // encoded_polyline: optional string
  pub fn has_encoded_polyline(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_encoded_polyline(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn encoded_polyline_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_encoded_polyline().then(|| self.encoded_polyline())
  }
  pub fn encoded_polyline(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_encoded_polyline(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ShapeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ShapeMut<'_> {}

// SAFETY:
// - `ShapeMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ShapeMut<'_> {}

impl<'msg> ::protobuf::AsView for ShapeMut<'msg> {
  type Proxied = Shape;
  fn as_view(&self) -> ::protobuf::View<'_, Shape> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ShapeMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Shape>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ShapeMut<'msg> {
  type MutProxied = Shape;
  fn as_mut(&mut self) -> ShapeMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ShapeMut<'msg> {
  fn into_mut<'shorter>(self) -> ShapeMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Shape {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Shape> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ShapeView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ShapeMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // shape_id: optional string
  pub fn has_shape_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_shape_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn shape_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shape_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // encoded_polyline: optional string
  pub fn has_encoded_polyline(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_encoded_polyline(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn encoded_polyline_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_encoded_polyline().then(|| self.encoded_polyline())
  }
  pub fn encoded_polyline(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_encoded_polyline(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl Shape

impl ::std::ops::Drop for Shape {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Shape {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Shape {
  type Proxied = Self;
  fn as_view(&self) -> ShapeView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Shape {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ShapeMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Shape {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__Shape_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P11");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__Shape_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__Shape_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Shape {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Shape {
  type Msg = Shape;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Shape> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Shape {
  type Msg = Shape;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Shape> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ShapeMut<'_> {
  type Msg = Shape;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Shape> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ShapeMut<'_> {
  type Msg = Shape;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Shape> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ShapeView<'_> {
  type Msg = Shape;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Shape> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ShapeMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__Stop_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Stop {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Stop>
}

impl ::protobuf::Message for Stop {
  type MessageView<'msg> = StopView<'msg>;
  type MessageMut<'msg> = StopMut<'msg>;
}

impl ::std::default::Default for Stop {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Stop {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Stop` is `Sync` because it does not implement interior mutability.
//    Neither does `StopMut`.
unsafe impl ::std::marker::Sync for Stop {}

// SAFETY:
// - `Stop` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Stop {}

impl ::protobuf::Proxied for Stop {
  type View<'msg> = StopView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Stop {}

impl ::protobuf::MutProxied for Stop {
  type Mut<'msg> = StopMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StopView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Stop>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StopView<'msg> {
  type Message = Stop;
}

impl ::std::fmt::Debug for StopView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StopView<'_> {
  fn default() -> StopView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Stop>> for StopView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Stop>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopView<'msg> {

  pub fn to_owned(&self) -> Stop {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // stop_id: optional string
  pub fn has_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stop_code: optional message transit_realtime.TranslatedString
  pub fn has_stop_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn stop_code_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_stop_code().then(|| self.stop_code())
  }
  pub fn stop_code(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // stop_name: optional message transit_realtime.TranslatedString
  pub fn has_stop_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn stop_name_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_stop_name().then(|| self.stop_name())
  }
  pub fn stop_name(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // tts_stop_name: optional message transit_realtime.TranslatedString
  pub fn has_tts_stop_name(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn tts_stop_name_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_tts_stop_name().then(|| self.tts_stop_name())
  }
  pub fn tts_stop_name(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // stop_desc: optional message transit_realtime.TranslatedString
  pub fn has_stop_desc(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn stop_desc_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_stop_desc().then(|| self.stop_desc())
  }
  pub fn stop_desc(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // stop_lat: optional float
  pub fn has_stop_lat(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn stop_lat_opt(self) -> ::std::option::Option<f32> {
    self.has_stop_lat().then(|| self.stop_lat())
  }
  pub fn stop_lat(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        5, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // stop_lon: optional float
  pub fn has_stop_lon(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn stop_lon_opt(self) -> ::std::option::Option<f32> {
    self.has_stop_lon().then(|| self.stop_lon())
  }
  pub fn stop_lon(self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        6, (0f32).into()
      ).try_into().unwrap()
    }
  }

  // zone_id: optional string
  pub fn has_zone_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn zone_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_zone_id().then(|| self.zone_id())
  }
  pub fn zone_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stop_url: optional message transit_realtime.TranslatedString
  pub fn has_stop_url(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn stop_url_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_stop_url().then(|| self.stop_url())
  }
  pub fn stop_url(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

  // parent_station: optional string
  pub fn has_parent_station(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn parent_station_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_parent_station().then(|| self.parent_station())
  }
  pub fn parent_station(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // stop_timezone: optional string
  pub fn has_stop_timezone(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn stop_timezone_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_timezone().then(|| self.stop_timezone())
  }
  pub fn stop_timezone(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // wheelchair_boarding: optional enum transit_realtime.Stop.WheelchairBoarding
  pub fn has_wheelchair_boarding(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn wheelchair_boarding_opt(self) -> ::std::option::Option<super::stop::WheelchairBoarding> {
    self.has_wheelchair_boarding().then(|| self.wheelchair_boarding())
  }
  pub fn wheelchair_boarding(self) -> super::stop::WheelchairBoarding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        11, (super::stop::WheelchairBoarding::Unknown).into()
      ).try_into().unwrap()
    }
  }

  // level_id: optional string
  pub fn has_level_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn level_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_level_id().then(|| self.level_id())
  }
  pub fn level_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // platform_code: optional message transit_realtime.TranslatedString
  pub fn has_platform_code(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn platform_code_opt(self) -> ::std::option::Option<super::TranslatedStringView<'msg>> {
    self.has_platform_code().then(|| self.platform_code())
  }
  pub fn platform_code(self) -> super::TranslatedStringView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }

}

// SAFETY:
// - `StopView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StopView<'_> {}

// SAFETY:
// - `StopView` is `Send` because while its alive a `StopMut` cannot.
// - `StopView` does not use thread-local data.
unsafe impl ::std::marker::Send for StopView<'_> {}

impl<'msg> ::protobuf::AsView for StopView<'msg> {
  type Proxied = Stop;
  fn as_view(&self) -> ::protobuf::View<'msg, Stop> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopView<'msg> {
  fn into_view<'shorter>(self) -> StopView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Stop> for StopView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Stop {
    let mut dst = Stop::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Stop> for StopMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Stop {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Stop {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StopMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Stop>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StopMut<'msg> {
  type Message = Stop;
}

impl ::std::fmt::Debug for StopMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Stop>> for StopMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Stop>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Stop> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Stop {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stop_code: optional message transit_realtime.TranslatedString
  pub fn has_stop_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_code_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_code().then(|| self.stop_code())
  }
  pub fn stop_code(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_code_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_code(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // stop_name: optional message transit_realtime.TranslatedString
  pub fn has_stop_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_stop_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn stop_name_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_name().then(|| self.stop_name())
  }
  pub fn stop_name(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_name_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // tts_stop_name: optional message transit_realtime.TranslatedString
  pub fn has_tts_stop_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_tts_stop_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn tts_stop_name_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_tts_stop_name().then(|| self.tts_stop_name())
  }
  pub fn tts_stop_name(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn tts_stop_name_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tts_stop_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // stop_desc: optional message transit_realtime.TranslatedString
  pub fn has_stop_desc(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_stop_desc(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn stop_desc_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_desc().then(|| self.stop_desc())
  }
  pub fn stop_desc(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_desc_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_desc(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // stop_lat: optional float
  pub fn has_stop_lat(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_stop_lat(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn stop_lat_opt(&self) -> ::std::option::Option<f32> {
    self.has_stop_lat().then(|| self.stop_lat())
  }
  pub fn stop_lat(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        5, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_lat(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        5, val.into()
      )
    }
  }

  // stop_lon: optional float
  pub fn has_stop_lon(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stop_lon(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stop_lon_opt(&self) -> ::std::option::Option<f32> {
    self.has_stop_lon().then(|| self.stop_lon())
  }
  pub fn stop_lon(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        6, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_lon(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        6, val.into()
      )
    }
  }

  // zone_id: optional string
  pub fn has_zone_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_zone_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn zone_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_zone_id().then(|| self.zone_id())
  }
  pub fn zone_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_zone_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // stop_url: optional message transit_realtime.TranslatedString
  pub fn has_stop_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_stop_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn stop_url_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_url().then(|| self.stop_url())
  }
  pub fn stop_url(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_url_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_url(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // parent_station: optional string
  pub fn has_parent_station(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_parent_station(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn parent_station_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_parent_station().then(|| self.parent_station())
  }
  pub fn parent_station(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_parent_station(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // stop_timezone: optional string
  pub fn has_stop_timezone(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_stop_timezone(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn stop_timezone_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_timezone().then(|| self.stop_timezone())
  }
  pub fn stop_timezone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_timezone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // wheelchair_boarding: optional enum transit_realtime.Stop.WheelchairBoarding
  pub fn has_wheelchair_boarding(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_wheelchair_boarding(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn wheelchair_boarding_opt(&self) -> ::std::option::Option<super::stop::WheelchairBoarding> {
    self.has_wheelchair_boarding().then(|| self.wheelchair_boarding())
  }
  pub fn wheelchair_boarding(&self) -> super::stop::WheelchairBoarding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        11, (super::stop::WheelchairBoarding::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_wheelchair_boarding(&mut self, val: super::stop::WheelchairBoarding) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        11, val.into()
      )
    }
  }

  // level_id: optional string
  pub fn has_level_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_level_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn level_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_level_id().then(|| self.level_id())
  }
  pub fn level_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_level_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // platform_code: optional message transit_realtime.TranslatedString
  pub fn has_platform_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_platform_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn platform_code_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_platform_code().then(|| self.platform_code())
  }
  pub fn platform_code(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn platform_code_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_platform_code(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

}

// SAFETY:
// - `StopMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StopMut<'_> {}

// SAFETY:
// - `StopMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StopMut<'_> {}

impl<'msg> ::protobuf::AsView for StopMut<'msg> {
  type Proxied = Stop;
  fn as_view(&self) -> ::protobuf::View<'_, Stop> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Stop>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StopMut<'msg> {
  type MutProxied = Stop;
  fn as_mut(&mut self) -> StopMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StopMut<'msg> {
  fn into_mut<'shorter>(self) -> StopMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Stop {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Stop> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StopView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StopMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // stop_code: optional message transit_realtime.TranslatedString
  pub fn has_stop_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_code_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_code().then(|| self.stop_code())
  }
  pub fn stop_code(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_code_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_code(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // stop_name: optional message transit_realtime.TranslatedString
  pub fn has_stop_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_stop_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn stop_name_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_name().then(|| self.stop_name())
  }
  pub fn stop_name(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(2)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_name_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         2, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        val
      );
    }
  }

  // tts_stop_name: optional message transit_realtime.TranslatedString
  pub fn has_tts_stop_name(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(3)
    }
  }
  pub fn clear_tts_stop_name(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        3
      );
    }
  }
  pub fn tts_stop_name_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_tts_stop_name().then(|| self.tts_stop_name())
  }
  pub fn tts_stop_name(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(3)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn tts_stop_name_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         3, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_tts_stop_name(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        val
      );
    }
  }

  // stop_desc: optional message transit_realtime.TranslatedString
  pub fn has_stop_desc(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_stop_desc(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn stop_desc_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_desc().then(|| self.stop_desc())
  }
  pub fn stop_desc(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(4)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_desc_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         4, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_desc(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val
      );
    }
  }

  // stop_lat: optional float
  pub fn has_stop_lat(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_stop_lat(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn stop_lat_opt(&self) -> ::std::option::Option<f32> {
    self.has_stop_lat().then(|| self.stop_lat())
  }
  pub fn stop_lat(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        5, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_lat(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        5, val.into()
      )
    }
  }

  // stop_lon: optional float
  pub fn has_stop_lon(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(6)
    }
  }
  pub fn clear_stop_lon(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        6
      );
    }
  }
  pub fn stop_lon_opt(&self) -> ::std::option::Option<f32> {
    self.has_stop_lon().then(|| self.stop_lon())
  }
  pub fn stop_lon(&self) -> f32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_f32_at_index(
        6, (0f32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_lon(&mut self, val: f32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_f32_at_index(
        6, val.into()
      )
    }
  }

  // zone_id: optional string
  pub fn has_zone_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(7)
    }
  }
  pub fn clear_zone_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        7
      );
    }
  }
  pub fn zone_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_zone_id().then(|| self.zone_id())
  }
  pub fn zone_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        7, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_zone_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        7,
        val);
    }
  }

  // stop_url: optional message transit_realtime.TranslatedString
  pub fn has_stop_url(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(8)
    }
  }
  pub fn clear_stop_url(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        8
      );
    }
  }
  pub fn stop_url_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_stop_url().then(|| self.stop_url())
  }
  pub fn stop_url(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(8)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn stop_url_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         8, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_stop_url(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        8,
        val
      );
    }
  }

  // parent_station: optional string
  pub fn has_parent_station(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(9)
    }
  }
  pub fn clear_parent_station(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        9
      );
    }
  }
  pub fn parent_station_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_parent_station().then(|| self.parent_station())
  }
  pub fn parent_station(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        9, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_parent_station(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        9,
        val);
    }
  }

  // stop_timezone: optional string
  pub fn has_stop_timezone(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(10)
    }
  }
  pub fn clear_stop_timezone(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        10
      );
    }
  }
  pub fn stop_timezone_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_timezone().then(|| self.stop_timezone())
  }
  pub fn stop_timezone(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        10, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_timezone(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        10,
        val);
    }
  }

  // wheelchair_boarding: optional enum transit_realtime.Stop.WheelchairBoarding
  pub fn has_wheelchair_boarding(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(11)
    }
  }
  pub fn clear_wheelchair_boarding(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        11
      );
    }
  }
  pub fn wheelchair_boarding_opt(&self) -> ::std::option::Option<super::stop::WheelchairBoarding> {
    self.has_wheelchair_boarding().then(|| self.wheelchair_boarding())
  }
  pub fn wheelchair_boarding(&self) -> super::stop::WheelchairBoarding {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        11, (super::stop::WheelchairBoarding::Unknown).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_wheelchair_boarding(&mut self, val: super::stop::WheelchairBoarding) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        11, val.into()
      )
    }
  }

  // level_id: optional string
  pub fn has_level_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(12)
    }
  }
  pub fn clear_level_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        12
      );
    }
  }
  pub fn level_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_level_id().then(|| self.level_id())
  }
  pub fn level_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        12, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_level_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        12,
        val);
    }
  }

  // platform_code: optional message transit_realtime.TranslatedString
  pub fn has_platform_code(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(13)
    }
  }
  pub fn clear_platform_code(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        13
      );
    }
  }
  pub fn platform_code_opt(&self) -> ::std::option::Option<super::TranslatedStringView<'_>> {
    self.has_platform_code().then(|| self.platform_code())
  }
  pub fn platform_code(&self) -> super::TranslatedStringView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(13)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TranslatedStringView::default())
  }
  pub fn platform_code_mut(&mut self) -> super::TranslatedStringMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         13, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_platform_code(&mut self,
    val: impl ::protobuf::IntoProxied<super::TranslatedString>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        13,
        val
      );
    }
  }

}  // impl Stop

impl ::std::ops::Drop for Stop {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Stop {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Stop {
  type Proxied = Self;
  fn as_view(&self) -> StopView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Stop {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StopMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Stop {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__Stop_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P13333!!13a11413");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__Stop_msg_init.0, &[<super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::TranslatedString as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[<super::stop::WheelchairBoarding as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__Stop_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Stop {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Stop {
  type Msg = Stop;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stop> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Stop {
  type Msg = Stop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stop> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopMut<'_> {
  type Msg = Stop;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stop> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopMut<'_> {
  type Msg = Stop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stop> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopView<'_> {
  type Msg = Stop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Stop> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod stop {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WheelchairBoarding(i32);

#[allow(non_upper_case_globals)]
impl WheelchairBoarding {
  pub const Unknown: WheelchairBoarding = WheelchairBoarding(0);
  pub const Available: WheelchairBoarding = WheelchairBoarding(1);
  pub const NotAvailable: WheelchairBoarding = WheelchairBoarding(2);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      0 => "Unknown",
      1 => "Available",
      2 => "NotAvailable",
      _ => return None
    })
  }
}

impl ::std::convert::From<WheelchairBoarding> for i32 {
  fn from(val: WheelchairBoarding) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for WheelchairBoarding {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<WheelchairBoarding, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for WheelchairBoarding {
  fn default() -> Self {
    Self(0)
  }
}

impl ::std::fmt::Debug for WheelchairBoarding {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "WheelchairBoarding::{}", constant_name)
    } else {
      write!(f, "WheelchairBoarding::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for WheelchairBoarding {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for WheelchairBoarding {}

impl ::protobuf::Proxied for WheelchairBoarding {
  type View<'a> = WheelchairBoarding;
}

impl ::protobuf::AsView for WheelchairBoarding {
  type Proxied = WheelchairBoarding;

  fn as_view(&self) -> WheelchairBoarding {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for WheelchairBoarding {
  fn into_view<'shorter>(self) -> WheelchairBoarding where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for WheelchairBoarding {
  const NAME: &'static str = "WheelchairBoarding";

  fn is_known(value: i32) -> bool {
    matches!(value, 0|1|2)
  }
}

impl ::protobuf::__internal::EntityType for WheelchairBoarding {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for WheelchairBoarding {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!)"))
    }).0
  }
}

}  // pub mod stop


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripModifications_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TripModifications {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TripModifications>
}

impl ::protobuf::Message for TripModifications {
  type MessageView<'msg> = TripModificationsView<'msg>;
  type MessageMut<'msg> = TripModificationsMut<'msg>;
}

impl ::std::default::Default for TripModifications {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TripModifications {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TripModifications` is `Sync` because it does not implement interior mutability.
//    Neither does `TripModificationsMut`.
unsafe impl ::std::marker::Sync for TripModifications {}

// SAFETY:
// - `TripModifications` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TripModifications {}

impl ::protobuf::Proxied for TripModifications {
  type View<'msg> = TripModificationsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TripModifications {}

impl ::protobuf::MutProxied for TripModifications {
  type Mut<'msg> = TripModificationsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TripModificationsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripModifications>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripModificationsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TripModificationsView<'msg> {
  type Message = TripModifications;
}

impl ::std::fmt::Debug for TripModificationsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TripModificationsView<'_> {
  fn default() -> TripModificationsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TripModifications>> for TripModificationsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripModifications>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripModificationsView<'msg> {

  pub fn to_owned(&self) -> TripModifications {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // selected_trips: repeated message transit_realtime.TripModifications.SelectedTrips
  pub fn selected_trips(self) -> ::protobuf::RepeatedView<'msg, super::trip_modifications::SelectedTrips> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_modifications::SelectedTrips>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // start_times: repeated string
  pub fn start_times(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // service_dates: repeated string
  pub fn service_dates(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // modifications: repeated message transit_realtime.TripModifications.Modification
  pub fn modifications(self) -> ::protobuf::RepeatedView<'msg, super::trip_modifications::Modification> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_modifications::Modification>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `TripModificationsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TripModificationsView<'_> {}

// SAFETY:
// - `TripModificationsView` is `Send` because while its alive a `TripModificationsMut` cannot.
// - `TripModificationsView` does not use thread-local data.
unsafe impl ::std::marker::Send for TripModificationsView<'_> {}

impl<'msg> ::protobuf::AsView for TripModificationsView<'msg> {
  type Proxied = TripModifications;
  fn as_view(&self) -> ::protobuf::View<'msg, TripModifications> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripModificationsView<'msg> {
  fn into_view<'shorter>(self) -> TripModificationsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TripModifications> for TripModificationsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripModifications {
    let mut dst = TripModifications::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TripModifications> for TripModificationsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripModifications {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TripModifications {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripModificationsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripModificationsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TripModificationsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripModifications>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripModificationsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TripModificationsMut<'msg> {
  type Message = TripModifications;
}

impl ::std::fmt::Debug for TripModificationsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TripModifications>> for TripModificationsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripModifications>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripModificationsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TripModifications> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TripModifications {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // selected_trips: repeated message transit_realtime.TripModifications.SelectedTrips
  pub fn selected_trips(&self) -> ::protobuf::RepeatedView<'_, super::trip_modifications::SelectedTrips> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_modifications::SelectedTrips>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn selected_trips_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::trip_modifications::SelectedTrips> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_selected_trips(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::trip_modifications::SelectedTrips>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // start_times: repeated string
  pub fn start_times(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn start_times_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_start_times(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // service_dates: repeated string
  pub fn service_dates(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn service_dates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_service_dates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // modifications: repeated message transit_realtime.TripModifications.Modification
  pub fn modifications(&self) -> ::protobuf::RepeatedView<'_, super::trip_modifications::Modification> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_modifications::Modification>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn modifications_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::trip_modifications::Modification> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_modifications(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::trip_modifications::Modification>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}

// SAFETY:
// - `TripModificationsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TripModificationsMut<'_> {}

// SAFETY:
// - `TripModificationsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TripModificationsMut<'_> {}

impl<'msg> ::protobuf::AsView for TripModificationsMut<'msg> {
  type Proxied = TripModifications;
  fn as_view(&self) -> ::protobuf::View<'_, TripModifications> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripModificationsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TripModifications>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TripModificationsMut<'msg> {
  type MutProxied = TripModifications;
  fn as_mut(&mut self) -> TripModificationsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TripModificationsMut<'msg> {
  fn into_mut<'shorter>(self) -> TripModificationsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TripModifications {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TripModifications> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TripModificationsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TripModificationsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // selected_trips: repeated message transit_realtime.TripModifications.SelectedTrips
  pub fn selected_trips(&self) -> ::protobuf::RepeatedView<'_, super::trip_modifications::SelectedTrips> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_modifications::SelectedTrips>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn selected_trips_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::trip_modifications::SelectedTrips> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_selected_trips(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::trip_modifications::SelectedTrips>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // start_times: repeated string
  pub fn start_times(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn start_times_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        1,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_start_times(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

  // service_dates: repeated string
  pub fn service_dates(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        2
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn service_dates_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        2,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_service_dates(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        2,
        src);
    }
  }

  // modifications: repeated message transit_realtime.TripModifications.Modification
  pub fn modifications(&self) -> ::protobuf::RepeatedView<'_, super::trip_modifications::Modification> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::trip_modifications::Modification>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn modifications_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::trip_modifications::Modification> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_modifications(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::trip_modifications::Modification>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

}  // impl TripModifications

impl ::std::ops::Drop for TripModifications {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TripModifications {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TripModifications {
  type Proxied = Self;
  fn as_view(&self) -> TripModificationsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TripModifications {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TripModificationsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TripModifications {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TripModifications_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$PGEEG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TripModifications_msg_init.0, &[<super::trip_modifications::SelectedTrips as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::trip_modifications::Modification as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TripModifications_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripModifications {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripModifications {
  type Msg = TripModifications;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripModifications> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripModifications {
  type Msg = TripModifications;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripModifications> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripModificationsMut<'_> {
  type Msg = TripModifications;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripModifications> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripModificationsMut<'_> {
  type Msg = TripModifications;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripModifications> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripModificationsView<'_> {
  type Msg = TripModifications;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripModifications> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripModificationsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod trip_modifications {// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripModifications__Modification_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct Modification {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<Modification>
}

impl ::protobuf::Message for Modification {
  type MessageView<'msg> = ModificationView<'msg>;
  type MessageMut<'msg> = ModificationMut<'msg>;
}

impl ::std::default::Default for Modification {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for Modification {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `Modification` is `Sync` because it does not implement interior mutability.
//    Neither does `ModificationMut`.
unsafe impl ::std::marker::Sync for Modification {}

// SAFETY:
// - `Modification` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for Modification {}

impl ::protobuf::Proxied for Modification {
  type View<'msg> = ModificationView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for Modification {}

impl ::protobuf::MutProxied for Modification {
  type Mut<'msg> = ModificationMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ModificationView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Modification>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModificationView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ModificationView<'msg> {
  type Message = Modification;
}

impl ::std::fmt::Debug for ModificationView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ModificationView<'_> {
  fn default() -> ModificationView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, Modification>> for ModificationView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, Modification>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModificationView<'msg> {

  pub fn to_owned(&self) -> Modification {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // start_stop_selector: optional message transit_realtime.StopSelector
  pub fn has_start_stop_selector(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn start_stop_selector_opt(self) -> ::std::option::Option<super::super::StopSelectorView<'msg>> {
    self.has_start_stop_selector().then(|| self.start_stop_selector())
  }
  pub fn start_stop_selector(self) -> super::super::StopSelectorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::StopSelectorView::default())
  }

  // end_stop_selector: optional message transit_realtime.StopSelector
  pub fn has_end_stop_selector(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn end_stop_selector_opt(self) -> ::std::option::Option<super::super::StopSelectorView<'msg>> {
    self.has_end_stop_selector().then(|| self.end_stop_selector())
  }
  pub fn end_stop_selector(self) -> super::super::StopSelectorView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::StopSelectorView::default())
  }

  // propagated_modification_delay: optional int32
  pub fn has_propagated_modification_delay(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn propagated_modification_delay_opt(self) -> ::std::option::Option<i32> {
    self.has_propagated_modification_delay().then(|| self.propagated_modification_delay())
  }
  pub fn propagated_modification_delay(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // replacement_stops: repeated message transit_realtime.ReplacementStop
  pub fn replacement_stops(self) -> ::protobuf::RepeatedView<'msg, super::super::ReplacementStop> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::ReplacementStop>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // service_alert_id: optional string
  pub fn has_service_alert_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn service_alert_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_service_alert_id().then(|| self.service_alert_id())
  }
  pub fn service_alert_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // last_modified_time: optional uint64
  pub fn has_last_modified_time(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn last_modified_time_opt(self) -> ::std::option::Option<u64> {
    self.has_last_modified_time().then(|| self.last_modified_time())
  }
  pub fn last_modified_time(self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        5, (0u64).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `ModificationView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ModificationView<'_> {}

// SAFETY:
// - `ModificationView` is `Send` because while its alive a `ModificationMut` cannot.
// - `ModificationView` does not use thread-local data.
unsafe impl ::std::marker::Send for ModificationView<'_> {}

impl<'msg> ::protobuf::AsView for ModificationView<'msg> {
  type Proxied = Modification;
  fn as_view(&self) -> ::protobuf::View<'msg, Modification> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModificationView<'msg> {
  fn into_view<'shorter>(self) -> ModificationView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<Modification> for ModificationView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Modification {
    let mut dst = Modification::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<Modification> for ModificationMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> Modification {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for Modification {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ModificationView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ModificationMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ModificationMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Modification>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ModificationMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ModificationMut<'msg> {
  type Message = Modification;
}

impl ::std::fmt::Debug for ModificationMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, Modification>> for ModificationMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, Modification>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ModificationMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, Modification> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> Modification {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // start_stop_selector: optional message transit_realtime.StopSelector
  pub fn has_start_stop_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_start_stop_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn start_stop_selector_opt(&self) -> ::std::option::Option<super::super::StopSelectorView<'_>> {
    self.has_start_stop_selector().then(|| self.start_stop_selector())
  }
  pub fn start_stop_selector(&self) -> super::super::StopSelectorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::StopSelectorView::default())
  }
  pub fn start_stop_selector_mut(&mut self) -> super::super::StopSelectorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_start_stop_selector(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::StopSelector>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // end_stop_selector: optional message transit_realtime.StopSelector
  pub fn has_end_stop_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_end_stop_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn end_stop_selector_opt(&self) -> ::std::option::Option<super::super::StopSelectorView<'_>> {
    self.has_end_stop_selector().then(|| self.end_stop_selector())
  }
  pub fn end_stop_selector(&self) -> super::super::StopSelectorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::StopSelectorView::default())
  }
  pub fn end_stop_selector_mut(&mut self) -> super::super::StopSelectorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_end_stop_selector(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::StopSelector>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // propagated_modification_delay: optional int32
  pub fn has_propagated_modification_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_propagated_modification_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn propagated_modification_delay_opt(&self) -> ::std::option::Option<i32> {
    self.has_propagated_modification_delay().then(|| self.propagated_modification_delay())
  }
  pub fn propagated_modification_delay(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_propagated_modification_delay(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // replacement_stops: repeated message transit_realtime.ReplacementStop
  pub fn replacement_stops(&self) -> ::protobuf::RepeatedView<'_, super::super::ReplacementStop> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::ReplacementStop>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn replacement_stops_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::ReplacementStop> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_replacement_stops(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::ReplacementStop>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // service_alert_id: optional string
  pub fn has_service_alert_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_service_alert_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn service_alert_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_service_alert_id().then(|| self.service_alert_id())
  }
  pub fn service_alert_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_alert_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // last_modified_time: optional uint64
  pub fn has_last_modified_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_last_modified_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn last_modified_time_opt(&self) -> ::std::option::Option<u64> {
    self.has_last_modified_time().then(|| self.last_modified_time())
  }
  pub fn last_modified_time(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        5, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_last_modified_time(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        5, val.into()
      )
    }
  }

}

// SAFETY:
// - `ModificationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ModificationMut<'_> {}

// SAFETY:
// - `ModificationMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ModificationMut<'_> {}

impl<'msg> ::protobuf::AsView for ModificationMut<'msg> {
  type Proxied = Modification;
  fn as_view(&self) -> ::protobuf::View<'_, Modification> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ModificationMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, Modification>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ModificationMut<'msg> {
  type MutProxied = Modification;
  fn as_mut(&mut self) -> ModificationMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ModificationMut<'msg> {
  fn into_mut<'shorter>(self) -> ModificationMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl Modification {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, Modification> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ModificationView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ModificationMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // start_stop_selector: optional message transit_realtime.StopSelector
  pub fn has_start_stop_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_start_stop_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn start_stop_selector_opt(&self) -> ::std::option::Option<super::super::StopSelectorView<'_>> {
    self.has_start_stop_selector().then(|| self.start_stop_selector())
  }
  pub fn start_stop_selector(&self) -> super::super::StopSelectorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(0)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::StopSelectorView::default())
  }
  pub fn start_stop_selector_mut(&mut self) -> super::super::StopSelectorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         0, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_start_stop_selector(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::StopSelector>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val
      );
    }
  }

  // end_stop_selector: optional message transit_realtime.StopSelector
  pub fn has_end_stop_selector(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_end_stop_selector(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn end_stop_selector_opt(&self) -> ::std::option::Option<super::super::StopSelectorView<'_>> {
    self.has_end_stop_selector().then(|| self.end_stop_selector())
  }
  pub fn end_stop_selector(&self) -> super::super::StopSelectorView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::super::StopSelectorView::default())
  }
  pub fn end_stop_selector_mut(&mut self) -> super::super::StopSelectorMut<'_> {
     let ptr = unsafe {
       self.inner.ptr_mut().get_or_create_mutable_message_at_index(
         1, self.inner.arena()
       ).unwrap()
     };
     ::protobuf::__internal::runtime::MessageMutInner::from_parent(
         self.as_message_mut_inner(::protobuf::__internal::Private),
         ptr
     ).into()
  }
  pub fn set_end_stop_selector(&mut self,
    val: impl ::protobuf::IntoProxied<super::super::StopSelector>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

  // propagated_modification_delay: optional int32
  pub fn has_propagated_modification_delay(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_propagated_modification_delay(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn propagated_modification_delay_opt(&self) -> ::std::option::Option<i32> {
    self.has_propagated_modification_delay().then(|| self.propagated_modification_delay())
  }
  pub fn propagated_modification_delay(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_propagated_modification_delay(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        2, val.into()
      )
    }
  }

  // replacement_stops: repeated message transit_realtime.ReplacementStop
  pub fn replacement_stops(&self) -> ::protobuf::RepeatedView<'_, super::super::ReplacementStop> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        3
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::super::ReplacementStop>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn replacement_stops_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::super::ReplacementStop> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        3,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_replacement_stops(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::super::ReplacementStop>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        3,
        src);
    }
  }

  // service_alert_id: optional string
  pub fn has_service_alert_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(4)
    }
  }
  pub fn clear_service_alert_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        4
      );
    }
  }
  pub fn service_alert_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_service_alert_id().then(|| self.service_alert_id())
  }
  pub fn service_alert_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        4, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_service_alert_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        4,
        val);
    }
  }

  // last_modified_time: optional uint64
  pub fn has_last_modified_time(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(5)
    }
  }
  pub fn clear_last_modified_time(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        5
      );
    }
  }
  pub fn last_modified_time_opt(&self) -> ::std::option::Option<u64> {
    self.has_last_modified_time().then(|| self.last_modified_time())
  }
  pub fn last_modified_time(&self) -> u64 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u64_at_index(
        5, (0u64).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_last_modified_time(&mut self, val: u64) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u64_at_index(
        5, val.into()
      )
    }
  }

}  // impl Modification

impl ::std::ops::Drop for Modification {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for Modification {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for Modification {
  type Proxied = Self;
  fn as_view(&self) -> ModificationView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for Modification {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ModificationMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for Modification {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::trip_modifications::transit_0realtime__TripModifications__Modification_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P33(G1,");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::trip_modifications::transit_0realtime__TripModifications__Modification_msg_init.0, &[<super::super::StopSelector as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::StopSelector as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            <super::super::ReplacementStop as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::trip_modifications::transit_0realtime__TripModifications__Modification_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for Modification {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for Modification {
  type Msg = Modification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Modification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for Modification {
  type Msg = Modification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Modification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ModificationMut<'_> {
  type Msg = Modification;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Modification> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModificationMut<'_> {
  type Msg = Modification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Modification> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ModificationView<'_> {
  type Msg = Modification;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<Modification> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ModificationMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripModifications__SelectedTrips_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct SelectedTrips {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<SelectedTrips>
}

impl ::protobuf::Message for SelectedTrips {
  type MessageView<'msg> = SelectedTripsView<'msg>;
  type MessageMut<'msg> = SelectedTripsMut<'msg>;
}

impl ::std::default::Default for SelectedTrips {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for SelectedTrips {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `SelectedTrips` is `Sync` because it does not implement interior mutability.
//    Neither does `SelectedTripsMut`.
unsafe impl ::std::marker::Sync for SelectedTrips {}

// SAFETY:
// - `SelectedTrips` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for SelectedTrips {}

impl ::protobuf::Proxied for SelectedTrips {
  type View<'msg> = SelectedTripsView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for SelectedTrips {}

impl ::protobuf::MutProxied for SelectedTrips {
  type Mut<'msg> = SelectedTripsMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct SelectedTripsView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SelectedTrips>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SelectedTripsView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for SelectedTripsView<'msg> {
  type Message = SelectedTrips;
}

impl ::std::fmt::Debug for SelectedTripsView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for SelectedTripsView<'_> {
  fn default() -> SelectedTripsView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, SelectedTrips>> for SelectedTripsView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, SelectedTrips>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SelectedTripsView<'msg> {

  pub fn to_owned(&self) -> SelectedTrips {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // trip_ids: repeated string
  pub fn trip_ids(self) -> ::protobuf::RepeatedView<'msg, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

  // shape_id: optional string
  pub fn has_shape_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn shape_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `SelectedTripsView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for SelectedTripsView<'_> {}

// SAFETY:
// - `SelectedTripsView` is `Send` because while its alive a `SelectedTripsMut` cannot.
// - `SelectedTripsView` does not use thread-local data.
unsafe impl ::std::marker::Send for SelectedTripsView<'_> {}

impl<'msg> ::protobuf::AsView for SelectedTripsView<'msg> {
  type Proxied = SelectedTrips;
  fn as_view(&self) -> ::protobuf::View<'msg, SelectedTrips> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelectedTripsView<'msg> {
  fn into_view<'shorter>(self) -> SelectedTripsView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<SelectedTrips> for SelectedTripsView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SelectedTrips {
    let mut dst = SelectedTrips::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<SelectedTrips> for SelectedTripsMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> SelectedTrips {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for SelectedTrips {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SelectedTripsView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for SelectedTripsMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct SelectedTripsMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SelectedTrips>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for SelectedTripsMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for SelectedTripsMut<'msg> {
  type Message = SelectedTrips;
}

impl ::std::fmt::Debug for SelectedTripsMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, SelectedTrips>> for SelectedTripsMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, SelectedTrips>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> SelectedTripsMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, SelectedTrips> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> SelectedTrips {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // trip_ids: repeated string
  pub fn trip_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn trip_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_trip_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // shape_id: optional string
  pub fn has_shape_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_shape_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn shape_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shape_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `SelectedTripsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for SelectedTripsMut<'_> {}

// SAFETY:
// - `SelectedTripsMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for SelectedTripsMut<'_> {}

impl<'msg> ::protobuf::AsView for SelectedTripsMut<'msg> {
  type Proxied = SelectedTrips;
  fn as_view(&self) -> ::protobuf::View<'_, SelectedTrips> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for SelectedTripsMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, SelectedTrips>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for SelectedTripsMut<'msg> {
  type MutProxied = SelectedTrips;
  fn as_mut(&mut self) -> SelectedTripsMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for SelectedTripsMut<'msg> {
  fn into_mut<'shorter>(self) -> SelectedTripsMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl SelectedTrips {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, SelectedTrips> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> SelectedTripsView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> SelectedTripsMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // trip_ids: repeated string
  pub fn trip_ids(&self) -> ::protobuf::RepeatedView<'_, ::protobuf::ProtoString> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        0
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<::protobuf::ProtoString>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn trip_ids_mut(&mut self) -> ::protobuf::RepeatedMut<'_, ::protobuf::ProtoString> {
    unsafe {
      let raw_array = self.inner.ptr_mut().get_or_create_mutable_array_at_index(
        0,
        self.inner.arena()
      ).expect("alloc should not fail");
      ::protobuf::RepeatedMut::from_inner(
        ::protobuf::__internal::Private,
        ::protobuf::__internal::runtime::InnerRepeatedMut::new(
          raw_array, self.inner.arena(),
        ),
      )
    }
  }
  pub fn set_trip_ids(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<::protobuf::ProtoString>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        src);
    }
  }

  // shape_id: optional string
  pub fn has_shape_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_shape_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn shape_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_shape_id().then(|| self.shape_id())
  }
  pub fn shape_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_shape_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl SelectedTrips

impl ::std::ops::Drop for SelectedTrips {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for SelectedTrips {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for SelectedTrips {
  type Proxied = Self;
  fn as_view(&self) -> SelectedTripsView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for SelectedTrips {
  type MutProxied = Self;
  fn as_mut(&mut self) -> SelectedTripsMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for SelectedTrips {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::super::trip_modifications::transit_0realtime__TripModifications__SelectedTrips_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$PE1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::super::trip_modifications::transit_0realtime__TripModifications__SelectedTrips_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::super::trip_modifications::transit_0realtime__TripModifications__SelectedTrips_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SelectedTrips {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SelectedTrips {
  type Msg = SelectedTrips;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelectedTrips> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelectedTrips {
  type Msg = SelectedTrips;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelectedTrips> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for SelectedTripsMut<'_> {
  type Msg = SelectedTrips;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelectedTrips> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelectedTripsMut<'_> {
  type Msg = SelectedTrips;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelectedTrips> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for SelectedTripsView<'_> {
  type Msg = SelectedTrips;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<SelectedTrips> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for SelectedTripsMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



}  // pub mod trip_modifications


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__StopSelector_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct StopSelector {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<StopSelector>
}

impl ::protobuf::Message for StopSelector {
  type MessageView<'msg> = StopSelectorView<'msg>;
  type MessageMut<'msg> = StopSelectorMut<'msg>;
}

impl ::std::default::Default for StopSelector {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for StopSelector {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `StopSelector` is `Sync` because it does not implement interior mutability.
//    Neither does `StopSelectorMut`.
unsafe impl ::std::marker::Sync for StopSelector {}

// SAFETY:
// - `StopSelector` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for StopSelector {}

impl ::protobuf::Proxied for StopSelector {
  type View<'msg> = StopSelectorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for StopSelector {}

impl ::protobuf::MutProxied for StopSelector {
  type Mut<'msg> = StopSelectorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct StopSelectorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopSelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopSelectorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for StopSelectorView<'msg> {
  type Message = StopSelector;
}

impl ::std::fmt::Debug for StopSelectorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for StopSelectorView<'_> {
  fn default() -> StopSelectorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, StopSelector>> for StopSelectorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, StopSelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopSelectorView<'msg> {

  pub fn to_owned(&self) -> StopSelector {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // stop_sequence: optional uint32
  pub fn has_stop_sequence(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn stop_sequence_opt(self) -> ::std::option::Option<u32> {
    self.has_stop_sequence().then(|| self.stop_sequence())
  }
  pub fn stop_sequence(self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `StopSelectorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for StopSelectorView<'_> {}

// SAFETY:
// - `StopSelectorView` is `Send` because while its alive a `StopSelectorMut` cannot.
// - `StopSelectorView` does not use thread-local data.
unsafe impl ::std::marker::Send for StopSelectorView<'_> {}

impl<'msg> ::protobuf::AsView for StopSelectorView<'msg> {
  type Proxied = StopSelector;
  fn as_view(&self) -> ::protobuf::View<'msg, StopSelector> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopSelectorView<'msg> {
  fn into_view<'shorter>(self) -> StopSelectorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<StopSelector> for StopSelectorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopSelector {
    let mut dst = StopSelector::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<StopSelector> for StopSelectorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> StopSelector {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for StopSelector {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopSelectorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for StopSelectorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct StopSelectorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopSelector>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for StopSelectorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for StopSelectorMut<'msg> {
  type Message = StopSelector;
}

impl ::std::fmt::Debug for StopSelectorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, StopSelector>> for StopSelectorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, StopSelector>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> StopSelectorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, StopSelector> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> StopSelector {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // stop_sequence: optional uint32
  pub fn has_stop_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stop_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stop_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_stop_sequence().then(|| self.stop_sequence())
  }
  pub fn stop_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `StopSelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for StopSelectorMut<'_> {}

// SAFETY:
// - `StopSelectorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for StopSelectorMut<'_> {}

impl<'msg> ::protobuf::AsView for StopSelectorMut<'msg> {
  type Proxied = StopSelector;
  fn as_view(&self) -> ::protobuf::View<'_, StopSelector> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for StopSelectorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, StopSelector>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for StopSelectorMut<'msg> {
  type MutProxied = StopSelector;
  fn as_mut(&mut self) -> StopSelectorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for StopSelectorMut<'msg> {
  fn into_mut<'shorter>(self) -> StopSelectorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl StopSelector {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, StopSelector> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> StopSelectorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> StopSelectorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // stop_sequence: optional uint32
  pub fn has_stop_sequence(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_stop_sequence(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn stop_sequence_opt(&self) -> ::std::option::Option<u32> {
    self.has_stop_sequence().then(|| self.stop_sequence())
  }
  pub fn stop_sequence(&self) -> u32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_u32_at_index(
        0, (0u32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_stop_sequence(&mut self, val: u32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_u32_at_index(
        0, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl StopSelector

impl ::std::ops::Drop for StopSelector {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for StopSelector {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for StopSelector {
  type Proxied = Self;
  fn as_view(&self) -> StopSelectorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for StopSelector {
  type MutProxied = Self;
  fn as_mut(&mut self) -> StopSelectorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for StopSelector {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__StopSelector_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P)1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__StopSelector_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__StopSelector_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopSelector {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopSelector {
  type Msg = StopSelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopSelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopSelector {
  type Msg = StopSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopSelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for StopSelectorMut<'_> {
  type Msg = StopSelector;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopSelector> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopSelectorMut<'_> {
  type Msg = StopSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopSelector> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for StopSelectorView<'_> {
  type Msg = StopSelector;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<StopSelector> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for StopSelectorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__ReplacementStop_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct ReplacementStop {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<ReplacementStop>
}

impl ::protobuf::Message for ReplacementStop {
  type MessageView<'msg> = ReplacementStopView<'msg>;
  type MessageMut<'msg> = ReplacementStopMut<'msg>;
}

impl ::std::default::Default for ReplacementStop {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for ReplacementStop {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `ReplacementStop` is `Sync` because it does not implement interior mutability.
//    Neither does `ReplacementStopMut`.
unsafe impl ::std::marker::Sync for ReplacementStop {}

// SAFETY:
// - `ReplacementStop` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for ReplacementStop {}

impl ::protobuf::Proxied for ReplacementStop {
  type View<'msg> = ReplacementStopView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for ReplacementStop {}

impl ::protobuf::MutProxied for ReplacementStop {
  type Mut<'msg> = ReplacementStopMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct ReplacementStopView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReplacementStop>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReplacementStopView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for ReplacementStopView<'msg> {
  type Message = ReplacementStop;
}

impl ::std::fmt::Debug for ReplacementStopView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for ReplacementStopView<'_> {
  fn default() -> ReplacementStopView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, ReplacementStop>> for ReplacementStopView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, ReplacementStop>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReplacementStopView<'msg> {

  pub fn to_owned(&self) -> ReplacementStop {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // travel_time_to_stop: optional int32
  pub fn has_travel_time_to_stop(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn travel_time_to_stop_opt(self) -> ::std::option::Option<i32> {
    self.has_travel_time_to_stop().then(|| self.travel_time_to_stop())
  }
  pub fn travel_time_to_stop(self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn stop_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `ReplacementStopView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for ReplacementStopView<'_> {}

// SAFETY:
// - `ReplacementStopView` is `Send` because while its alive a `ReplacementStopMut` cannot.
// - `ReplacementStopView` does not use thread-local data.
unsafe impl ::std::marker::Send for ReplacementStopView<'_> {}

impl<'msg> ::protobuf::AsView for ReplacementStopView<'msg> {
  type Proxied = ReplacementStop;
  fn as_view(&self) -> ::protobuf::View<'msg, ReplacementStop> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReplacementStopView<'msg> {
  fn into_view<'shorter>(self) -> ReplacementStopView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<ReplacementStop> for ReplacementStopView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReplacementStop {
    let mut dst = ReplacementStop::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<ReplacementStop> for ReplacementStopMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> ReplacementStop {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for ReplacementStop {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ReplacementStopView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for ReplacementStopMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct ReplacementStopMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReplacementStop>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for ReplacementStopMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for ReplacementStopMut<'msg> {
  type Message = ReplacementStop;
}

impl ::std::fmt::Debug for ReplacementStopMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, ReplacementStop>> for ReplacementStopMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, ReplacementStop>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> ReplacementStopMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, ReplacementStop> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> ReplacementStop {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // travel_time_to_stop: optional int32
  pub fn has_travel_time_to_stop(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_travel_time_to_stop(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn travel_time_to_stop_opt(&self) -> ::std::option::Option<i32> {
    self.has_travel_time_to_stop().then(|| self.travel_time_to_stop())
  }
  pub fn travel_time_to_stop(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_travel_time_to_stop(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `ReplacementStopMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for ReplacementStopMut<'_> {}

// SAFETY:
// - `ReplacementStopMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for ReplacementStopMut<'_> {}

impl<'msg> ::protobuf::AsView for ReplacementStopMut<'msg> {
  type Proxied = ReplacementStop;
  fn as_view(&self) -> ::protobuf::View<'_, ReplacementStop> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for ReplacementStopMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, ReplacementStop>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for ReplacementStopMut<'msg> {
  type MutProxied = ReplacementStop;
  fn as_mut(&mut self) -> ReplacementStopMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for ReplacementStopMut<'msg> {
  fn into_mut<'shorter>(self) -> ReplacementStopMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl ReplacementStop {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, ReplacementStop> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> ReplacementStopView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> ReplacementStopMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // travel_time_to_stop: optional int32
  pub fn has_travel_time_to_stop(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_travel_time_to_stop(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn travel_time_to_stop_opt(&self) -> ::std::option::Option<i32> {
    self.has_travel_time_to_stop().then(|| self.travel_time_to_stop())
  }
  pub fn travel_time_to_stop(&self) -> i32 {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        0, (0i32).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_travel_time_to_stop(&mut self, val: i32) {
    unsafe {
      // TODO: b/361751487: This .into() is only here
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      //perfectly.
      self.inner.ptr_mut().set_base_field_i32_at_index(
        0, val.into()
      )
    }
  }

  // stop_id: optional string
  pub fn has_stop_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_stop_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn stop_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_stop_id().then(|| self.stop_id())
  }
  pub fn stop_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_stop_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl ReplacementStop

impl ::std::ops::Drop for ReplacementStop {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for ReplacementStop {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for ReplacementStop {
  type Proxied = Self;
  fn as_view(&self) -> ReplacementStopView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for ReplacementStop {
  type MutProxied = Self;
  fn as_mut(&mut self) -> ReplacementStopMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for ReplacementStop {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__ReplacementStop_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$P(1");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__ReplacementStop_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__ReplacementStop_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReplacementStop {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReplacementStop {
  type Msg = ReplacementStop;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReplacementStop> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReplacementStop {
  type Msg = ReplacementStop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReplacementStop> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for ReplacementStopMut<'_> {
  type Msg = ReplacementStop;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReplacementStop> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReplacementStopMut<'_> {
  type Msg = ReplacementStop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReplacementStop> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for ReplacementStopView<'_> {
  type Msg = ReplacementStop;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<ReplacementStop> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for ReplacementStopMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



