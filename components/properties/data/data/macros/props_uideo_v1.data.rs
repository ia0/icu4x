// @generated
/// Implement `DataProvider<UnifiedIdeographV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_uideo_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_UIDEO_V1: &'static <icu::properties::provider::UnifiedIdeographV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x004\0\0\xC0M\0\0\0N\0\0\0\xA0\0\0\x0E\xFA\0\0\x10\xFA\0\0\x11\xFA\0\0\x12\xFA\0\0\x13\xFA\0\0\x15\xFA\0\0\x1F\xFA\0\0 \xFA\0\0!\xFA\0\0\"\xFA\0\0#\xFA\0\0%\xFA\0\0'\xFA\0\0*\xFA\0\0\0\0\x02\0\xE0\xA6\x02\0\0\xA7\x02\0:\xB7\x02\0@\xB7\x02\0\x1E\xB8\x02\0 \xB8\x02\0\xA2\xCE\x02\0\xB0\xCE\x02\0\xE1\xEB\x02\0\0\0\x03\0K\x13\x03\0P\x13\x03\0\xB0#\x03\0") }, 97058u32)
            });
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::UnifiedIdeographV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::UnifiedIdeographV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_UIDEO_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::UnifiedIdeographV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
