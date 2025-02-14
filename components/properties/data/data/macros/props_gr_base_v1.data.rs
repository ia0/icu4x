// @generated
/// Implement `DataProvider<GraphemeBaseV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_gr_base_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_GR_BASE_V1: &'static <icu::properties::provider::GraphemeBaseV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b" \0\0\0\x7F\0\0\0\xA0\0\0\0\xAD\0\0\0\xAE\0\0\0\0\x03\0\0p\x03\0\0x\x03\0\0z\x03\0\0\x80\x03\0\0\x84\x03\0\0\x8B\x03\0\0\x8C\x03\0\0\x8D\x03\0\0\x8E\x03\0\0\xA2\x03\0\0\xA3\x03\0\0\x83\x04\0\0\x8A\x04\0\x000\x05\0\x001\x05\0\0W\x05\0\0Y\x05\0\0\x8B\x05\0\0\x8D\x05\0\0\x90\x05\0\0\xBE\x05\0\0\xBF\x05\0\0\xC0\x05\0\0\xC1\x05\0\0\xC3\x05\0\0\xC4\x05\0\0\xC6\x05\0\0\xC7\x05\0\0\xD0\x05\0\0\xEB\x05\0\0\xEF\x05\0\0\xF5\x05\0\0\x06\x06\0\0\x10\x06\0\0\x1B\x06\0\0\x1C\x06\0\0\x1D\x06\0\0K\x06\0\0`\x06\0\0p\x06\0\0q\x06\0\0\xD6\x06\0\0\xDE\x06\0\0\xDF\x06\0\0\xE5\x06\0\0\xE7\x06\0\0\xE9\x06\0\0\xEA\x06\0\0\xEE\x06\0\0\x0E\x07\0\0\x10\x07\0\0\x11\x07\0\0\x12\x07\0\x000\x07\0\0M\x07\0\0\xA6\x07\0\0\xB1\x07\0\0\xB2\x07\0\0\xC0\x07\0\0\xEB\x07\0\0\xF4\x07\0\0\xFB\x07\0\0\xFE\x07\0\0\x16\x08\0\0\x1A\x08\0\0\x1B\x08\0\0$\x08\0\0%\x08\0\0(\x08\0\0)\x08\0\x000\x08\0\0?\x08\0\0@\x08\0\0Y\x08\0\0^\x08\0\0_\x08\0\0`\x08\0\0k\x08\0\0p\x08\0\0\x8F\x08\0\0\xA0\x08\0\0\xCA\x08\0\0\x03\t\0\0:\t\0\0;\t\0\0<\t\0\0=\t\0\0A\t\0\0I\t\0\0M\t\0\0N\t\0\0Q\t\0\0X\t\0\0b\t\0\0d\t\0\0\x81\t\0\0\x82\t\0\0\x84\t\0\0\x85\t\0\0\x8D\t\0\0\x8F\t\0\0\x91\t\0\0\x93\t\0\0\xA9\t\0\0\xAA\t\0\0\xB1\t\0\0\xB2\t\0\0\xB3\t\0\0\xB6\t\0\0\xBA\t\0\0\xBD\t\0\0\xBE\t\0\0\xBF\t\0\0\xC1\t\0\0\xC7\t\0\0\xC9\t\0\0\xCB\t\0\0\xCD\t\0\0\xCE\t\0\0\xCF\t\0\0\xDC\t\0\0\xDE\t\0\0\xDF\t\0\0\xE2\t\0\0\xE6\t\0\0\xFE\t\0\0\x03\n\0\0\x04\n\0\0\x05\n\0\0\x0B\n\0\0\x0F\n\0\0\x11\n\0\0\x13\n\0\0)\n\0\0*\n\0\x001\n\0\x002\n\0\x004\n\0\x005\n\0\x007\n\0\08\n\0\0:\n\0\0>\n\0\0A\n\0\0Y\n\0\0]\n\0\0^\n\0\0_\n\0\0f\n\0\0p\n\0\0r\n\0\0u\n\0\0v\n\0\0w\n\0\0\x83\n\0\0\x84\n\0\0\x85\n\0\0\x8E\n\0\0\x8F\n\0\0\x92\n\0\0\x93\n\0\0\xA9\n\0\0\xAA\n\0\0\xB1\n\0\0\xB2\n\0\0\xB4\n\0\0\xB5\n\0\0\xBA\n\0\0\xBD\n\0\0\xC1\n\0\0\xC9\n\0\0\xCA\n\0\0\xCB\n\0\0\xCD\n\0\0\xD0\n\0\0\xD1\n\0\0\xE0\n\0\0\xE2\n\0\0\xE6\n\0\0\xF2\n\0\0\xF9\n\0\0\xFA\n\0\0\x02\x0B\0\0\x04\x0B\0\0\x05\x0B\0\0\r\x0B\0\0\x0F\x0B\0\0\x11\x0B\0\0\x13\x0B\0\0)\x0B\0\0*\x0B\0\x001\x0B\0\x002\x0B\0\x004\x0B\0\x005\x0B\0\0:\x0B\0\0=\x0B\0\0>\x0B\0\0@\x0B\0\0A\x0B\0\0G\x0B\0\0I\x0B\0\0K\x0B\0\0M\x0B\0\0\\\x0B\0\0^\x0B\0\0_\x0B\0\0b\x0B\0\0f\x0B\0\0x\x0B\0\0\x83\x0B\0\0\x84\x0B\0\0\x85\x0B\0\0\x8B\x0B\0\0\x8E\x0B\0\0\x91\x0B\0\0\x92\x0B\0\0\x96\x0B\0\0\x99\x0B\0\0\x9B\x0B\0\0\x9C\x0B\0\0\x9D\x0B\0\0\x9E\x0B\0\0\xA0\x0B\0\0\xA3\x0B\0\0\xA5\x0B\0\0\xA8\x0B\0\0\xAB\x0B\0\0\xAE\x0B\0\0\xBA\x0B\0\0\xBF\x0B\0\0\xC0\x0B\0\0\xC1\x0B\0\0\xC3\x0B\0\0\xC6\x0B\0\0\xC9\x0B\0\0\xCA\x0B\0\0\xCD\x0B\0\0\xD0\x0B\0\0\xD1\x0B\0\0\xE6\x0B\0\0\xFB\x0B\0\0\x01\x0C\0\0\x04\x0C\0\0\x05\x0C\0\0\r\x0C\0\0\x0E\x0C\0\0\x11\x0C\0\0\x12\x0C\0\0)\x0C\0\0*\x0C\0\0:\x0C\0\0=\x0C\0\0>\x0C\0\0A\x0C\0\0E\x0C\0\0X\x0C\0\0[\x0C\0\0]\x0C\0\0^\x0C\0\0`\x0C\0\0b\x0C\0\0f\x0C\0\0p\x0C\0\0w\x0C\0\0\x81\x0C\0\0\x82\x0C\0\0\x8D\x0C\0\0\x8E\x0C\0\0\x91\x0C\0\0\x92\x0C\0\0\xA9\x0C\0\0\xAA\x0C\0\0\xB4\x0C\0\0\xB5\x0C\0\0\xBA\x0C\0\0\xBD\x0C\0\0\xBF\x0C\0\0\xC0\x0C\0\0\xC2\x0C\0\0\xC3\x0C\0\0\xC5\x0C\0\0\xC7\x0C\0\0\xC9\x0C\0\0\xCA\x0C\0\0\xCC\x0C\0\0\xDD\x0C\0\0\xDF\x0C\0\0\xE0\x0C\0\0\xE2\x0C\0\0\xE6\x0C\0\0\xF0\x0C\0\0\xF1\x0C\0\0\xF4\x0C\0\0\x02\r\0\0\r\r\0\0\x0E\r\0\0\x11\r\0\0\x12\r\0\0;\r\0\0=\r\0\0>\r\0\0?\r\0\0A\r\0\0F\r\0\0I\r\0\0J\r\0\0M\r\0\0N\r\0\0P\r\0\0T\r\0\0W\r\0\0X\r\0\0b\r\0\0f\r\0\0\x80\r\0\0\x82\r\0\0\x84\r\0\0\x85\r\0\0\x97\r\0\0\x9A\r\0\0\xB2\r\0\0\xB3\r\0\0\xBC\r\0\0\xBD\r\0\0\xBE\r\0\0\xC0\r\0\0\xC7\r\0\0\xD0\r\0\0\xD2\r\0\0\xD8\r\0\0\xDF\r\0\0\xE6\r\0\0\xF0\r\0\0\xF2\r\0\0\xF5\r\0\0\x01\x0E\0\x001\x0E\0\x002\x0E\0\x004\x0E\0\0?\x0E\0\0G\x0E\0\0O\x0E\0\0\\\x0E\0\0\x81\x0E\0\0\x83\x0E\0\0\x84\x0E\0\0\x85\x0E\0\0\x86\x0E\0\0\x8B\x0E\0\0\x8C\x0E\0\0\xA4\x0E\0\0\xA5\x0E\0\0\xA6\x0E\0\0\xA7\x0E\0\0\xB1\x0E\0\0\xB2\x0E\0\0\xB4\x0E\0\0\xBD\x0E\0\0\xBE\x0E\0\0\xC0\x0E\0\0\xC5\x0E\0\0\xC6\x0E\0\0\xC7\x0E\0\0\xD0\x0E\0\0\xDA\x0E\0\0\xDC\x0E\0\0\xE0\x0E\0\0\0\x0F\0\0\x18\x0F\0\0\x1A\x0F\0\x005\x0F\0\x006\x0F\0\x007\x0F\0\08\x0F\0\09\x0F\0\0:\x0F\0\0H\x0F\0\0I\x0F\0\0m\x0F\0\0\x7F\x0F\0\0\x80\x0F\0\0\x85\x0F\0\0\x86\x0F\0\0\x88\x0F\0\0\x8D\x0F\0\0\xBE\x0F\0\0\xC6\x0F\0\0\xC7\x0F\0\0\xCD\x0F\0\0\xCE\x0F\0\0\xDB\x0F\0\0\0\x10\0\0-\x10\0\x001\x10\0\x002\x10\0\08\x10\0\09\x10\0\0;\x10\0\0=\x10\0\0?\x10\0\0X\x10\0\0Z\x10\0\0^\x10\0\0a\x10\0\0q\x10\0\0u\x10\0\0\x82\x10\0\0\x83\x10\0\0\x85\x10\0\0\x87\x10\0\0\x8D\x10\0\0\x8E\x10\0\0\x9D\x10\0\0\x9E\x10\0\0\xC6\x10\0\0\xC7\x10\0\0\xC8\x10\0\0\xCD\x10\0\0\xCE\x10\0\0\xD0\x10\0\0I\x12\0\0J\x12\0\0N\x12\0\0P\x12\0\0W\x12\0\0X\x12\0\0Y\x12\0\0Z\x12\0\0^\x12\0\0`\x12\0\0\x89\x12\0\0\x8A\x12\0\0\x8E\x12\0\0\x90\x12\0\0\xB1\x12\0\0\xB2\x12\0\0\xB6\x12\0\0\xB8\x12\0\0\xBF\x12\0\0\xC0\x12\0\0\xC1\x12\0\0\xC2\x12\0\0\xC6\x12\0\0\xC8\x12\0\0\xD7\x12\0\0\xD8\x12\0\0\x11\x13\0\0\x12\x13\0\0\x16\x13\0\0\x18\x13\0\0[\x13\0\0`\x13\0\0}\x13\0\0\x80\x13\0\0\x9A\x13\0\0\xA0\x13\0\0\xF6\x13\0\0\xF8\x13\0\0\xFE\x13\0\0\0\x14\0\0\x9D\x16\0\0\xA0\x16\0\0\xF9\x16\0\0\0\x17\0\0\x12\x17\0\0\x15\x17\0\0\x16\x17\0\0\x1F\x17\0\x002\x17\0\x004\x17\0\x007\x17\0\0@\x17\0\0R\x17\0\0`\x17\0\0m\x17\0\0n\x17\0\0q\x17\0\0\x80\x17\0\0\xB4\x17\0\0\xB6\x17\0\0\xB7\x17\0\0\xBE\x17\0\0\xC6\x17\0\0\xC7\x17\0\0\xC9\x17\0\0\xD4\x17\0\0\xDD\x17\0\0\xE0\x17\0\0\xEA\x17\0\0\xF0\x17\0\0\xFA\x17\0\0\0\x18\0\0\x0B\x18\0\0\x10\x18\0\0\x1A\x18\0\0 \x18\0\0y\x18\0\0\x80\x18\0\0\x85\x18\0\0\x87\x18\0\0\xA9\x18\0\0\xAA\x18\0\0\xAB\x18\0\0\xB0\x18\0\0\xF6\x18\0\0\0\x19\0\0\x1F\x19\0\0#\x19\0\0'\x19\0\0)\x19\0\0,\x19\0\x000\x19\0\x002\x19\0\x003\x19\0\09\x19\0\0@\x19\0\0A\x19\0\0D\x19\0\0n\x19\0\0p\x19\0\0u\x19\0\0\x80\x19\0\0\xAC\x19\0\0\xB0\x19\0\0\xCA\x19\0\0\xD0\x19\0\0\xDB\x19\0\0\xDE\x19\0\0\x17\x1A\0\0\x19\x1A\0\0\x1B\x1A\0\0\x1E\x1A\0\0V\x1A\0\0W\x1A\0\0X\x1A\0\0a\x1A\0\0b\x1A\0\0c\x1A\0\0e\x1A\0\0m\x1A\0\0s\x1A\0\0\x80\x1A\0\0\x8A\x1A\0\0\x90\x1A\0\0\x9A\x1A\0\0\xA0\x1A\0\0\xAE\x1A\0\0\x04\x1B\0\x004\x1B\0\0;\x1B\0\0<\x1B\0\0=\x1B\0\0B\x1B\0\0C\x1B\0\0M\x1B\0\0P\x1B\0\0k\x1B\0\0t\x1B\0\0\x7F\x1B\0\0\x82\x1B\0\0\xA2\x1B\0\0\xA6\x1B\0\0\xA8\x1B\0\0\xAA\x1B\0\0\xAB\x1B\0\0\xAE\x1B\0\0\xE6\x1B\0\0\xE7\x1B\0\0\xE8\x1B\0\0\xEA\x1B\0\0\xED\x1B\0\0\xEE\x1B\0\0\xEF\x1B\0\0\xF2\x1B\0\0\xF4\x1B\0\0\xFC\x1B\0\0,\x1C\0\x004\x1C\0\x006\x1C\0\0;\x1C\0\0J\x1C\0\0M\x1C\0\0\x89\x1C\0\0\x90\x1C\0\0\xBB\x1C\0\0\xBD\x1C\0\0\xC8\x1C\0\0\xD3\x1C\0\0\xD4\x1C\0\0\xE1\x1C\0\0\xE2\x1C\0\0\xE9\x1C\0\0\xED\x1C\0\0\xEE\x1C\0\0\xF4\x1C\0\0\xF5\x1C\0\0\xF8\x1C\0\0\xFA\x1C\0\0\xFB\x1C\0\0\0\x1D\0\0\xC0\x1D\0\0\0\x1E\0\0\x16\x1F\0\0\x18\x1F\0\0\x1E\x1F\0\0 \x1F\0\0F\x1F\0\0H\x1F\0\0N\x1F\0\0P\x1F\0\0X\x1F\0\0Y\x1F\0\0Z\x1F\0\0[\x1F\0\0\\\x1F\0\0]\x1F\0\0^\x1F\0\0_\x1F\0\0~\x1F\0\0\x80\x1F\0\0\xB5\x1F\0\0\xB6\x1F\0\0\xC5\x1F\0\0\xC6\x1F\0\0\xD4\x1F\0\0\xD6\x1F\0\0\xDC\x1F\0\0\xDD\x1F\0\0\xF0\x1F\0\0\xF2\x1F\0\0\xF5\x1F\0\0\xF6\x1F\0\0\xFF\x1F\0\0\0 \0\0\x0B \0\0\x10 \0\0( \0\0/ \0\0` \0\0p \0\0r \0\0t \0\0\x8F \0\0\x90 \0\0\x9D \0\0\xA0 \0\0\xC1 \0\0\0!\0\0\x8C!\0\0\x90!\0\0'$\0\0@$\0\0K$\0\0`$\0\0t+\0\0v+\0\0\x96+\0\0\x97+\0\0\xEF,\0\0\xF2,\0\0\xF4,\0\0\xF9,\0\0&-\0\0'-\0\0(-\0\0--\0\0.-\0\x000-\0\0h-\0\0o-\0\0q-\0\0\x80-\0\0\x97-\0\0\xA0-\0\0\xA7-\0\0\xA8-\0\0\xAF-\0\0\xB0-\0\0\xB7-\0\0\xB8-\0\0\xBF-\0\0\xC0-\0\0\xC7-\0\0\xC8-\0\0\xCF-\0\0\xD0-\0\0\xD7-\0\0\xD8-\0\0\xDF-\0\0\0.\0\0^.\0\0\x80.\0\0\x9A.\0\0\x9B.\0\0\xF4.\0\0\0/\0\0\xD6/\0\0\xF0/\0\0\xFC/\0\0\x000\0\0*0\0\x0000\0\0@0\0\0A0\0\0\x970\0\0\x9B0\0\0\x001\0\0\x051\0\x0001\0\x0011\0\0\x8F1\0\0\x901\0\0\xE41\0\0\xF01\0\0\x1F2\0\0 2\0\0\x8D\xA4\0\0\x90\xA4\0\0\xC7\xA4\0\0\xD0\xA4\0\0,\xA6\0\0@\xA6\0\0o\xA6\0\0s\xA6\0\0t\xA6\0\0~\xA6\0\0\x9E\xA6\0\0\xA0\xA6\0\0\xF0\xA6\0\0\xF2\xA6\0\0\xF8\xA6\0\0\0\xA7\0\0\xCB\xA7\0\0\xD0\xA7\0\0\xD2\xA7\0\0\xD3\xA7\0\0\xD4\xA7\0\0\xD5\xA7\0\0\xDA\xA7\0\0\xF2\xA7\0\0\x02\xA8\0\0\x03\xA8\0\0\x06\xA8\0\0\x07\xA8\0\0\x0B\xA8\0\0\x0C\xA8\0\0%\xA8\0\0'\xA8\0\0,\xA8\0\x000\xA8\0\0:\xA8\0\0@\xA8\0\0x\xA8\0\0\x80\xA8\0\0\xC4\xA8\0\0\xCE\xA8\0\0\xDA\xA8\0\0\xF2\xA8\0\0\xFF\xA8\0\0\0\xA9\0\0&\xA9\0\0.\xA9\0\0G\xA9\0\0R\xA9\0\0T\xA9\0\0_\xA9\0\0}\xA9\0\0\x83\xA9\0\0\xB3\xA9\0\0\xB4\xA9\0\0\xB6\xA9\0\0\xBA\xA9\0\0\xBC\xA9\0\0\xBE\xA9\0\0\xCE\xA9\0\0\xCF\xA9\0\0\xDA\xA9\0\0\xDE\xA9\0\0\xE5\xA9\0\0\xE6\xA9\0\0\xFF\xA9\0\0\0\xAA\0\0)\xAA\0\0/\xAA\0\x001\xAA\0\x003\xAA\0\x005\xAA\0\0@\xAA\0\0C\xAA\0\0D\xAA\0\0L\xAA\0\0M\xAA\0\0N\xAA\0\0P\xAA\0\0Z\xAA\0\0\\\xAA\0\0|\xAA\0\0}\xAA\0\0\xB0\xAA\0\0\xB1\xAA\0\0\xB2\xAA\0\0\xB5\xAA\0\0\xB7\xAA\0\0\xB9\xAA\0\0\xBE\xAA\0\0\xC0\xAA\0\0\xC1\xAA\0\0\xC2\xAA\0\0\xC3\xAA\0\0\xDB\xAA\0\0\xEC\xAA\0\0\xEE\xAA\0\0\xF6\xAA\0\0\x01\xAB\0\0\x07\xAB\0\0\t\xAB\0\0\x0F\xAB\0\0\x11\xAB\0\0\x17\xAB\0\0 \xAB\0\0'\xAB\0\0(\xAB\0\0/\xAB\0\x000\xAB\0\0l\xAB\0\0p\xAB\0\0\xE5\xAB\0\0\xE6\xAB\0\0\xE8\xAB\0\0\xE9\xAB\0\0\xED\xAB\0\0\xF0\xAB\0\0\xFA\xAB\0\0\0\xAC\0\0\xA4\xD7\0\0\xB0\xD7\0\0\xC7\xD7\0\0\xCB\xD7\0\0\xFC\xD7\0\0\0\xF9\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\0\xFB\0\0\x07\xFB\0\0\x13\xFB\0\0\x18\xFB\0\0\x1D\xFB\0\0\x1E\xFB\0\0\x1F\xFB\0\x007\xFB\0\08\xFB\0\0=\xFB\0\0>\xFB\0\0?\xFB\0\0@\xFB\0\0B\xFB\0\0C\xFB\0\0E\xFB\0\0F\xFB\0\0\xC3\xFB\0\0\xD3\xFB\0\0\x90\xFD\0\0\x92\xFD\0\0\xC8\xFD\0\0\xCF\xFD\0\0\xD0\xFD\0\0\xF0\xFD\0\0\0\xFE\0\0\x10\xFE\0\0\x1A\xFE\0\x000\xFE\0\0S\xFE\0\0T\xFE\0\0g\xFE\0\0h\xFE\0\0l\xFE\0\0p\xFE\0\0u\xFE\0\0v\xFE\0\0\xFD\xFE\0\0\x01\xFF\0\0\x9E\xFF\0\0\xA0\xFF\0\0\xBF\xFF\0\0\xC2\xFF\0\0\xC8\xFF\0\0\xCA\xFF\0\0\xD0\xFF\0\0\xD2\xFF\0\0\xD8\xFF\0\0\xDA\xFF\0\0\xDD\xFF\0\0\xE0\xFF\0\0\xE7\xFF\0\0\xE8\xFF\0\0\xEF\xFF\0\0\xFC\xFF\0\0\xFE\xFF\0\0\0\0\x01\0\x0C\0\x01\0\r\0\x01\0'\0\x01\0(\0\x01\0;\0\x01\0<\0\x01\0>\0\x01\0?\0\x01\0N\0\x01\0P\0\x01\0^\0\x01\0\x80\0\x01\0\xFB\0\x01\0\0\x01\x01\0\x03\x01\x01\0\x07\x01\x01\x004\x01\x01\x007\x01\x01\0\x8F\x01\x01\0\x90\x01\x01\0\x9D\x01\x01\0\xA0\x01\x01\0\xA1\x01\x01\0\xD0\x01\x01\0\xFD\x01\x01\0\x80\x02\x01\0\x9D\x02\x01\0\xA0\x02\x01\0\xD1\x02\x01\0\xE1\x02\x01\0\xFC\x02\x01\0\0\x03\x01\0$\x03\x01\0-\x03\x01\0K\x03\x01\0P\x03\x01\0v\x03\x01\0\x80\x03\x01\0\x9E\x03\x01\0\x9F\x03\x01\0\xC4\x03\x01\0\xC8\x03\x01\0\xD6\x03\x01\0\0\x04\x01\0\x9E\x04\x01\0\xA0\x04\x01\0\xAA\x04\x01\0\xB0\x04\x01\0\xD4\x04\x01\0\xD8\x04\x01\0\xFC\x04\x01\0\0\x05\x01\0(\x05\x01\x000\x05\x01\0d\x05\x01\0o\x05\x01\0{\x05\x01\0|\x05\x01\0\x8B\x05\x01\0\x8C\x05\x01\0\x93\x05\x01\0\x94\x05\x01\0\x96\x05\x01\0\x97\x05\x01\0\xA2\x05\x01\0\xA3\x05\x01\0\xB2\x05\x01\0\xB3\x05\x01\0\xBA\x05\x01\0\xBB\x05\x01\0\xBD\x05\x01\0\0\x06\x01\x007\x07\x01\0@\x07\x01\0V\x07\x01\0`\x07\x01\0h\x07\x01\0\x80\x07\x01\0\x86\x07\x01\0\x87\x07\x01\0\xB1\x07\x01\0\xB2\x07\x01\0\xBB\x07\x01\0\0\x08\x01\0\x06\x08\x01\0\x08\x08\x01\0\t\x08\x01\0\n\x08\x01\x006\x08\x01\x007\x08\x01\09\x08\x01\0<\x08\x01\0=\x08\x01\0?\x08\x01\0V\x08\x01\0W\x08\x01\0\x9F\x08\x01\0\xA7\x08\x01\0\xB0\x08\x01\0\xE0\x08\x01\0\xF3\x08\x01\0\xF4\x08\x01\0\xF6\x08\x01\0\xFB\x08\x01\0\x1C\t\x01\0\x1F\t\x01\0:\t\x01\0?\t\x01\0@\t\x01\0\x80\t\x01\0\xB8\t\x01\0\xBC\t\x01\0\xD0\t\x01\0\xD2\t\x01\0\x01\n\x01\0\x10\n\x01\0\x14\n\x01\0\x15\n\x01\0\x18\n\x01\0\x19\n\x01\x006\n\x01\0@\n\x01\0I\n\x01\0P\n\x01\0Y\n\x01\0`\n\x01\0\xA0\n\x01\0\xC0\n\x01\0\xE5\n\x01\0\xEB\n\x01\0\xF7\n\x01\0\0\x0B\x01\x006\x0B\x01\09\x0B\x01\0V\x0B\x01\0X\x0B\x01\0s\x0B\x01\0x\x0B\x01\0\x92\x0B\x01\0\x99\x0B\x01\0\x9D\x0B\x01\0\xA9\x0B\x01\0\xB0\x0B\x01\0\0\x0C\x01\0I\x0C\x01\0\x80\x0C\x01\0\xB3\x0C\x01\0\xC0\x0C\x01\0\xF3\x0C\x01\0\xFA\x0C\x01\0$\r\x01\x000\r\x01\0:\r\x01\0`\x0E\x01\0\x7F\x0E\x01\0\x80\x0E\x01\0\xAA\x0E\x01\0\xAD\x0E\x01\0\xAE\x0E\x01\0\xB0\x0E\x01\0\xB2\x0E\x01\0\0\x0F\x01\0(\x0F\x01\x000\x0F\x01\0F\x0F\x01\0Q\x0F\x01\0Z\x0F\x01\0p\x0F\x01\0\x82\x0F\x01\0\x86\x0F\x01\0\x8A\x0F\x01\0\xB0\x0F\x01\0\xCC\x0F\x01\0\xE0\x0F\x01\0\xF7\x0F\x01\0\0\x10\x01\0\x01\x10\x01\0\x02\x10\x01\08\x10\x01\0G\x10\x01\0N\x10\x01\0R\x10\x01\0p\x10\x01\0q\x10\x01\0s\x10\x01\0u\x10\x01\0v\x10\x01\0\x82\x10\x01\0\xB3\x10\x01\0\xB7\x10\x01\0\xB9\x10\x01\0\xBB\x10\x01\0\xBD\x10\x01\0\xBE\x10\x01\0\xC2\x10\x01\0\xD0\x10\x01\0\xE9\x10\x01\0\xF0\x10\x01\0\xFA\x10\x01\0\x03\x11\x01\0'\x11\x01\0,\x11\x01\0-\x11\x01\x006\x11\x01\0H\x11\x01\0P\x11\x01\0s\x11\x01\0t\x11\x01\0w\x11\x01\0\x82\x11\x01\0\xB6\x11\x01\0\xBF\x11\x01\0\xC9\x11\x01\0\xCD\x11\x01\0\xCF\x11\x01\0\xD0\x11\x01\0\xE0\x11\x01\0\xE1\x11\x01\0\xF5\x11\x01\0\0\x12\x01\0\x12\x12\x01\0\x13\x12\x01\0/\x12\x01\x002\x12\x01\x004\x12\x01\x005\x12\x01\x006\x12\x01\08\x12\x01\0>\x12\x01\0?\x12\x01\0A\x12\x01\0\x80\x12\x01\0\x87\x12\x01\0\x88\x12\x01\0\x89\x12\x01\0\x8A\x12\x01\0\x8E\x12\x01\0\x8F\x12\x01\0\x9E\x12\x01\0\x9F\x12\x01\0\xAA\x12\x01\0\xB0\x12\x01\0\xDF\x12\x01\0\xE0\x12\x01\0\xE3\x12\x01\0\xF0\x12\x01\0\xFA\x12\x01\0\x02\x13\x01\0\x04\x13\x01\0\x05\x13\x01\0\r\x13\x01\0\x0F\x13\x01\0\x11\x13\x01\0\x13\x13\x01\0)\x13\x01\0*\x13\x01\x001\x13\x01\x002\x13\x01\x004\x13\x01\x005\x13\x01\0:\x13\x01\0=\x13\x01\0>\x13\x01\0?\x13\x01\0@\x13\x01\0A\x13\x01\0E\x13\x01\0G\x13\x01\0I\x13\x01\0K\x13\x01\0N\x13\x01\0P\x13\x01\0Q\x13\x01\0]\x13\x01\0d\x13\x01\0\0\x14\x01\08\x14\x01\0@\x14\x01\0B\x14\x01\0E\x14\x01\0F\x14\x01\0G\x14\x01\0\\\x14\x01\0]\x14\x01\0^\x14\x01\0_\x14\x01\0b\x14\x01\0\x80\x14\x01\0\xB0\x14\x01\0\xB1\x14\x01\0\xB3\x14\x01\0\xB9\x14\x01\0\xBA\x14\x01\0\xBB\x14\x01\0\xBD\x14\x01\0\xBE\x14\x01\0\xBF\x14\x01\0\xC1\x14\x01\0\xC2\x14\x01\0\xC4\x14\x01\0\xC8\x14\x01\0\xD0\x14\x01\0\xDA\x14\x01\0\x80\x15\x01\0\xAF\x15\x01\0\xB0\x15\x01\0\xB2\x15\x01\0\xB8\x15\x01\0\xBC\x15\x01\0\xBE\x15\x01\0\xBF\x15\x01\0\xC1\x15\x01\0\xDC\x15\x01\0\0\x16\x01\x003\x16\x01\0;\x16\x01\0=\x16\x01\0>\x16\x01\0?\x16\x01\0A\x16\x01\0E\x16\x01\0P\x16\x01\0Z\x16\x01\0`\x16\x01\0m\x16\x01\0\x80\x16\x01\0\xAB\x16\x01\0\xAC\x16\x01\0\xAD\x16\x01\0\xAE\x16\x01\0\xB0\x16\x01\0\xB6\x16\x01\0\xB7\x16\x01\0\xB8\x16\x01\0\xBA\x16\x01\0\xC0\x16\x01\0\xCA\x16\x01\0\0\x17\x01\0\x1B\x17\x01\0 \x17\x01\0\"\x17\x01\0&\x17\x01\0'\x17\x01\x000\x17\x01\0G\x17\x01\0\0\x18\x01\0/\x18\x01\08\x18\x01\09\x18\x01\0;\x18\x01\0<\x18\x01\0\xA0\x18\x01\0\xF3\x18\x01\0\xFF\x18\x01\0\x07\x19\x01\0\t\x19\x01\0\n\x19\x01\0\x0C\x19\x01\0\x14\x19\x01\0\x15\x19\x01\0\x17\x19\x01\0\x18\x19\x01\x000\x19\x01\x001\x19\x01\x006\x19\x01\x007\x19\x01\09\x19\x01\0=\x19\x01\0>\x19\x01\0?\x19\x01\0C\x19\x01\0D\x19\x01\0G\x19\x01\0P\x19\x01\0Z\x19\x01\0\xA0\x19\x01\0\xA8\x19\x01\0\xAA\x19\x01\0\xD4\x19\x01\0\xDC\x19\x01\0\xE0\x19\x01\0\xE1\x19\x01\0\xE5\x19\x01\0\0\x1A\x01\0\x01\x1A\x01\0\x0B\x1A\x01\x003\x1A\x01\09\x1A\x01\0;\x1A\x01\0?\x1A\x01\0G\x1A\x01\0P\x1A\x01\0Q\x1A\x01\0W\x1A\x01\0Y\x1A\x01\0\\\x1A\x01\0\x8A\x1A\x01\0\x97\x1A\x01\0\x98\x1A\x01\0\x9A\x1A\x01\0\xA3\x1A\x01\0\xB0\x1A\x01\0\xF9\x1A\x01\0\0\x1B\x01\0\n\x1B\x01\0\0\x1C\x01\0\t\x1C\x01\0\n\x1C\x01\x000\x1C\x01\0>\x1C\x01\0?\x1C\x01\0@\x1C\x01\0F\x1C\x01\0P\x1C\x01\0m\x1C\x01\0p\x1C\x01\0\x90\x1C\x01\0\xA9\x1C\x01\0\xAA\x1C\x01\0\xB1\x1C\x01\0\xB2\x1C\x01\0\xB4\x1C\x01\0\xB5\x1C\x01\0\0\x1D\x01\0\x07\x1D\x01\0\x08\x1D\x01\0\n\x1D\x01\0\x0B\x1D\x01\x001\x1D\x01\0F\x1D\x01\0G\x1D\x01\0P\x1D\x01\0Z\x1D\x01\0`\x1D\x01\0f\x1D\x01\0g\x1D\x01\0i\x1D\x01\0j\x1D\x01\0\x8F\x1D\x01\0\x93\x1D\x01\0\x95\x1D\x01\0\x96\x1D\x01\0\x97\x1D\x01\0\x98\x1D\x01\0\x99\x1D\x01\0\xA0\x1D\x01\0\xAA\x1D\x01\0\xE0\x1E\x01\0\xF3\x1E\x01\0\xF5\x1E\x01\0\xF9\x1E\x01\0\x02\x1F\x01\0\x11\x1F\x01\0\x12\x1F\x01\x006\x1F\x01\0>\x1F\x01\0@\x1F\x01\0A\x1F\x01\0B\x1F\x01\0C\x1F\x01\0Z\x1F\x01\0\xB0\x1F\x01\0\xB1\x1F\x01\0\xC0\x1F\x01\0\xF2\x1F\x01\0\xFF\x1F\x01\0\x9A#\x01\0\0$\x01\0o$\x01\0p$\x01\0u$\x01\0\x80$\x01\0D%\x01\0\x90/\x01\0\xF3/\x01\0\x000\x01\x0004\x01\0A4\x01\0G4\x01\0\0D\x01\0GF\x01\0\0h\x01\09j\x01\0@j\x01\0_j\x01\0`j\x01\0jj\x01\0nj\x01\0\xBFj\x01\0\xC0j\x01\0\xCAj\x01\0\xD0j\x01\0\xEEj\x01\0\xF5j\x01\0\xF6j\x01\0\0k\x01\x000k\x01\x007k\x01\0Fk\x01\0Pk\x01\0Zk\x01\0[k\x01\0bk\x01\0ck\x01\0xk\x01\0}k\x01\0\x90k\x01\0@n\x01\0\x9Bn\x01\0\0o\x01\0Ko\x01\0Po\x01\0\x88o\x01\0\x93o\x01\0\xA0o\x01\0\xE0o\x01\0\xE4o\x01\0\xF0o\x01\0\xF2o\x01\0\0p\x01\0\xF8\x87\x01\0\0\x88\x01\0\xD6\x8C\x01\0\0\x8D\x01\0\t\x8D\x01\0\xF0\xAF\x01\0\xF4\xAF\x01\0\xF5\xAF\x01\0\xFC\xAF\x01\0\xFD\xAF\x01\0\xFF\xAF\x01\0\0\xB0\x01\0#\xB1\x01\x002\xB1\x01\x003\xB1\x01\0P\xB1\x01\0S\xB1\x01\0U\xB1\x01\0V\xB1\x01\0d\xB1\x01\0h\xB1\x01\0p\xB1\x01\0\xFC\xB2\x01\0\0\xBC\x01\0k\xBC\x01\0p\xBC\x01\0}\xBC\x01\0\x80\xBC\x01\0\x89\xBC\x01\0\x90\xBC\x01\0\x9A\xBC\x01\0\x9C\xBC\x01\0\x9D\xBC\x01\0\x9F\xBC\x01\0\xA0\xBC\x01\0P\xCF\x01\0\xC4\xCF\x01\0\0\xD0\x01\0\xF6\xD0\x01\0\0\xD1\x01\0'\xD1\x01\0)\xD1\x01\0e\xD1\x01\0f\xD1\x01\0g\xD1\x01\0j\xD1\x01\0n\xD1\x01\0\x83\xD1\x01\0\x85\xD1\x01\0\x8C\xD1\x01\0\xAA\xD1\x01\0\xAE\xD1\x01\0\xEB\xD1\x01\0\0\xD2\x01\0B\xD2\x01\0E\xD2\x01\0F\xD2\x01\0\xC0\xD2\x01\0\xD4\xD2\x01\0\xE0\xD2\x01\0\xF4\xD2\x01\0\0\xD3\x01\0W\xD3\x01\0`\xD3\x01\0y\xD3\x01\0\0\xD4\x01\0U\xD4\x01\0V\xD4\x01\0\x9D\xD4\x01\0\x9E\xD4\x01\0\xA0\xD4\x01\0\xA2\xD4\x01\0\xA3\xD4\x01\0\xA5\xD4\x01\0\xA7\xD4\x01\0\xA9\xD4\x01\0\xAD\xD4\x01\0\xAE\xD4\x01\0\xBA\xD4\x01\0\xBB\xD4\x01\0\xBC\xD4\x01\0\xBD\xD4\x01\0\xC4\xD4\x01\0\xC5\xD4\x01\0\x06\xD5\x01\0\x07\xD5\x01\0\x0B\xD5\x01\0\r\xD5\x01\0\x15\xD5\x01\0\x16\xD5\x01\0\x1D\xD5\x01\0\x1E\xD5\x01\0:\xD5\x01\0;\xD5\x01\0?\xD5\x01\0@\xD5\x01\0E\xD5\x01\0F\xD5\x01\0G\xD5\x01\0J\xD5\x01\0Q\xD5\x01\0R\xD5\x01\0\xA6\xD6\x01\0\xA8\xD6\x01\0\xCC\xD7\x01\0\xCE\xD7\x01\0\0\xDA\x01\x007\xDA\x01\0;\xDA\x01\0m\xDA\x01\0u\xDA\x01\0v\xDA\x01\0\x84\xDA\x01\0\x85\xDA\x01\0\x8C\xDA\x01\0\0\xDF\x01\0\x1F\xDF\x01\0%\xDF\x01\0+\xDF\x01\x000\xE0\x01\0n\xE0\x01\0\0\xE1\x01\0-\xE1\x01\x007\xE1\x01\0>\xE1\x01\0@\xE1\x01\0J\xE1\x01\0N\xE1\x01\0P\xE1\x01\0\x90\xE2\x01\0\xAE\xE2\x01\0\xC0\xE2\x01\0\xEC\xE2\x01\0\xF0\xE2\x01\0\xFA\xE2\x01\0\xFF\xE2\x01\0\0\xE3\x01\0\xD0\xE4\x01\0\xEC\xE4\x01\0\xF0\xE4\x01\0\xFA\xE4\x01\0\xE0\xE7\x01\0\xE7\xE7\x01\0\xE8\xE7\x01\0\xEC\xE7\x01\0\xED\xE7\x01\0\xEF\xE7\x01\0\xF0\xE7\x01\0\xFF\xE7\x01\0\0\xE8\x01\0\xC5\xE8\x01\0\xC7\xE8\x01\0\xD0\xE8\x01\0\0\xE9\x01\0D\xE9\x01\0K\xE9\x01\0L\xE9\x01\0P\xE9\x01\0Z\xE9\x01\0^\xE9\x01\0`\xE9\x01\0q\xEC\x01\0\xB5\xEC\x01\0\x01\xED\x01\0>\xED\x01\0\0\xEE\x01\0\x04\xEE\x01\0\x05\xEE\x01\0 \xEE\x01\0!\xEE\x01\0#\xEE\x01\0$\xEE\x01\0%\xEE\x01\0'\xEE\x01\0(\xEE\x01\0)\xEE\x01\x003\xEE\x01\x004\xEE\x01\08\xEE\x01\09\xEE\x01\0:\xEE\x01\0;\xEE\x01\0<\xEE\x01\0B\xEE\x01\0C\xEE\x01\0G\xEE\x01\0H\xEE\x01\0I\xEE\x01\0J\xEE\x01\0K\xEE\x01\0L\xEE\x01\0M\xEE\x01\0P\xEE\x01\0Q\xEE\x01\0S\xEE\x01\0T\xEE\x01\0U\xEE\x01\0W\xEE\x01\0X\xEE\x01\0Y\xEE\x01\0Z\xEE\x01\0[\xEE\x01\0\\\xEE\x01\0]\xEE\x01\0^\xEE\x01\0_\xEE\x01\0`\xEE\x01\0a\xEE\x01\0c\xEE\x01\0d\xEE\x01\0e\xEE\x01\0g\xEE\x01\0k\xEE\x01\0l\xEE\x01\0s\xEE\x01\0t\xEE\x01\0x\xEE\x01\0y\xEE\x01\0}\xEE\x01\0~\xEE\x01\0\x7F\xEE\x01\0\x80\xEE\x01\0\x8A\xEE\x01\0\x8B\xEE\x01\0\x9C\xEE\x01\0\xA1\xEE\x01\0\xA4\xEE\x01\0\xA5\xEE\x01\0\xAA\xEE\x01\0\xAB\xEE\x01\0\xBC\xEE\x01\0\xF0\xEE\x01\0\xF2\xEE\x01\0\0\xF0\x01\0,\xF0\x01\x000\xF0\x01\0\x94\xF0\x01\0\xA0\xF0\x01\0\xAF\xF0\x01\0\xB1\xF0\x01\0\xC0\xF0\x01\0\xC1\xF0\x01\0\xD0\xF0\x01\0\xD1\xF0\x01\0\xF6\xF0\x01\0\0\xF1\x01\0\xAE\xF1\x01\0\xE6\xF1\x01\0\x03\xF2\x01\0\x10\xF2\x01\0<\xF2\x01\0@\xF2\x01\0I\xF2\x01\0P\xF2\x01\0R\xF2\x01\0`\xF2\x01\0f\xF2\x01\0\0\xF3\x01\0\xD8\xF6\x01\0\xDC\xF6\x01\0\xED\xF6\x01\0\xF0\xF6\x01\0\xFD\xF6\x01\0\0\xF7\x01\0w\xF7\x01\0{\xF7\x01\0\xDA\xF7\x01\0\xE0\xF7\x01\0\xEC\xF7\x01\0\xF0\xF7\x01\0\xF1\xF7\x01\0\0\xF8\x01\0\x0C\xF8\x01\0\x10\xF8\x01\0H\xF8\x01\0P\xF8\x01\0Z\xF8\x01\0`\xF8\x01\0\x88\xF8\x01\0\x90\xF8\x01\0\xAE\xF8\x01\0\xB0\xF8\x01\0\xB2\xF8\x01\0\0\xF9\x01\0T\xFA\x01\0`\xFA\x01\0n\xFA\x01\0p\xFA\x01\0}\xFA\x01\0\x80\xFA\x01\0\x89\xFA\x01\0\x90\xFA\x01\0\xBE\xFA\x01\0\xBF\xFA\x01\0\xC6\xFA\x01\0\xCE\xFA\x01\0\xDC\xFA\x01\0\xE0\xFA\x01\0\xE9\xFA\x01\0\xF0\xFA\x01\0\xF9\xFA\x01\0\0\xFB\x01\0\x93\xFB\x01\0\x94\xFB\x01\0\xCB\xFB\x01\0\xF0\xFB\x01\0\xFA\xFB\x01\0\0\0\x02\0\xE0\xA6\x02\0\0\xA7\x02\0:\xB7\x02\0@\xB7\x02\0\x1E\xB8\x02\0 \xB8\x02\0\xA2\xCE\x02\0\xB0\xCE\x02\0\xE1\xEB\x02\0\0\xF8\x02\0\x1E\xFA\x02\0\0\0\x03\0K\x13\x03\0P\x13\x03\0\xB0#\x03\0") }, 146986u32)
            });
        }
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::properties::provider::GraphemeBaseV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::GraphemeBaseV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_GR_BASE_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::GraphemeBaseV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
