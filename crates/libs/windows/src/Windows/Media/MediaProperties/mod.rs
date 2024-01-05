::windows_core::imp::com_interface!(IAudioEncodingProperties, IAudioEncodingProperties_Vtbl, 0x62bc7a16_005c_4b3b_8a0b_0a090e9687f3);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetSampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub SampleRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetBitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BitsPerSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioEncodingProperties2, IAudioEncodingProperties2_Vtbl, 0xc45d54da_80bd_4c23_80d5_72d4a181e894);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSpatial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioEncodingProperties3, IAudioEncodingProperties3_Vtbl, 0x87600341_748c_4f8d_b0fd_10caf08ff087);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingProperties3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioEncodingPropertiesStatics, IAudioEncodingPropertiesStatics_Vtbl, 0x0cad332c_ebe9_4527_b36d_e42a13cf38db);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateAac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAacAdts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMp3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreatePcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitrate: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioEncodingPropertiesStatics2, IAudioEncodingPropertiesStatics2_Vtbl, 0x7489316f_77a0_433d_8ed5_4040280e8665);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateAlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, samplerate: u32, channelcount: u32, bitspersample: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAudioEncodingPropertiesWithFormatUserData, IAudioEncodingPropertiesWithFormatUserData_Vtbl, 0x98f10d79_13ea_49ff_be70_2673db69702c);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEncodingPropertiesWithFormatUserData_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IAv1ProfileIdsStatics, IAv1ProfileIdsStatics_Vtbl, 0x9105812b_7c09_5882_88a4_678008a5174d);
#[repr(C)]
#[doc(hidden)]
pub struct IAv1ProfileIdsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MainChromaSubsampling420BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling420BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling400BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling400BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub HighChromaSubsampling444BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub HighChromaSubsampling444BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProfessionalChromaSubsampling420BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProfessionalChromaSubsampling400BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProfessionalChromaSubsampling444BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProfessionalChromaSubsampling422BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProfessionalChromaSubsampling422BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub ProfessionalChromaSubsampling422BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IContainerEncodingProperties, IContainerEncodingProperties_Vtbl, 0x59ac2a57_b32a_479e_8a61_4b7f2e9e7ea0);
#[repr(C)]
#[doc(hidden)]
pub struct IContainerEncodingProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
::windows_core::imp::com_interface!(IContainerEncodingProperties2, IContainerEncodingProperties2_Vtbl, 0xb272c029_ae26_4819_baad_ad7a49b0a876);
#[repr(C)]
#[doc(hidden)]
pub struct IContainerEncodingProperties2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IH264ProfileIdsStatics, IH264ProfileIdsStatics_Vtbl, 0x38654ca7_846a_4f97_a2e5_c3a15bbf70fd);
#[repr(C)]
#[doc(hidden)]
pub struct IH264ProfileIdsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ConstrainedBaseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Baseline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Extended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Main: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub High: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub High10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub High422: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub High444: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub StereoHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MultiviewHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IHevcProfileIdsStatics, IHevcProfileIdsStatics_Vtbl, 0x1e50d280_2aa7_53c1_973f_2189fa656f53);
#[repr(C)]
#[doc(hidden)]
pub struct IHevcProfileIdsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MainChromaSubsampling420BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling420BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling420BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling422BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling422BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling444BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling444BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainChromaSubsampling444BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MonochromeBitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MonochromeBitDepth16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling420BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling420BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling420BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling422BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling422BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling444BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling444BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling444BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainIntraChromaSubsampling444BitDepth16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainStillChromaSubsampling420BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainStillChromaSubsampling444BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub MainStillChromaSubsampling444BitDepth16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IImageEncodingProperties, IImageEncodingProperties_Vtbl, 0x78625635_f331_4189_b1c3_b48d5ae034f1);
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IImageEncodingProperties2, IImageEncodingProperties2_Vtbl, 0xc854a2df_c923_469b_ac8e_6a9f3c1cd9e3);
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingProperties2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IImageEncodingPropertiesStatics, IImageEncodingPropertiesStatics_Vtbl, 0x257c68dc_8b99_439e_aa59_913a36161297);
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateJpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreatePng: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateJpegXR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IImageEncodingPropertiesStatics2, IImageEncodingPropertiesStatics2_Vtbl, 0xf6c25b29_3824_46b0_956e_501329e1be3c);
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: MediaPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateBmp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IImageEncodingPropertiesStatics3, IImageEncodingPropertiesStatics3_Vtbl, 0x48f4814d_a2ff_48dc_8ea0_e90680663656);
#[repr(C)]
#[doc(hidden)]
pub struct IImageEncodingPropertiesStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateHeif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingProfile, IMediaEncodingProfile_Vtbl, 0xe7dbf5a8_1db9_4783_876b_3dfe12acfdb3);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetAudio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Audio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Video: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Container: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingProfile2, IMediaEncodingProfile2_Vtbl, 0x349b3e0a_4035_488e_9877_85632865ed10);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetAudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetAudioTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetAudioTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetVideoTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetVideoTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetVideoTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetVideoTracks: usize,
}
::windows_core::imp::com_interface!(IMediaEncodingProfile3, IMediaEncodingProfile3_Vtbl, 0xba6ebe88_7570_4e69_accf_5611ad015f88);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfile3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub SetTimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    SetTimedMetadataTracks: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub GetTimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Core")))]
    GetTimedMetadataTracks: usize,
}
::windows_core::imp::com_interface!(IMediaEncodingProfileStatics, IMediaEncodingProfileStatics_Vtbl, 0x197f352c_2ede_4a45_a896_817a4854f8fe);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateM4a: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMp3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMp4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWmv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CreateFromFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFromFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamAsync: usize,
}
::windows_core::imp::com_interface!(IMediaEncodingProfileStatics2, IMediaEncodingProfileStatics2_Vtbl, 0xce8de74f_6af4_4288_8fe2_79adf1f79a43);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateWav: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAvi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingProfileStatics3, IMediaEncodingProfileStatics3_Vtbl, 0x90dac5aa_cf76_4294_a9ed_1a1420f51f6b);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateAlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: AudioEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateHevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingProfileStatics4, IMediaEncodingProfileStatics4_Vtbl, 0x6fafd7b5_9404_514a_81dd_c9444d648af0);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProfileStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateVp9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAv1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, quality: VideoEncodingQuality, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingProperties, IMediaEncodingProperties_Vtbl, 0xb4002af6_acd4_4e5a_a24b_5d7498a8b8c4);
impl IMediaEncodingProperties {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<MediaPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtype(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtype)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaEncodingProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::RuntimeType for IMediaEncodingProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSubtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Subtype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics, IMediaEncodingSubtypesStatics_Vtbl, 0x37b6580e_a171_4464_ba5a_53189e48c1c8);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Aac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AacAdts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Ac3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AmrNb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AmrWb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Argb32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Asf: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Avi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Bgra8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Bmp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Eac3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Float: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Gif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub H263: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub H264: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub H264Es: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Hevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HevcEs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Iyuv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Jpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub JpegXr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mjpg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mpeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mpeg1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mpeg2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mp3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Mpeg4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Nv12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Pcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Png: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rgb24: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Rgb32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Tiff: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wave: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wma8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wma9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wmv3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Wvc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Yuy2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Yv12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics2, IMediaEncodingSubtypesStatics2_Vtbl, 0x4b7cd23d_42ff_4d33_8531_0626bee4b52d);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Vp9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub L8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub L16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub D16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics3, IMediaEncodingSubtypesStatics3_Vtbl, 0xba2414e4_883d_464e_a44f_097da08ef7ff);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Alac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Flac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics4, IMediaEncodingSubtypesStatics4_Vtbl, 0xddece58a_3949_4644_8a2c_59ef02c642fa);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub P010: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics5, IMediaEncodingSubtypesStatics5_Vtbl, 0x5ad4a007_ffce_4760_9828_5d0c99637e6a);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Heif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics6, IMediaEncodingSubtypesStatics6_Vtbl, 0xa1252973_a984_5912_93bb_54e7e569e053);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics6_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Pgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Srt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Ssa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VobSub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaEncodingSubtypesStatics7, IMediaEncodingSubtypesStatics7_Vtbl, 0x92f2dca7_9937_52a1_b619_ddfad81cd99c);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaEncodingSubtypesStatics7_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Av1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMediaRatio, IMediaRatio_Vtbl, 0xd2d0fee5_8929_401d_ac78_7d357e378163);
#[repr(C)]
#[doc(hidden)]
pub struct IMediaRatio_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetNumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Numerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetDenominator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Denominator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IMpeg2ProfileIdsStatics, IMpeg2ProfileIdsStatics_Vtbl, 0xa461ff85_e57a_4128_9b21_d5331b04235c);
#[repr(C)]
#[doc(hidden)]
pub struct IMpeg2ProfileIdsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Simple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Main: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SignalNoiseRatioScalable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SpatiallyScalable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub High: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataEncodingProperties, ITimedMetadataEncodingProperties_Vtbl, 0x51cd30d3_d690_4cfa_97f4_4a398e9db420);
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ITimedMetadataEncodingPropertiesStatics, ITimedMetadataEncodingPropertiesStatics_Vtbl, 0x6629bb67_6e55_5643_89a0_7a7e8d85b52c);
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataEncodingPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreatePgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSrt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSsa: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateVobSub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, formatUserData_array_size: u32, formatuserdata: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingProperties, IVideoEncodingProperties_Vtbl, 0x76ee6c9a_37c2_4f2a_880a_1282bbb4373d);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetBitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Bitrate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub FrameRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub PixelAspectRatio: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingProperties2, IVideoEncodingProperties2_Vtbl, 0xf743a1ef_d465_4290_a94b_ef0f1528f8e3);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetFormatUserData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetProfileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ProfileId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingProperties3, IVideoEncodingProperties3_Vtbl, 0x386bcdc4_873a_479f_b3eb_56c1fcbec6d7);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StereoscopicVideoPackingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StereoscopicVideoPackingMode) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingProperties4, IVideoEncodingProperties4_Vtbl, 0x724ef014_c10c_40f2_9d72_3ee13b45fa8e);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SphericalVideoFrameFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SphericalVideoFrameFormat) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingProperties5, IVideoEncodingProperties5_Vtbl, 0x4959080f_272f_4ece_a4df_c0ccdb33d840);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingProperties5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingPropertiesStatics, IVideoEncodingPropertiesStatics_Vtbl, 0x3ce14d44_1dc5_43db_9f38_ebebf90152cb);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateH264: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMpeg2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateUncompressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subtype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, width: u32, height: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingPropertiesStatics2, IVideoEncodingPropertiesStatics2_Vtbl, 0xcf1ebd5d_49fe_4d00_b59a_cfa4dfc51944);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateHevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVideoEncodingPropertiesStatics3, IVideoEncodingPropertiesStatics3_Vtbl, 0x65b46685_60da_5e51_91a2_b38c4763b872);
#[repr(C)]
#[doc(hidden)]
pub struct IVideoEncodingPropertiesStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateVp9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateAv1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IVp9ProfileIdsStatics, IVp9ProfileIdsStatics_Vtbl, 0x20311a55_fe06_5883_92d9_6080c97743e5);
#[repr(C)]
#[doc(hidden)]
pub struct IVp9ProfileIdsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Profile0ChromaSubsampling420BitDepth8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Profile2ChromaSubsampling420BitDepth10: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub Profile2ChromaSubsampling420BitDepth12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct AudioEncodingProperties(::windows_core::IUnknown);
impl AudioEncodingProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioEncodingProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetChannelCount(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetChannelCount)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ChannelCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ChannelCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSampleRate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSampleRate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SampleRate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBitsPerSample(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitsPerSample)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BitsPerSample(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitsPerSample)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSpatial(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IAudioEncodingProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSpatial)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<AudioEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IAudioEncodingProperties3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateAac(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAac)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitrate, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateAacAdts(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAacAdts)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitrate, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateMp3(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMp3)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitrate, &mut result__).from_abi(result__)
        })
    }
    pub fn CreatePcm(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePcm)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitspersample, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWma(samplerate: u32, channelcount: u32, bitrate: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWma)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitrate, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateAlac(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAlac)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitspersample, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFlac(samplerate: u32, channelcount: u32, bitspersample: u32) -> ::windows_core::Result<AudioEncodingProperties> {
        Self::IAudioEncodingPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFlac)(::windows_core::Interface::as_raw(this), samplerate, channelcount, bitspersample, &mut result__).from_abi(result__)
        })
    }
    pub fn SetFormatUserData(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAudioEncodingPropertiesWithFormatUserData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFormatUserData)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn GetFormatUserData(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAudioEncodingPropertiesWithFormatUserData>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GetFormatUserData)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<MediaPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtype(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtype)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAudioEncodingPropertiesStatics<R, F: FnOnce(&IAudioEncodingPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioEncodingProperties, IAudioEncodingPropertiesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IAudioEncodingPropertiesStatics2<R, F: FnOnce(&IAudioEncodingPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioEncodingProperties, IAudioEncodingPropertiesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for AudioEncodingProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for AudioEncodingProperties {
    type Vtable = IAudioEncodingProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioEncodingProperties {
    const IID: ::windows_core::GUID = <IAudioEncodingProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.AudioEncodingProperties";
}
::windows_core::imp::interface_hierarchy!(AudioEncodingProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(AudioEncodingProperties, IMediaEncodingProperties);
unsafe impl ::core::marker::Send for AudioEncodingProperties {}
unsafe impl ::core::marker::Sync for AudioEncodingProperties {}
pub struct Av1ProfileIds;
impl Av1ProfileIds {
    pub fn MainChromaSubsampling420BitDepth8() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling420BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling420BitDepth10() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling420BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling400BitDepth8() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling400BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling400BitDepth10() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling400BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HighChromaSubsampling444BitDepth8() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighChromaSubsampling444BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HighChromaSubsampling444BitDepth10() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighChromaSubsampling444BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProfessionalChromaSubsampling420BitDepth12() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfessionalChromaSubsampling420BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProfessionalChromaSubsampling400BitDepth12() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfessionalChromaSubsampling400BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProfessionalChromaSubsampling444BitDepth12() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfessionalChromaSubsampling444BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProfessionalChromaSubsampling422BitDepth8() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfessionalChromaSubsampling422BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProfessionalChromaSubsampling422BitDepth10() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfessionalChromaSubsampling422BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn ProfessionalChromaSubsampling422BitDepth12() -> ::windows_core::Result<i32> {
        Self::IAv1ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfessionalChromaSubsampling422BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IAv1ProfileIdsStatics<R, F: FnOnce(&IAv1ProfileIdsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Av1ProfileIds, IAv1ProfileIdsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for Av1ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.Av1ProfileIds";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ContainerEncodingProperties(::windows_core::IUnknown);
impl ContainerEncodingProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ContainerEncodingProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Copy(&self) -> ::windows_core::Result<ContainerEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IContainerEncodingProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<MediaPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtype(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtype)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ContainerEncodingProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ContainerEncodingProperties {
    type Vtable = IContainerEncodingProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ContainerEncodingProperties {
    const IID: ::windows_core::GUID = <IContainerEncodingProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ContainerEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ContainerEncodingProperties";
}
::windows_core::imp::interface_hierarchy!(ContainerEncodingProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(ContainerEncodingProperties, IMediaEncodingProperties);
unsafe impl ::core::marker::Send for ContainerEncodingProperties {}
unsafe impl ::core::marker::Sync for ContainerEncodingProperties {}
pub struct H264ProfileIds;
impl H264ProfileIds {
    pub fn ConstrainedBaseline() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConstrainedBaseline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Baseline() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Baseline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Extended() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extended)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Main() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Main)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn High() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).High)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn High10() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).High10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn High422() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).High422)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn High444() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).High444)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn StereoHigh() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StereoHigh)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MultiviewHigh() -> ::windows_core::Result<i32> {
        Self::IH264ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MultiviewHigh)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IH264ProfileIdsStatics<R, F: FnOnce(&IH264ProfileIdsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<H264ProfileIds, IH264ProfileIdsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for H264ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.H264ProfileIds";
}
pub struct HevcProfileIds;
impl HevcProfileIds {
    pub fn MainChromaSubsampling420BitDepth8() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling420BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling420BitDepth10() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling420BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling420BitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling420BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling422BitDepth10() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling422BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling422BitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling422BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling444BitDepth8() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling444BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling444BitDepth10() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling444BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainChromaSubsampling444BitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainChromaSubsampling444BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MonochromeBitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonochromeBitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MonochromeBitDepth16() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MonochromeBitDepth16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling420BitDepth8() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling420BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling420BitDepth10() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling420BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling420BitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling420BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling422BitDepth10() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling422BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling422BitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling422BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling444BitDepth8() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling444BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling444BitDepth10() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling444BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling444BitDepth12() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling444BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainIntraChromaSubsampling444BitDepth16() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainIntraChromaSubsampling444BitDepth16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainStillChromaSubsampling420BitDepth8() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainStillChromaSubsampling420BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainStillChromaSubsampling444BitDepth8() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainStillChromaSubsampling444BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MainStillChromaSubsampling444BitDepth16() -> ::windows_core::Result<i32> {
        Self::IHevcProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainStillChromaSubsampling444BitDepth16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHevcProfileIdsStatics<R, F: FnOnce(&IHevcProfileIdsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<HevcProfileIds, IHevcProfileIdsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for HevcProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.HevcProfileIds";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ImageEncodingProperties(::windows_core::IUnknown);
impl ImageEncodingProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageEncodingProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<ImageEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IImageEncodingProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateJpeg() -> ::windows_core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJpeg)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreatePng() -> ::windows_core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePng)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateJpegXR() -> ::windows_core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateJpegXR)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUncompressed(format: MediaPixelFormat) -> ::windows_core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUncompressed)(::windows_core::Interface::as_raw(this), format, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateBmp() -> ::windows_core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateBmp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateHeif() -> ::windows_core::Result<ImageEncodingProperties> {
        Self::IImageEncodingPropertiesStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateHeif)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<MediaPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtype(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtype)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IImageEncodingPropertiesStatics<R, F: FnOnce(&IImageEncodingPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IImageEncodingPropertiesStatics2<R, F: FnOnce(&IImageEncodingPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IImageEncodingPropertiesStatics3<R, F: FnOnce(&IImageEncodingPropertiesStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageEncodingProperties, IImageEncodingPropertiesStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ImageEncodingProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for ImageEncodingProperties {
    type Vtable = IImageEncodingProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageEncodingProperties {
    const IID: ::windows_core::GUID = <IImageEncodingProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.ImageEncodingProperties";
}
::windows_core::imp::interface_hierarchy!(ImageEncodingProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(ImageEncodingProperties, IMediaEncodingProperties);
unsafe impl ::core::marker::Send for ImageEncodingProperties {}
unsafe impl ::core::marker::Sync for ImageEncodingProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaEncodingProfile(::windows_core::IUnknown);
impl MediaEncodingProfile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingProfile, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetAudio<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<AudioEncodingProperties>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudio)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Audio(&self) -> ::windows_core::Result<AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Audio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVideo<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VideoEncodingProperties>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetVideo)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Video(&self) -> ::windows_core::Result<VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Video)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContainer<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ContainerEncodingProperties>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContainer)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Container(&self) -> ::windows_core::Result<ContainerEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Container)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn SetAudioTracks<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IIterable<super::Core::AudioStreamDescriptor>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAudioTracks)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetAudioTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Core::AudioStreamDescriptor>> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAudioTracks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn SetVideoTracks<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IIterable<super::Core::VideoStreamDescriptor>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetVideoTracks)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetVideoTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Core::VideoStreamDescriptor>> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProfile2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVideoTracks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn SetTimedMetadataTracks<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Collections::IIterable<super::Core::TimedMetadataStreamDescriptor>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProfile3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTimedMetadataTracks)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Media_Core\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Core"))]
    pub fn GetTimedMetadataTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<super::Core::TimedMetadataStreamDescriptor>> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProfile3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetTimedMetadataTracks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateM4a(quality: AudioEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateM4a)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateMp3(quality: AudioEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMp3)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWma(quality: AudioEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWma)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateMp4(quality: VideoEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMp4)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWmv(quality: VideoEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWmv)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn CreateFromFileAsync<P0>(file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamAsync<P0>(stream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaEncodingProfile>>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IMediaEncodingProfileStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamAsync)(::windows_core::Interface::as_raw(this), stream.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWav(quality: AudioEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWav)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateAvi(quality: VideoEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAvi)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateAlac(quality: AudioEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAlac)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFlac(quality: AudioEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFlac)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateHevc(quality: VideoEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateHevc)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateVp9(quality: VideoEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVp9)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateAv1(quality: VideoEncodingQuality) -> ::windows_core::Result<MediaEncodingProfile> {
        Self::IMediaEncodingProfileStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAv1)(::windows_core::Interface::as_raw(this), quality, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics<R, F: FnOnce(&IMediaEncodingProfileStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics2<R, F: FnOnce(&IMediaEncodingProfileStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics3<R, F: FnOnce(&IMediaEncodingProfileStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingProfileStatics4<R, F: FnOnce(&IMediaEncodingProfileStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingProfile, IMediaEncodingProfileStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for MediaEncodingProfile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaEncodingProfile {
    type Vtable = IMediaEncodingProfile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaEncodingProfile {
    const IID: ::windows_core::GUID = <IMediaEncodingProfile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaEncodingProfile {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaEncodingProfile";
}
::windows_core::imp::interface_hierarchy!(MediaEncodingProfile, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaEncodingProfile {}
unsafe impl ::core::marker::Sync for MediaEncodingProfile {}
pub struct MediaEncodingSubtypes;
impl MediaEncodingSubtypes {
    pub fn Aac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Aac)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AacAdts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AacAdts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Ac3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ac3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AmrNb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AmrNb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AmrWb() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AmrWb)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Argb32() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Argb32)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Asf() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Asf)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Avi() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Avi)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Bgra8() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bgra8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Bmp() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bmp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Eac3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Eac3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Float() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Float)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Gif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Gif)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn H263() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).H263)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn H264() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).H264)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn H264Es() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).H264Es)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Hevc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Hevc)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn HevcEs() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HevcEs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Iyuv() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Iyuv)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Jpeg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Jpeg)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JpegXr() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JpegXr)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mjpg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mjpg)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mpeg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mpeg)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mpeg1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mpeg1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mpeg2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mpeg2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mp3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mp3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Mpeg4() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mpeg4)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Nv12() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Nv12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Pcm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pcm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Png() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Png)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Rgb24() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rgb24)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Rgb32() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rgb32)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Tiff() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tiff)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Wave() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wave)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Wma8() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wma8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Wma9() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wma9)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Wmv3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wmv3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Wvc1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Wvc1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Yuy2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Yuy2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Yv12() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Yv12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Vp9() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Vp9)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn L8() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).L8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn L16() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).L16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn D16() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).D16)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Alac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Alac)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Flac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Flac)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn P010() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).P010)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Heif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics5(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Heif)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Pgs() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Pgs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Srt() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Srt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Ssa() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ssa)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VobSub() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics6(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VobSub)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Av1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IMediaEncodingSubtypesStatics7(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Av1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics<R, F: FnOnce(&IMediaEncodingSubtypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics2<R, F: FnOnce(&IMediaEncodingSubtypesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics3<R, F: FnOnce(&IMediaEncodingSubtypesStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics4<R, F: FnOnce(&IMediaEncodingSubtypesStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics5<R, F: FnOnce(&IMediaEncodingSubtypesStatics5) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics5> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics6<R, F: FnOnce(&IMediaEncodingSubtypesStatics6) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics6> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaEncodingSubtypesStatics7<R, F: FnOnce(&IMediaEncodingSubtypesStatics7) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaEncodingSubtypes, IMediaEncodingSubtypesStatics7> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for MediaEncodingSubtypes {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaEncodingSubtypes";
}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MediaPropertySet {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaPropertySet, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: ::windows_core::GUID) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: ::windows_core::GUID, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key, value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for MediaPropertySet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for MediaPropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows_core::GUID, ::windows_core::IInspectable>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for MediaPropertySet {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for MediaPropertySet {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaPropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for MediaPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &MediaPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(MediaPropertySet, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::required_hierarchy!(MediaPropertySet, super::super::Foundation::Collections::IIterable::<super::super::Foundation::Collections::IKeyValuePair::<::windows_core::GUID, ::windows_core::IInspectable>>, super::super::Foundation::Collections::IMap::<::windows_core::GUID, ::windows_core::IInspectable>);
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaPropertySet {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct MediaRatio(::windows_core::IUnknown);
impl MediaRatio {
    pub fn SetNumerator(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetNumerator)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Numerator(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Numerator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDenominator(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDenominator)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Denominator(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Denominator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for MediaRatio {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for MediaRatio {
    type Vtable = IMediaRatio_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaRatio {
    const IID: ::windows_core::GUID = <IMediaRatio as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaRatio {
    const NAME: &'static str = "Windows.Media.MediaProperties.MediaRatio";
}
::windows_core::imp::interface_hierarchy!(MediaRatio, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaRatio {}
unsafe impl ::core::marker::Sync for MediaRatio {}
pub struct Mpeg2ProfileIds;
impl Mpeg2ProfileIds {
    pub fn Simple() -> ::windows_core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Simple)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Main() -> ::windows_core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Main)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SignalNoiseRatioScalable() -> ::windows_core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SignalNoiseRatioScalable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SpatiallyScalable() -> ::windows_core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpatiallyScalable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn High() -> ::windows_core::Result<i32> {
        Self::IMpeg2ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).High)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMpeg2ProfileIdsStatics<R, F: FnOnce(&IMpeg2ProfileIdsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Mpeg2ProfileIds, IMpeg2ProfileIdsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for Mpeg2ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.Mpeg2ProfileIds";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct TimedMetadataEncodingProperties(::windows_core::IUnknown);
impl TimedMetadataEncodingProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedMetadataEncodingProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<MediaPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtype(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtype)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormatUserData(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFormatUserData)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn GetFormatUserData(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GetFormatUserData)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn Copy(&self) -> ::windows_core::Result<TimedMetadataEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreatePgs() -> ::windows_core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreatePgs)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateSrt() -> ::windows_core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSrt)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateSsa(formatuserdata: &[u8]) -> ::windows_core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateSsa)(::windows_core::Interface::as_raw(this), formatuserdata.len().try_into().unwrap(), formatuserdata.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateVobSub(formatuserdata: &[u8]) -> ::windows_core::Result<TimedMetadataEncodingProperties> {
        Self::ITimedMetadataEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVobSub)(::windows_core::Interface::as_raw(this), formatuserdata.len().try_into().unwrap(), formatuserdata.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataEncodingPropertiesStatics<R, F: FnOnce(&ITimedMetadataEncodingPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedMetadataEncodingProperties, ITimedMetadataEncodingPropertiesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for TimedMetadataEncodingProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for TimedMetadataEncodingProperties {
    type Vtable = IMediaEncodingProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedMetadataEncodingProperties {
    const IID: ::windows_core::GUID = <IMediaEncodingProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.TimedMetadataEncodingProperties";
}
::windows_core::imp::interface_hierarchy!(TimedMetadataEncodingProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(TimedMetadataEncodingProperties, IMediaEncodingProperties);
unsafe impl ::core::marker::Send for TimedMetadataEncodingProperties {}
unsafe impl ::core::marker::Sync for TimedMetadataEncodingProperties {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct VideoEncodingProperties(::windows_core::IUnknown);
impl VideoEncodingProperties {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoEncodingProperties, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<MediaPropertySet> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubtype(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSubtype)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Subtype(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaEncodingProperties>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtype)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBitrate(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBitrate)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bitrate(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bitrate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Width(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Width)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Height)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameRate(&self) -> ::windows_core::Result<MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelAspectRatio(&self) -> ::windows_core::Result<MediaRatio> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelAspectRatio)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFormatUserData(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFormatUserData)(::windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn GetFormatUserData(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).GetFormatUserData)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetProfileId(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProfileId)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProfileId(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProfileId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn StereoscopicVideoPackingMode(&self) -> ::windows_core::Result<StereoscopicVideoPackingMode> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StereoscopicVideoPackingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SphericalVideoFrameFormat(&self) -> ::windows_core::Result<SphericalVideoFrameFormat> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SphericalVideoFrameFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<VideoEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IVideoEncodingProperties5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateH264() -> ::windows_core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateH264)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateMpeg2() -> ::windows_core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMpeg2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateUncompressed(subtype: &::windows_core::HSTRING, width: u32, height: u32) -> ::windows_core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateUncompressed)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(subtype), width, height, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateHevc() -> ::windows_core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateHevc)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateVp9() -> ::windows_core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateVp9)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateAv1() -> ::windows_core::Result<VideoEncodingProperties> {
        Self::IVideoEncodingPropertiesStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAv1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoEncodingPropertiesStatics<R, F: FnOnce(&IVideoEncodingPropertiesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IVideoEncodingPropertiesStatics2<R, F: FnOnce(&IVideoEncodingPropertiesStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IVideoEncodingPropertiesStatics3<R, F: FnOnce(&IVideoEncodingPropertiesStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoEncodingProperties, IVideoEncodingPropertiesStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for VideoEncodingProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for VideoEncodingProperties {
    type Vtable = IVideoEncodingProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoEncodingProperties {
    const IID: ::windows_core::GUID = <IVideoEncodingProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoEncodingProperties {
    const NAME: &'static str = "Windows.Media.MediaProperties.VideoEncodingProperties";
}
::windows_core::imp::interface_hierarchy!(VideoEncodingProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(VideoEncodingProperties, IMediaEncodingProperties);
unsafe impl ::core::marker::Send for VideoEncodingProperties {}
unsafe impl ::core::marker::Sync for VideoEncodingProperties {}
pub struct Vp9ProfileIds;
impl Vp9ProfileIds {
    pub fn Profile0ChromaSubsampling420BitDepth8() -> ::windows_core::Result<i32> {
        Self::IVp9ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Profile0ChromaSubsampling420BitDepth8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Profile2ChromaSubsampling420BitDepth10() -> ::windows_core::Result<i32> {
        Self::IVp9ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Profile2ChromaSubsampling420BitDepth10)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Profile2ChromaSubsampling420BitDepth12() -> ::windows_core::Result<i32> {
        Self::IVp9ProfileIdsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Profile2ChromaSubsampling420BitDepth12)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVp9ProfileIdsStatics<R, F: FnOnce(&IVp9ProfileIdsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<Vp9ProfileIds, IVp9ProfileIdsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for Vp9ProfileIds {
    const NAME: &'static str = "Windows.Media.MediaProperties.Vp9ProfileIds";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct AudioEncodingQuality(pub i32);
impl AudioEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Medium: Self = Self(2i32);
    pub const Low: Self = Self(3i32);
}
impl ::windows_core::TypeKind for AudioEncodingQuality {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioEncodingQuality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioEncodingQuality").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioEncodingQuality {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.AudioEncodingQuality;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaMirroringOptions(pub u32);
impl MediaMirroringOptions {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::windows_core::TypeKind for MediaMirroringOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaMirroringOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaMirroringOptions").field(&self.0).finish()
    }
}
impl MediaMirroringOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for MediaMirroringOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MediaMirroringOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MediaMirroringOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MediaMirroringOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MediaMirroringOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for MediaMirroringOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaMirroringOptions;u4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaPixelFormat(pub i32);
impl MediaPixelFormat {
    pub const Nv12: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
    pub const P010: Self = Self(2i32);
}
impl ::windows_core::TypeKind for MediaPixelFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaPixelFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaPixelFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaPixelFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaPixelFormat;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaRotation(pub i32);
impl MediaRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::windows_core::TypeKind for MediaRotation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaRotation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaRotation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaRotation;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct MediaThumbnailFormat(pub i32);
impl MediaThumbnailFormat {
    pub const Bmp: Self = Self(0i32);
    pub const Bgra8: Self = Self(1i32);
}
impl ::windows_core::TypeKind for MediaThumbnailFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaThumbnailFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaThumbnailFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaThumbnailFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.MediaThumbnailFormat;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SphericalVideoFrameFormat(pub i32);
impl SphericalVideoFrameFormat {
    pub const None: Self = Self(0i32);
    pub const Unsupported: Self = Self(1i32);
    pub const Equirectangular: Self = Self(2i32);
}
impl ::windows_core::TypeKind for SphericalVideoFrameFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SphericalVideoFrameFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SphericalVideoFrameFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SphericalVideoFrameFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.SphericalVideoFrameFormat;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct StereoscopicVideoPackingMode(pub i32);
impl StereoscopicVideoPackingMode {
    pub const None: Self = Self(0i32);
    pub const SideBySide: Self = Self(1i32);
    pub const TopBottom: Self = Self(2i32);
}
impl ::windows_core::TypeKind for StereoscopicVideoPackingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for StereoscopicVideoPackingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StereoscopicVideoPackingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StereoscopicVideoPackingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.StereoscopicVideoPackingMode;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct VideoEncodingQuality(pub i32);
impl VideoEncodingQuality {
    pub const Auto: Self = Self(0i32);
    pub const HD1080p: Self = Self(1i32);
    pub const HD720p: Self = Self(2i32);
    pub const Wvga: Self = Self(3i32);
    pub const Ntsc: Self = Self(4i32);
    pub const Pal: Self = Self(5i32);
    pub const Vga: Self = Self(6i32);
    pub const Qvga: Self = Self(7i32);
    pub const Uhd2160p: Self = Self(8i32);
    pub const Uhd4320p: Self = Self(9i32);
}
impl ::windows_core::TypeKind for VideoEncodingQuality {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoEncodingQuality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoEncodingQuality").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoEncodingQuality {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.MediaProperties.VideoEncodingQuality;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
