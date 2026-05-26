const _: () = ::protobuf::__internal::assert_compatible_gencode_version("4.35.0-release");
// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__TripReplacementPeriod_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct TripReplacementPeriod {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<TripReplacementPeriod>
}

impl ::protobuf::Message for TripReplacementPeriod {
  type MessageView<'msg> = TripReplacementPeriodView<'msg>;
  type MessageMut<'msg> = TripReplacementPeriodMut<'msg>;
}

impl ::std::default::Default for TripReplacementPeriod {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for TripReplacementPeriod {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `TripReplacementPeriod` is `Sync` because it does not implement interior mutability.
//    Neither does `TripReplacementPeriodMut`.
unsafe impl ::std::marker::Sync for TripReplacementPeriod {}

// SAFETY:
// - `TripReplacementPeriod` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for TripReplacementPeriod {}

impl ::protobuf::Proxied for TripReplacementPeriod {
  type View<'msg> = TripReplacementPeriodView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for TripReplacementPeriod {}

impl ::protobuf::MutProxied for TripReplacementPeriod {
  type Mut<'msg> = TripReplacementPeriodMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TripReplacementPeriodView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripReplacementPeriod>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripReplacementPeriodView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for TripReplacementPeriodView<'msg> {
  type Message = TripReplacementPeriod;
}

impl ::std::fmt::Debug for TripReplacementPeriodView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for TripReplacementPeriodView<'_> {
  fn default() -> TripReplacementPeriodView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, TripReplacementPeriod>> for TripReplacementPeriodView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, TripReplacementPeriod>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripReplacementPeriodView<'msg> {

  pub fn to_owned(&self) -> TripReplacementPeriod {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // route_id: optional string
  pub fn has_route_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn route_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // replacement_period: optional message transit_realtime.TimeRange
  pub fn has_replacement_period(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn replacement_period_opt(self) -> ::std::option::Option<super::TimeRangeView<'msg>> {
    self.has_replacement_period().then(|| self.replacement_period())
  }
  pub fn replacement_period(self) -> super::TimeRangeView<'msg> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimeRangeView::default())
  }

}

// SAFETY:
// - `TripReplacementPeriodView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for TripReplacementPeriodView<'_> {}

// SAFETY:
// - `TripReplacementPeriodView` is `Send` because while its alive a `TripReplacementPeriodMut` cannot.
// - `TripReplacementPeriodView` does not use thread-local data.
unsafe impl ::std::marker::Send for TripReplacementPeriodView<'_> {}

impl<'msg> ::protobuf::AsView for TripReplacementPeriodView<'msg> {
  type Proxied = TripReplacementPeriod;
  fn as_view(&self) -> ::protobuf::View<'msg, TripReplacementPeriod> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripReplacementPeriodView<'msg> {
  fn into_view<'shorter>(self) -> TripReplacementPeriodView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<TripReplacementPeriod> for TripReplacementPeriodView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripReplacementPeriod {
    let mut dst = TripReplacementPeriod::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<TripReplacementPeriod> for TripReplacementPeriodMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> TripReplacementPeriod {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for TripReplacementPeriod {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripReplacementPeriodView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for TripReplacementPeriodMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct TripReplacementPeriodMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripReplacementPeriod>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for TripReplacementPeriodMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for TripReplacementPeriodMut<'msg> {
  type Message = TripReplacementPeriod;
}

impl ::std::fmt::Debug for TripReplacementPeriodMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, TripReplacementPeriod>> for TripReplacementPeriodMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, TripReplacementPeriod>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> TripReplacementPeriodMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, TripReplacementPeriod> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> TripReplacementPeriod {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // route_id: optional string
  pub fn has_route_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_route_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn route_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // replacement_period: optional message transit_realtime.TimeRange
  pub fn has_replacement_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_replacement_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn replacement_period_opt(&self) -> ::std::option::Option<super::TimeRangeView<'_>> {
    self.has_replacement_period().then(|| self.replacement_period())
  }
  pub fn replacement_period(&self) -> super::TimeRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimeRangeView::default())
  }
  pub fn replacement_period_mut(&mut self) -> super::TimeRangeMut<'_> {
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
  pub fn set_replacement_period(&mut self,
    val: impl ::protobuf::IntoProxied<super::TimeRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}

// SAFETY:
// - `TripReplacementPeriodMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for TripReplacementPeriodMut<'_> {}

// SAFETY:
// - `TripReplacementPeriodMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for TripReplacementPeriodMut<'_> {}

impl<'msg> ::protobuf::AsView for TripReplacementPeriodMut<'msg> {
  type Proxied = TripReplacementPeriod;
  fn as_view(&self) -> ::protobuf::View<'_, TripReplacementPeriod> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for TripReplacementPeriodMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, TripReplacementPeriod>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for TripReplacementPeriodMut<'msg> {
  type MutProxied = TripReplacementPeriod;
  fn as_mut(&mut self) -> TripReplacementPeriodMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for TripReplacementPeriodMut<'msg> {
  fn into_mut<'shorter>(self) -> TripReplacementPeriodMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl TripReplacementPeriod {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, TripReplacementPeriod> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> TripReplacementPeriodView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> TripReplacementPeriodMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // route_id: optional string
  pub fn has_route_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_route_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn route_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_route_id().then(|| self.route_id())
  }
  pub fn route_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_route_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // replacement_period: optional message transit_realtime.TimeRange
  pub fn has_replacement_period(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_replacement_period(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn replacement_period_opt(&self) -> ::std::option::Option<super::TimeRangeView<'_>> {
    self.has_replacement_period().then(|| self.replacement_period())
  }
  pub fn replacement_period(&self) -> super::TimeRangeView<'_> {
    let submsg = unsafe {
      self.inner.ptr().get_message_at_index(1)
    };
    submsg
        .map(|ptr| unsafe { ::protobuf::__internal::runtime::MessageViewInner::wrap(ptr).into() })
       .unwrap_or(super::TimeRangeView::default())
  }
  pub fn replacement_period_mut(&mut self) -> super::TimeRangeMut<'_> {
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
  pub fn set_replacement_period(&mut self,
    val: impl ::protobuf::IntoProxied<super::TimeRange>) {

    unsafe {
      ::protobuf::__internal::runtime::message_set_sub_message(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val
      );
    }
  }

}  // impl TripReplacementPeriod

impl ::std::ops::Drop for TripReplacementPeriod {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for TripReplacementPeriod {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for TripReplacementPeriod {
  type Proxied = Self;
  fn as_view(&self) -> TripReplacementPeriodView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for TripReplacementPeriod {
  type MutProxied = Self;
  fn as_mut(&mut self) -> TripReplacementPeriodMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for TripReplacementPeriod {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__TripReplacementPeriod_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$13");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__TripReplacementPeriod_msg_init.0, &[<super::TimeRange as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__TripReplacementPeriod_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripReplacementPeriod {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripReplacementPeriod {
  type Msg = TripReplacementPeriod;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripReplacementPeriod> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripReplacementPeriod {
  type Msg = TripReplacementPeriod;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripReplacementPeriod> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for TripReplacementPeriodMut<'_> {
  type Msg = TripReplacementPeriod;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripReplacementPeriod> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripReplacementPeriodMut<'_> {
  type Msg = TripReplacementPeriod;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripReplacementPeriod> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for TripReplacementPeriodView<'_> {
  type Msg = TripReplacementPeriod;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<TripReplacementPeriod> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for TripReplacementPeriodMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__NyctFeedHeader_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NyctFeedHeader {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NyctFeedHeader>
}

impl ::protobuf::Message for NyctFeedHeader {
  type MessageView<'msg> = NyctFeedHeaderView<'msg>;
  type MessageMut<'msg> = NyctFeedHeaderMut<'msg>;
}

impl ::std::default::Default for NyctFeedHeader {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NyctFeedHeader {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NyctFeedHeader` is `Sync` because it does not implement interior mutability.
//    Neither does `NyctFeedHeaderMut`.
unsafe impl ::std::marker::Sync for NyctFeedHeader {}

// SAFETY:
// - `NyctFeedHeader` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NyctFeedHeader {}

impl ::protobuf::Proxied for NyctFeedHeader {
  type View<'msg> = NyctFeedHeaderView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NyctFeedHeader {}

impl ::protobuf::MutProxied for NyctFeedHeader {
  type Mut<'msg> = NyctFeedHeaderMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NyctFeedHeaderView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NyctFeedHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NyctFeedHeaderView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NyctFeedHeaderView<'msg> {
  type Message = NyctFeedHeader;
}

impl ::std::fmt::Debug for NyctFeedHeaderView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NyctFeedHeaderView<'_> {
  fn default() -> NyctFeedHeaderView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NyctFeedHeader>> for NyctFeedHeaderView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NyctFeedHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NyctFeedHeaderView<'msg> {

  pub fn to_owned(&self) -> NyctFeedHeader {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // nyct_subway_version: optional string
  pub fn has_nyct_subway_version(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn nyct_subway_version_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_nyct_subway_version().then(|| self.nyct_subway_version())
  }
  pub fn nyct_subway_version(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // trip_replacement_period: repeated message transit_realtime.TripReplacementPeriod
  pub fn trip_replacement_period(self) -> ::protobuf::RepeatedView<'msg, super::TripReplacementPeriod> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TripReplacementPeriod>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }

}

// SAFETY:
// - `NyctFeedHeaderView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NyctFeedHeaderView<'_> {}

// SAFETY:
// - `NyctFeedHeaderView` is `Send` because while its alive a `NyctFeedHeaderMut` cannot.
// - `NyctFeedHeaderView` does not use thread-local data.
unsafe impl ::std::marker::Send for NyctFeedHeaderView<'_> {}

impl<'msg> ::protobuf::AsView for NyctFeedHeaderView<'msg> {
  type Proxied = NyctFeedHeader;
  fn as_view(&self) -> ::protobuf::View<'msg, NyctFeedHeader> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NyctFeedHeaderView<'msg> {
  fn into_view<'shorter>(self) -> NyctFeedHeaderView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NyctFeedHeader> for NyctFeedHeaderView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NyctFeedHeader {
    let mut dst = NyctFeedHeader::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NyctFeedHeader> for NyctFeedHeaderMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NyctFeedHeader {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NyctFeedHeader {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NyctFeedHeaderView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NyctFeedHeaderMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NyctFeedHeaderMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctFeedHeader>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NyctFeedHeaderMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NyctFeedHeaderMut<'msg> {
  type Message = NyctFeedHeader;
}

impl ::std::fmt::Debug for NyctFeedHeaderMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NyctFeedHeader>> for NyctFeedHeaderMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctFeedHeader>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NyctFeedHeaderMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctFeedHeader> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NyctFeedHeader {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // nyct_subway_version: optional string
  pub fn has_nyct_subway_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_nyct_subway_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn nyct_subway_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_nyct_subway_version().then(|| self.nyct_subway_version())
  }
  pub fn nyct_subway_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_nyct_subway_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // trip_replacement_period: repeated message transit_realtime.TripReplacementPeriod
  pub fn trip_replacement_period(&self) -> ::protobuf::RepeatedView<'_, super::TripReplacementPeriod> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TripReplacementPeriod>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn trip_replacement_period_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TripReplacementPeriod> {
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
  pub fn set_trip_replacement_period(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TripReplacementPeriod>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}

// SAFETY:
// - `NyctFeedHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NyctFeedHeaderMut<'_> {}

// SAFETY:
// - `NyctFeedHeaderMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NyctFeedHeaderMut<'_> {}

impl<'msg> ::protobuf::AsView for NyctFeedHeaderMut<'msg> {
  type Proxied = NyctFeedHeader;
  fn as_view(&self) -> ::protobuf::View<'_, NyctFeedHeader> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NyctFeedHeaderMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NyctFeedHeader>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NyctFeedHeaderMut<'msg> {
  type MutProxied = NyctFeedHeader;
  fn as_mut(&mut self) -> NyctFeedHeaderMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NyctFeedHeaderMut<'msg> {
  fn into_mut<'shorter>(self) -> NyctFeedHeaderMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NyctFeedHeader {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NyctFeedHeader> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NyctFeedHeaderView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NyctFeedHeaderMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // nyct_subway_version: optional string
  pub fn has_nyct_subway_version(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_nyct_subway_version(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn nyct_subway_version_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_nyct_subway_version().then(|| self.nyct_subway_version())
  }
  pub fn nyct_subway_version(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_nyct_subway_version(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // trip_replacement_period: repeated message transit_realtime.TripReplacementPeriod
  pub fn trip_replacement_period(&self) -> ::protobuf::RepeatedView<'_, super::TripReplacementPeriod> {
    unsafe {
      self.inner.ptr().get_array_at_index(
        1
      )
    }.map_or_else(
        ::protobuf::__internal::runtime::empty_array::<super::TripReplacementPeriod>,
        |raw| unsafe {
          ::protobuf::RepeatedView::from_raw(::protobuf::__internal::Private, raw)
        }
      )
  }
  pub fn trip_replacement_period_mut(&mut self) -> ::protobuf::RepeatedMut<'_, super::TripReplacementPeriod> {
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
  pub fn set_trip_replacement_period(&mut self, src: impl ::protobuf::IntoProxied<::protobuf::Repeated<super::TripReplacementPeriod>>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_repeated_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        src);
    }
  }

}  // impl NyctFeedHeader

impl ::std::ops::Drop for NyctFeedHeader {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NyctFeedHeader {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NyctFeedHeader {
  type Proxied = Self;
  fn as_view(&self) -> NyctFeedHeaderView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NyctFeedHeader {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NyctFeedHeaderMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NyctFeedHeader {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__NyctFeedHeader_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1NG");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__NyctFeedHeader_msg_init.0, &[<super::TripReplacementPeriod as ::protobuf::__internal::runtime::AssociatedMiniTable>::mini_table(),
            ], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__NyctFeedHeader_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NyctFeedHeader {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NyctFeedHeader {
  type Msg = NyctFeedHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctFeedHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctFeedHeader {
  type Msg = NyctFeedHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctFeedHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NyctFeedHeaderMut<'_> {
  type Msg = NyctFeedHeader;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctFeedHeader> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctFeedHeaderMut<'_> {
  type Msg = NyctFeedHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctFeedHeader> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctFeedHeaderView<'_> {
  type Msg = NyctFeedHeader;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctFeedHeader> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NyctFeedHeaderMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}



// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__NyctTripDescriptor_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NyctTripDescriptor {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NyctTripDescriptor>
}

impl ::protobuf::Message for NyctTripDescriptor {
  type MessageView<'msg> = NyctTripDescriptorView<'msg>;
  type MessageMut<'msg> = NyctTripDescriptorMut<'msg>;
}

impl ::std::default::Default for NyctTripDescriptor {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NyctTripDescriptor {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NyctTripDescriptor` is `Sync` because it does not implement interior mutability.
//    Neither does `NyctTripDescriptorMut`.
unsafe impl ::std::marker::Sync for NyctTripDescriptor {}

// SAFETY:
// - `NyctTripDescriptor` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NyctTripDescriptor {}

impl ::protobuf::Proxied for NyctTripDescriptor {
  type View<'msg> = NyctTripDescriptorView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NyctTripDescriptor {}

impl ::protobuf::MutProxied for NyctTripDescriptor {
  type Mut<'msg> = NyctTripDescriptorMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NyctTripDescriptorView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NyctTripDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NyctTripDescriptorView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NyctTripDescriptorView<'msg> {
  type Message = NyctTripDescriptor;
}

impl ::std::fmt::Debug for NyctTripDescriptorView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NyctTripDescriptorView<'_> {
  fn default() -> NyctTripDescriptorView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NyctTripDescriptor>> for NyctTripDescriptorView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NyctTripDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NyctTripDescriptorView<'msg> {

  pub fn to_owned(&self) -> NyctTripDescriptor {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // train_id: optional string
  pub fn has_train_id(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn train_id_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_train_id().then(|| self.train_id())
  }
  pub fn train_id(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // is_assigned: optional bool
  pub fn has_is_assigned(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn is_assigned_opt(self) -> ::std::option::Option<bool> {
    self.has_is_assigned().then(|| self.is_assigned())
  }
  pub fn is_assigned(self) -> bool {
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

  // direction: optional enum transit_realtime.NyctTripDescriptor.Direction
  pub fn has_direction(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn direction_opt(self) -> ::std::option::Option<super::nyct_trip_descriptor::Direction> {
    self.has_direction().then(|| self.direction())
  }
  pub fn direction(self) -> super::nyct_trip_descriptor::Direction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::nyct_trip_descriptor::Direction::North).into()
      ).try_into().unwrap()
    }
  }

}

// SAFETY:
// - `NyctTripDescriptorView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NyctTripDescriptorView<'_> {}

// SAFETY:
// - `NyctTripDescriptorView` is `Send` because while its alive a `NyctTripDescriptorMut` cannot.
// - `NyctTripDescriptorView` does not use thread-local data.
unsafe impl ::std::marker::Send for NyctTripDescriptorView<'_> {}

impl<'msg> ::protobuf::AsView for NyctTripDescriptorView<'msg> {
  type Proxied = NyctTripDescriptor;
  fn as_view(&self) -> ::protobuf::View<'msg, NyctTripDescriptor> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NyctTripDescriptorView<'msg> {
  fn into_view<'shorter>(self) -> NyctTripDescriptorView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NyctTripDescriptor> for NyctTripDescriptorView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NyctTripDescriptor {
    let mut dst = NyctTripDescriptor::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NyctTripDescriptor> for NyctTripDescriptorMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NyctTripDescriptor {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NyctTripDescriptor {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NyctTripDescriptorView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NyctTripDescriptorMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NyctTripDescriptorMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctTripDescriptor>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NyctTripDescriptorMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NyctTripDescriptorMut<'msg> {
  type Message = NyctTripDescriptor;
}

impl ::std::fmt::Debug for NyctTripDescriptorMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NyctTripDescriptor>> for NyctTripDescriptorMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctTripDescriptor>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NyctTripDescriptorMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctTripDescriptor> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NyctTripDescriptor {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // train_id: optional string
  pub fn has_train_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_train_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn train_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_train_id().then(|| self.train_id())
  }
  pub fn train_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_train_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // is_assigned: optional bool
  pub fn has_is_assigned(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_is_assigned(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn is_assigned_opt(&self) -> ::std::option::Option<bool> {
    self.has_is_assigned().then(|| self.is_assigned())
  }
  pub fn is_assigned(&self) -> bool {
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
  pub fn set_is_assigned(&mut self, val: bool) {
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

  // direction: optional enum transit_realtime.NyctTripDescriptor.Direction
  pub fn has_direction(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_direction(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn direction_opt(&self) -> ::std::option::Option<super::nyct_trip_descriptor::Direction> {
    self.has_direction().then(|| self.direction())
  }
  pub fn direction(&self) -> super::nyct_trip_descriptor::Direction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::nyct_trip_descriptor::Direction::North).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_direction(&mut self, val: super::nyct_trip_descriptor::Direction) {
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

}

// SAFETY:
// - `NyctTripDescriptorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NyctTripDescriptorMut<'_> {}

// SAFETY:
// - `NyctTripDescriptorMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NyctTripDescriptorMut<'_> {}

impl<'msg> ::protobuf::AsView for NyctTripDescriptorMut<'msg> {
  type Proxied = NyctTripDescriptor;
  fn as_view(&self) -> ::protobuf::View<'_, NyctTripDescriptor> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NyctTripDescriptorMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NyctTripDescriptor>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NyctTripDescriptorMut<'msg> {
  type MutProxied = NyctTripDescriptor;
  fn as_mut(&mut self) -> NyctTripDescriptorMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NyctTripDescriptorMut<'msg> {
  fn into_mut<'shorter>(self) -> NyctTripDescriptorMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NyctTripDescriptor {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NyctTripDescriptor> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NyctTripDescriptorView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NyctTripDescriptorMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // train_id: optional string
  pub fn has_train_id(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_train_id(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn train_id_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_train_id().then(|| self.train_id())
  }
  pub fn train_id(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_train_id(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // is_assigned: optional bool
  pub fn has_is_assigned(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_is_assigned(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn is_assigned_opt(&self) -> ::std::option::Option<bool> {
    self.has_is_assigned().then(|| self.is_assigned())
  }
  pub fn is_assigned(&self) -> bool {
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
  pub fn set_is_assigned(&mut self, val: bool) {
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

  // direction: optional enum transit_realtime.NyctTripDescriptor.Direction
  pub fn has_direction(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(2)
    }
  }
  pub fn clear_direction(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        2
      );
    }
  }
  pub fn direction_opt(&self) -> ::std::option::Option<super::nyct_trip_descriptor::Direction> {
    self.has_direction().then(|| self.direction())
  }
  pub fn direction(&self) -> super::nyct_trip_descriptor::Direction {
    unsafe {
      // TODO: b/361751487: This .into() and .try_into() is only
      // here for the enum<->i32 case, we should avoid it for
      // other primitives where the types naturally match
      // perfectly (and do an unchecked conversion for
      // i32->enum types, since even for closed enums we trust
      // upb to only return one of the named values).
      self.inner.ptr().get_i32_at_index(
        2, (super::nyct_trip_descriptor::Direction::North).into()
      ).try_into().unwrap()
    }
  }
  pub fn set_direction(&mut self, val: super::nyct_trip_descriptor::Direction) {
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

}  // impl NyctTripDescriptor

impl ::std::ops::Drop for NyctTripDescriptor {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NyctTripDescriptor {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NyctTripDescriptor {
  type Proxied = Self;
  fn as_view(&self) -> NyctTripDescriptorView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NyctTripDescriptor {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NyctTripDescriptorMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NyctTripDescriptor {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__NyctTripDescriptor_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$1/4");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__NyctTripDescriptor_msg_init.0, &[], &[<super::nyct_trip_descriptor::Direction as ::protobuf::__internal::runtime::AssociatedMiniTableEnum>::mini_table(),
            ]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__NyctTripDescriptor_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NyctTripDescriptor {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NyctTripDescriptor {
  type Msg = NyctTripDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctTripDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctTripDescriptor {
  type Msg = NyctTripDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctTripDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NyctTripDescriptorMut<'_> {
  type Msg = NyctTripDescriptor;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctTripDescriptor> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctTripDescriptorMut<'_> {
  type Msg = NyctTripDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctTripDescriptor> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctTripDescriptorView<'_> {
  type Msg = NyctTripDescriptor;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctTripDescriptor> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NyctTripDescriptorMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

pub mod nyct_trip_descriptor {
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Direction(i32);

#[allow(non_upper_case_globals)]
impl Direction {
  pub const North: Direction = Direction(1);
  pub const East: Direction = Direction(2);
  pub const South: Direction = Direction(3);
  pub const West: Direction = Direction(4);

  fn constant_name(&self) -> ::std::option::Option<&'static str> {
    #[allow(unreachable_patterns)] // In the case of aliases, just emit them all and let the first one match.
    Some(match self.0 {
      1 => "North",
      2 => "East",
      3 => "South",
      4 => "West",
      _ => return None
    })
  }
}

impl ::std::convert::From<Direction> for i32 {
  fn from(val: Direction) -> i32 {
    val.0
  }
}

impl ::std::convert::TryFrom<i32> for Direction {
  type Error = ::protobuf::UnknownEnumValue<Self>;

  fn try_from(val: i32) -> ::std::result::Result<Direction, Self::Error> {
    if <Self as ::protobuf::__internal::Enum>::is_known(val) {
      Ok(Self(val))
    } else {
      Err(::protobuf::UnknownEnumValue::new(::protobuf::__internal::Private, val))
    }
  }
}

impl ::std::default::Default for Direction {
  fn default() -> Self {
    Self(1)
  }
}

impl ::std::fmt::Debug for Direction {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    if let Some(constant_name) = self.constant_name() {
      write!(f, "Direction::{}", constant_name)
    } else {
      write!(f, "Direction::from({})", self.0)
    }
  }
}

impl ::protobuf::IntoProxied<i32> for Direction {
  fn into_proxied(self, _: ::protobuf::__internal::Private) -> i32 {
    self.0
  }
}

impl ::protobuf::__internal::SealedInternal for Direction {}

impl ::protobuf::Proxied for Direction {
  type View<'a> = Direction;
}

impl ::protobuf::AsView for Direction {
  type Proxied = Direction;

  fn as_view(&self) -> Direction {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for Direction {
  fn into_view<'shorter>(self) -> Direction where 'msg: 'shorter {
    self
  }
}

// SAFETY: this is an enum type
unsafe impl ::protobuf::__internal::Enum for Direction {
  const NAME: &'static str = "Direction";

  fn is_known(value: i32) -> bool {
    matches!(value, 1|2|3|4)
  }
}

impl ::protobuf::__internal::EntityType for Direction {
    type Tag = ::protobuf::__internal::entity_tag::EnumTag;
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTableEnum for Direction {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTableEnumPtr {
    static MINI_TABLE: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableEnumInitPtr> =
        ::std::sync::OnceLock::new();
    MINI_TABLE.get_or_init(|| unsafe {
      ::protobuf::__internal::runtime::MiniTableEnumInitPtr(
          ::protobuf::__internal::runtime::build_enum_mini_table("!@"))
    }).0
  }
}

}  // pub mod nyct_trip_descriptor


// This variable must not be referenced except by protobuf generated
// code.
pub(crate) static mut transit_0realtime__NyctStopTimeUpdate_msg_init: ::protobuf::__internal::runtime::MiniTableInitPtr =
    ::protobuf::__internal::runtime::MiniTableInitPtr(::protobuf::__internal::runtime::MiniTablePtr::dangling());
#[allow(non_camel_case_types)]
pub struct NyctStopTimeUpdate {
  inner: ::protobuf::__internal::runtime::OwnedMessageInner<NyctStopTimeUpdate>
}

impl ::protobuf::Message for NyctStopTimeUpdate {
  type MessageView<'msg> = NyctStopTimeUpdateView<'msg>;
  type MessageMut<'msg> = NyctStopTimeUpdateMut<'msg>;
}

impl ::std::default::Default for NyctStopTimeUpdate {
  fn default() -> Self {
    Self::new()
  }
}

impl ::std::fmt::Debug for NyctStopTimeUpdate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

// SAFETY:
// - `NyctStopTimeUpdate` is `Sync` because it does not implement interior mutability.
//    Neither does `NyctStopTimeUpdateMut`.
unsafe impl ::std::marker::Sync for NyctStopTimeUpdate {}

// SAFETY:
// - `NyctStopTimeUpdate` is `Send` because it uniquely owns its arena and does
//   not use thread-local data.
unsafe impl ::std::marker::Send for NyctStopTimeUpdate {}

impl ::protobuf::Proxied for NyctStopTimeUpdate {
  type View<'msg> = NyctStopTimeUpdateView<'msg>;
}

impl ::protobuf::__internal::SealedInternal for NyctStopTimeUpdate {}

impl ::protobuf::MutProxied for NyctStopTimeUpdate {
  type Mut<'msg> = NyctStopTimeUpdateMut<'msg>;
}

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct NyctStopTimeUpdateView<'msg> {
  inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NyctStopTimeUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NyctStopTimeUpdateView<'msg> {}

impl<'msg> ::protobuf::MessageView<'msg> for NyctStopTimeUpdateView<'msg> {
  type Message = NyctStopTimeUpdate;
}

impl ::std::fmt::Debug for NyctStopTimeUpdateView<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl ::std::default::Default for NyctStopTimeUpdateView<'_> {
  fn default() -> NyctStopTimeUpdateView<'static> {
    ::protobuf::__internal::runtime::MessageViewInner::default().into()
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageViewInner<'msg, NyctStopTimeUpdate>> for NyctStopTimeUpdateView<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageViewInner<'msg, NyctStopTimeUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NyctStopTimeUpdateView<'msg> {

  pub fn to_owned(&self) -> NyctStopTimeUpdate {
    ::protobuf::IntoProxied::into_proxied(*self, ::protobuf::__internal::Private)
  }

  // scheduled_track: optional string
  pub fn has_scheduled_track(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn scheduled_track_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_scheduled_track().then(|| self.scheduled_track())
  }
  pub fn scheduled_track(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

  // actual_track: optional string
  pub fn has_actual_track(self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn actual_track_opt(self) -> ::std::option::Option<&'msg ::protobuf::ProtoStr> {
    self.has_actual_track().then(|| self.actual_track())
  }
  pub fn actual_track(self) -> ::protobuf::View<'msg, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }

}

// SAFETY:
// - `NyctStopTimeUpdateView` is `Sync` because it does not support mutation.
unsafe impl ::std::marker::Sync for NyctStopTimeUpdateView<'_> {}

// SAFETY:
// - `NyctStopTimeUpdateView` is `Send` because while its alive a `NyctStopTimeUpdateMut` cannot.
// - `NyctStopTimeUpdateView` does not use thread-local data.
unsafe impl ::std::marker::Send for NyctStopTimeUpdateView<'_> {}

impl<'msg> ::protobuf::AsView for NyctStopTimeUpdateView<'msg> {
  type Proxied = NyctStopTimeUpdate;
  fn as_view(&self) -> ::protobuf::View<'msg, NyctStopTimeUpdate> {
    *self
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NyctStopTimeUpdateView<'msg> {
  fn into_view<'shorter>(self) -> NyctStopTimeUpdateView<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

impl<'msg> ::protobuf::IntoProxied<NyctStopTimeUpdate> for NyctStopTimeUpdateView<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NyctStopTimeUpdate {
    let mut dst = NyctStopTimeUpdate::new();
    assert!(unsafe {
      dst.inner.ptr_mut().deep_copy(self.inner.ptr(), dst.inner.arena())
    });
    dst
  }
}

impl<'msg> ::protobuf::IntoProxied<NyctStopTimeUpdate> for NyctStopTimeUpdateMut<'msg> {
  fn into_proxied(self, _private: ::protobuf::__internal::Private) -> NyctStopTimeUpdate {
    ::protobuf::IntoProxied::into_proxied(::protobuf::IntoView::into_view(self), _private)
  }
}

impl ::protobuf::__internal::EntityType for NyctStopTimeUpdate {
    type Tag = ::protobuf::__internal::entity_tag::MessageTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NyctStopTimeUpdateView<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::ViewProxyTag;
}

impl<'msg> ::protobuf::__internal::EntityType for NyctStopTimeUpdateMut<'msg> {
    type Tag = ::protobuf::__internal::entity_tag::MutProxyTag;
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct NyctStopTimeUpdateMut<'msg> {
  inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctStopTimeUpdate>,
}

impl<'msg> ::protobuf::__internal::SealedInternal for NyctStopTimeUpdateMut<'msg> {}

impl<'msg> ::protobuf::MessageMut<'msg> for NyctStopTimeUpdateMut<'msg> {
  type Message = NyctStopTimeUpdate;
}

impl ::std::fmt::Debug for NyctStopTimeUpdateMut<'_> {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    write!(f, "{}", ::protobuf::__internal::runtime::debug_string(self))
  }
}

impl<'msg> From<::protobuf::__internal::runtime::MessageMutInner<'msg, NyctStopTimeUpdate>> for NyctStopTimeUpdateMut<'msg> {
  fn from(inner: ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctStopTimeUpdate>) -> Self {
    Self { inner }
  }
}

#[allow(dead_code)]
impl<'msg> NyctStopTimeUpdateMut<'msg> {

  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private)
    -> ::protobuf::__internal::runtime::MessageMutInner<'msg, NyctStopTimeUpdate> {
    self.inner.reborrow()
  }

  pub fn to_owned(&self) -> NyctStopTimeUpdate {
    ::protobuf::AsView::as_view(self).to_owned()
  }

  // scheduled_track: optional string
  pub fn has_scheduled_track(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_scheduled_track(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn scheduled_track_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_scheduled_track().then(|| self.scheduled_track())
  }
  pub fn scheduled_track(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheduled_track(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // actual_track: optional string
  pub fn has_actual_track(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_actual_track(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn actual_track_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_actual_track().then(|| self.actual_track())
  }
  pub fn actual_track(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_actual_track(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}

// SAFETY:
// - `NyctStopTimeUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Send for NyctStopTimeUpdateMut<'_> {}

// SAFETY:
// - `NyctStopTimeUpdateMut` does not perform any shared mutation.
unsafe impl ::std::marker::Sync for NyctStopTimeUpdateMut<'_> {}

impl<'msg> ::protobuf::AsView for NyctStopTimeUpdateMut<'msg> {
  type Proxied = NyctStopTimeUpdate;
  fn as_view(&self) -> ::protobuf::View<'_, NyctStopTimeUpdate> {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::IntoView<'msg> for NyctStopTimeUpdateMut<'msg> {
  fn into_view<'shorter>(self) -> ::protobuf::View<'shorter, NyctStopTimeUpdate>
  where
      'msg: 'shorter {
    self.inner.as_view().into()
  }
}

impl<'msg> ::protobuf::AsMut for NyctStopTimeUpdateMut<'msg> {
  type MutProxied = NyctStopTimeUpdate;
  fn as_mut(&mut self) -> NyctStopTimeUpdateMut<'msg> {
    self.inner.reborrow().into()
  }
}

impl<'msg> ::protobuf::IntoMut<'msg> for NyctStopTimeUpdateMut<'msg> {
  fn into_mut<'shorter>(self) -> NyctStopTimeUpdateMut<'shorter>
  where
      'msg: 'shorter {
    self
  }
}

#[allow(dead_code)]
impl NyctStopTimeUpdate {
  pub fn new() -> Self {
    Self { inner: ::protobuf::__internal::runtime::OwnedMessageInner::<Self>::new() }
  }


  #[doc(hidden)]
  pub fn as_message_mut_inner(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessageMutInner<'_, NyctStopTimeUpdate> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner)
  }

  pub fn as_view(&self) -> NyctStopTimeUpdateView<'_> {
    ::protobuf::__internal::runtime::MessageViewInner::view_of_owned(&self.inner).into()
  }

  pub fn as_mut(&mut self) -> NyctStopTimeUpdateMut<'_> {
    ::protobuf::__internal::runtime::MessageMutInner::mut_of_owned(&mut self.inner).into()
  }

  // scheduled_track: optional string
  pub fn has_scheduled_track(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(0)
    }
  }
  pub fn clear_scheduled_track(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        0
      );
    }
  }
  pub fn scheduled_track_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_scheduled_track().then(|| self.scheduled_track())
  }
  pub fn scheduled_track(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        0, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_scheduled_track(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        0,
        val);
    }
  }

  // actual_track: optional string
  pub fn has_actual_track(&self) -> bool {
    unsafe {
      self.inner.ptr().has_field_at_index(1)
    }
  }
  pub fn clear_actual_track(&mut self) {
    unsafe {
      self.inner.ptr().clear_field_at_index(
        1
      );
    }
  }
  pub fn actual_track_opt(&self) -> ::std::option::Option<&'_ ::protobuf::ProtoStr> {
    self.has_actual_track().then(|| self.actual_track())
  }
  pub fn actual_track(&self) -> ::protobuf::View<'_, ::protobuf::ProtoString> {
    let str_view = unsafe {
      self.inner.ptr().get_string_at_index(
        1, (b"").into()
      )
    };
    ::protobuf::ProtoStr::from_utf8_unchecked(unsafe { str_view.as_ref() })
  }
  pub fn set_actual_track(&mut self, val: impl ::protobuf::IntoProxied<::protobuf::ProtoString>) {
    unsafe {
      ::protobuf::__internal::runtime::message_set_string_field(
        ::protobuf::AsMut::as_mut(self).inner,
        1,
        val);
    }
  }

}  // impl NyctStopTimeUpdate

impl ::std::ops::Drop for NyctStopTimeUpdate {
  #[inline]
  fn drop(&mut self) {
  }
}

impl ::std::clone::Clone for NyctStopTimeUpdate {
  fn clone(&self) -> Self {
    self.as_view().to_owned()
  }
}

impl ::protobuf::AsView for NyctStopTimeUpdate {
  type Proxied = Self;
  fn as_view(&self) -> NyctStopTimeUpdateView<'_> {
    self.as_view()
  }
}

impl ::protobuf::AsMut for NyctStopTimeUpdate {
  type MutProxied = Self;
  fn as_mut(&mut self) -> NyctStopTimeUpdateMut<'_> {
    self.as_mut()
  }
}

unsafe impl ::protobuf::__internal::runtime::AssociatedMiniTable for NyctStopTimeUpdate {
  fn mini_table() -> ::protobuf::__internal::runtime::MiniTablePtr {
    static ONCE_LOCK: ::std::sync::OnceLock<::protobuf::__internal::runtime::MiniTableInitPtr> =
        ::std::sync::OnceLock::new();
    unsafe {
      ONCE_LOCK.get_or_init(|| {
        super::transit_0realtime__NyctStopTimeUpdate_msg_init.0 =
            ::protobuf::__internal::runtime::build_mini_table("$11");
        ::protobuf::__internal::runtime::link_mini_table(
            super::transit_0realtime__NyctStopTimeUpdate_msg_init.0, &[], &[]);
        ::protobuf::__internal::runtime::MiniTableInitPtr(super::transit_0realtime__NyctStopTimeUpdate_msg_init.0)
      }).0
    }
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NyctStopTimeUpdate {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NyctStopTimeUpdate {
  type Msg = NyctStopTimeUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctStopTimeUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctStopTimeUpdate {
  type Msg = NyctStopTimeUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctStopTimeUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtrMut for NyctStopTimeUpdateMut<'_> {
  type Msg = NyctStopTimeUpdate;
  fn get_ptr_mut(&mut self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctStopTimeUpdate> {
    self.inner.ptr_mut()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctStopTimeUpdateMut<'_> {
  type Msg = NyctStopTimeUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctStopTimeUpdate> {
    self.inner.ptr()
  }
}
unsafe impl ::protobuf::__internal::runtime::UpbGetMessagePtr for NyctStopTimeUpdateView<'_> {
  type Msg = NyctStopTimeUpdate;
  fn get_ptr(&self, _private: ::protobuf::__internal::Private) -> ::protobuf::__internal::runtime::MessagePtr<NyctStopTimeUpdate> {
    self.inner.ptr()
  }
}

unsafe impl ::protobuf::__internal::runtime::UpbGetArena for NyctStopTimeUpdateMut<'_> {
  fn get_arena(&mut self, _private: ::protobuf::__internal::Private) -> &::protobuf::__internal::runtime::Arena {
    self.inner.arena()
  }
}






