// @generated
/// Implement `DataProvider<LineBreakValueToLongNameV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_long_linear_lb_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_LONG_LINEAR_LB_V1: &'static <icu::properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"+\0\0\0\0\0\x07\0\x10\0\x1A\0$\0/\0;\0J\0Z\0k\0y\0\x88\0\x93\0\x97\0\x9D\0\xA8\0\xB3\0\xC0\0\xC9\0\xD3\0\xDA\0\xEA\0\xF9\0\x07\x01\x10\x01\x1F\x01(\x01-\x01:\x01A\x01J\x01U\x01W\x01Y\x01[\x01]\x01_\x01p\x01\x8C\x01\x99\x01\xAB\x01\xB1\x01\xBB\x01UnknownAmbiguousAlphabeticBreak_BothBreak_AfterBreak_BeforeMandatory_BreakContingent_BreakClose_PunctuationCombining_MarkCarriage_ReturnExclamationGlueHyphenIdeographicInseparableInfix_NumericLine_FeedNonstarterNumericOpen_PunctuationPostfix_NumericPrefix_NumericQuotationComplex_ContextSurrogateSpaceBreak_SymbolsZWSpaceNext_LineWord_JoinerH2H3JLJTJVClose_ParenthesisConditional_Japanese_StarterHebrew_LetterRegional_IndicatorE_BaseE_ModifierZWJ") } };
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::LineBreakValueToLongNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::LineBreakValueToLongNameV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_TO_LONG_LINEAR_LB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::LineBreakValueToLongNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
