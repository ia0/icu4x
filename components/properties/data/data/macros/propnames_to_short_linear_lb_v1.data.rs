// @generated
/// Implement `DataProvider<LineBreakValueToShortNameV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_linear_lb_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_SHORT_LINEAR_LB_V1: &'static <icu::properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"+\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x14\0\x16\0\x18\0\x1A\0\x1C\0\x1E\0 \0\"\0$\0&\0(\0*\0,\0.\x000\x002\x004\x006\08\0:\0<\0>\0@\0B\0D\0F\0H\0J\0L\0N\0P\0R\0T\0XXAIALB2BABBBKCBCLCMCREXGLHYIDINISLFNSNUOPPOPRQUSASGSPSYZWNLWJH2H3JLJTJVCPCJHLRIEBEMZWJ") } };
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::LineBreakValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakValueToShortNameV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_TO_SHORT_LINEAR_LB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::LineBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
