// @generated
/// Implement `DataProvider<IdeographicV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_ideo_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_IDEO_V1: &'static <icu::properties::provider::IdeographicV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\x060\0\0\x080\0\0!0\0\0*0\0\080\0\0;0\0\0\x004\0\0\xC0M\0\0\0N\0\0\0\xA0\0\0\0\xF9\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\xE4o\x01\0\xE5o\x01\0\0p\x01\0\xF8\x87\x01\0\0\x88\x01\0\xD6\x8C\x01\0\0\x8D\x01\0\t\x8D\x01\0p\xB1\x01\0\xFC\xB2\x01\0\0\0\x02\0\xE0\xA6\x02\0\0\xA7\x02\0:\xB7\x02\0@\xB7\x02\0\x1E\xB8\x02\0 \xB8\x02\0\xA2\xCE\x02\0\xB0\xCE\x02\0\xE1\xEB\x02\0\0\xF8\x02\0\x1E\xFA\x02\0\0\0\x03\0K\x13\x03\0P\x13\x03\0\xB0#\x03\0") }, 105854u32)
            });
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::IdeographicV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::IdeographicV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_IDEO_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::IdeographicV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
