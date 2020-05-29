#![no_std]
use libc::{c_char, c_int, c_uint, c_ulonglong, c_void, size_t};

// lz4.h
pub type LZ4StreamEncode = c_void;

pub type LZ4StreamDecode = c_void;

extern "C" {
    // int LZ4_versionNumber(void)
    pub fn LZ4_versionNumber() -> c_int;

    // const char* LZ4_versionString (void);
    pub fn LZ4_versionString() -> *const u8;

    // int LZ4_compress_default(const char* src, char* dst, int srcSize, int dstCapacity);
    pub fn LZ4_compress_default(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
    ) -> c_int;

    // int LZ4_decompress_safe (const char* src, char* dst, int compressedSize, int dstCapacity);
    pub fn LZ4_decompress_safe(
        src: *const c_char,
        dst: *mut c_char,
        compressedSize: c_int,
        dstCapacity: c_int,
    ) -> c_int;

    // int LZ4_compressBound(int inputSize)
    pub fn LZ4_compressBound(inputSize: c_int) -> c_int;

    // int LZ4_compress_fast (const char* src, char* dst, int srcSize, int dstCapacity, int acceleration);
    pub fn LZ4_compress_fast(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
        acceleration: c_int,
    ) -> c_int;

    // int LZ4_sizeofState(void);
    pub fn LZ4_sizeofState() -> c_int;

    // int LZ4_compress_fast_extState (void* state, const char* src, char* dst, int srcSize, int dstCapacity, int acceleration);
    pub fn LZ4_compress_fast_extState(
        state: *mut c_void,
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
        acceleration: c_int,
    ) -> c_int;

    // int LZ4_compress_destSize (const char* src, char* dst, int* srcSizePtr, int targetDstSize);
    pub fn LZ4_compress_destSize(
        src: *const c_char,
        dst: *mut c_char,
        srcSizePtr: *mut c_int,
        targetDstSize: c_int,
    ) -> c_int;

    // int LZ4_decompress_safe_partial (const char* src, char* dst, int srcSize, int targetOutputSize, int dstCapacity);
    pub fn LZ4_decompress_safe_partial(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        targetOutputSize: c_int,
        dstCapacity: c_int,
    ) -> c_int;

    // LZ4_stream_t* LZ4_createStream(void)
    pub fn LZ4_createStream() -> *mut LZ4StreamEncode;

    // int LZ4_freeStream(LZ4_stream_t* LZ4_streamPtr)
    pub fn LZ4_freeStream(LZ4_stream: *mut LZ4StreamEncode) -> c_int;

    // void LZ4_resetStream_fast (LZ4_stream_t* streamPtr);
    pub fn LZ4_resetStream_fast(LZ4_stream: *mut LZ4StreamEncode) -> c_void;

    // int LZ4_loadDict (LZ4_stream_t* streamPtr, const char* dictionary, int dictSize);
    pub fn LZ4_loadDict(
        streamPtr: *mut LZ4StreamEncode,
        dictionary: *const c_char,
        dictSize: c_int,
    ) -> c_int;

    // int LZ4_compress_fast_continue (LZ4_stream_t* streamPtr, const char* src, char* dst, int srcSize, int dstCapacity, int acceleration);
    pub fn LZ4_compress_fast_continue(
        streamPtr: *mut LZ4StreamEncode,
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
        acceleration: c_int,
    ) -> c_int;

    // int LZ4_saveDict (LZ4_stream_t* streamPtr, char* safeBuffer, int maxDictSize);
    pub fn LZ4_saveDict(
        streamPtr: *mut LZ4StreamEncode,
        safeBuffer: *mut c_char,
        maxDictSize: c_int,
    ) -> c_int;

    // LZ4_streamDecode_t* LZ4_createStreamDecode(void);
    pub fn LZ4_createStreamDecode() -> *mut LZ4StreamDecode;

    // int LZ4_freeStreamDecode (LZ4_streamDecode_t* LZ4_stream);
    pub fn LZ4_freeStreamDecode(LZ4_stream: *mut LZ4StreamDecode) -> c_int;

    // int LZ4_setStreamDecode (LZ4_streamDecode_t* LZ4_streamDecode, const char* dictionary, int dictSize);
    pub fn LZ4_setStreamDecode(
        LZ4_stream: *mut LZ4StreamDecode,
        dictionary: *const c_char,
        dictSize: c_int,
    ) -> c_int;

    // int LZ4_decoderRingBufferSize(int maxBlockSize);
    pub fn LZ4_decoderRingBufferSize(maxBlockSize: c_int) -> c_int;

    // int LZ4_decompress_safe_continue (LZ4_streamDecode_t* LZ4_streamDecode, const char* src, char* dst, int srcSize, int dstCapacity);
    pub fn LZ4_decompress_safe_continue(
        LZ4_stream: *mut LZ4StreamDecode,
        src: *const u8,
        dst: *mut u8,
        srcSize: c_int,
        dstCapacity: c_int,
    ) -> c_int;

    // int LZ4_decompress_safe_usingDict (const char* src, char* dst, int srcSize, int dstCapcity, const char* dictStart, int dictSize);
    pub fn LZ4_decompress_safe_usingDict(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
        dictStart: *const c_char,
        dictSize: c_int,
    ) -> c_int;
}

// lz4hc.h
pub type LZ4StreamHC = c_void;

extern "C" {
    // int LZ4_compress_HC (const char* src, char* dst, int srcSize, int dstCapacity, int compressionLevel);
    pub fn LZ4_compress_HC(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
        compressionLevel: c_int,
    ) -> c_int;

    // int LZ4_sizeofStateHC(void);
    pub fn LZ4_sizeofStateHC() -> c_int;

    // int LZ4_compress_HC_extStateHC(void* stateHC, const char* src, char* dst, int srcSize, int maxDstSize, int compressionLevel);
    pub fn LZ4_compress_HC_extStateHC(
        stateHC: *mut c_void,
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        maxDstSize: c_int,
        compressionLevel: c_int,
    ) -> c_int;

    // int LZ4_compress_HC_destSize(void* stateHC, const char* src, char* dst, int* srcSizePtr, int targetDstSize, int compressionLevel);
    pub fn LZ4_compress_HC_destSize(
        stateHC: *mut c_void,
        src: *const c_char,
        dst: *mut c_char,
        srcSizePtr: *mut c_int,
        targetDstSize: c_int,
        compressionLevel: c_int,
    ) -> c_int;

    // LZ4_streamHC_t* LZ4_createStreamHC(void);
    pub fn LZ4_createStreamHC() -> *mut LZ4StreamHC;

    // int LZ4_freeStreamHC (LZ4_streamHC_t* streamHCPtr);
    pub fn LZ4_freeStreamHC(streamHCPtr: *mut LZ4StreamHC) -> c_int;

    // void LZ4_resetStreamHC_fast(LZ4_streamHC_t* streamHCPtr, int compressionLevel);
    pub fn LZ4_resetStreamHC_fast(streamHCPtr: *mut LZ4StreamHC, compressionLevel: c_int)
        -> c_void;

    // int LZ4_loadDictHC (LZ4_streamHC_t* streamHCPtr, const char* dictionary, int dictSize);
    pub fn LZ4_loadDictHC(
        streamHCPtr: *mut LZ4StreamHC,
        dictionary: *const c_char,
        dictSize: c_int,
    ) -> c_int;

    // int LZ4_compress_HC_continue (LZ4_streamHC_t* streamHCPtr, const char* src, char* dst, int srcSize, int maxDstSize);
    pub fn LZ4_compress_HC_continue(
        streamHCPtr: *mut LZ4StreamHC,
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        maxDstSize: c_int,
    ) -> c_int;

    // int LZ4_compress_HC_continue_destSize(LZ4_streamHC_t* LZ4_streamHCPtr, const char* src, char* dst, int* srcSizePtr, int targetDstSize);
    pub fn LZ4_compress_HC_continue_destSize(
        LZ4_streamHCPtr: *mut LZ4StreamHC,
        src: *const c_char,
        dst: *mut c_char,
        srcSizePtr: *mut c_int,
        targetDstSize: c_int,
    ) -> c_int;

    // int LZ4_saveDictHC (LZ4_streamHC_t* streamHCPtr, char* safeBuffer, int maxDictSize);
    pub fn LZ4_saveDictHC(
        streamHCPtr: *mut LZ4StreamHC,
        safeBuffer: *mut c_char,
        maxDictSize: c_int,
    ) -> c_int;
}

// lz4frame.h
pub type LZ4FErrorCode = size_t;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LZ4FBlockSize {
    Default = 0,
    Max64KB = 4,
    Max256KB = 5,
    Max1MB = 6,
    Max4MB = 7,
}

impl LZ4FBlockSize {
    pub fn get_size(&self) -> usize {
        use LZ4FBlockSize::*;

        match *self {
            Default | Max64KB => 64 * 1024,
            Max256KB => 256 * 1024,
            Max1MB => 1024 * 1024,
            Max4MB => 4 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LZ4FBlockMode {
    Linked = 0,
    Independent,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LZ4FContentChecksum {
    NoChecksum = 0,
    ChecksumEnabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LZ4FFrameType {
    Frame = 0,
    Skippable,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LZ4FBlockChecksum {
    NoChecksum = 0,
    ChecksumEnabled,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct LZ4FFrameInfo {
    pub block_size: LZ4FBlockSize,
    pub block_mode: LZ4FBlockMode,
    pub content_checksum: LZ4FContentChecksum,
    pub frame_type: LZ4FFrameType,
    pub content_size: c_ulonglong,
    pub dict_id: c_uint,
    pub block_checksum: LZ4FBlockChecksum,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct LZ4FPreferences {
    pub frame_info: LZ4FFrameInfo,
    pub compression_level: c_int,
    pub auto_flush: c_uint,
    pub favor_dec_speed: c_uint,
    reserved: [c_uint; 3],
}

pub type LZ4FCompressionContext = c_void;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct LZ4FCompressOptions {
    pub stable_src: c_uint,
    reserved: [c_uint; 3],
}

pub type LZ4FDecompressionContext = c_void;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct LZ4FDecompressOptions {
    pub stable_dst: c_uint,
    reserved: [c_uint; 3],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub enum LZ4FErrorCodes {
    NoError = 0,
    GENERIC,
    MaxBlockSizeInvalid,
    BlockModeInvalid,
    ContentChecksumFlagInvalid,
    CompressionLevelInvalid,
    HeaderVersionWrong,
    BlockChecksumInvalid,
    ReservedFlagSet,
    AllocationFailed,
    SrcSizeTooLarge,
    DstMaxSizeTooSmall,
    FrameHeaderIncomplete,
    FrameTypeUnknown,
    FrameSizeWrong,
    SrcPtrWrong,
    DecompressionFailed,
    HeaderChecksumInvalid,
    ContentChecksumInvalid,
    FrameDecodingAlreadyStarted,
    MaxCode,
}

pub type LZ4FCDict = c_void;

extern "C" {
    // unsigned LZ4F_isError(LZ4F_errorCode_t code);
    pub fn LZ4F_isError(code: LZ4FErrorCode) -> c_uint;

    // const char* LZ4F_getErrorName(LZ4F_errorCode_t code);
    pub fn LZ4F_getErrorName(code: LZ4FErrorCode) -> *const c_char;

    // int LZ4F_compressionLevel_max(void);
    pub fn LZ4F_compressionLevel_max() -> c_int;

    // size_t LZ4F_compressFrameBound(size_t srcSize, const LZ4F_preferences_t* preferencesPtr);
    pub fn LZ4F_compressFrameBound(
        srcSize: size_t,
        preferencesPtr: *const LZ4FPreferences,
    ) -> size_t;

    // size_t LZ4F_compressFrame(void* dstBuffer, size_t dstCapacity, const void* srcBuffer, size_t srcSize, const LZ4F_preferences_t* preferencesPtr);
    pub fn LZ4F_compressFrame(
        dstBuffer: *mut c_void,
        dstCapacity: size_t,
        srcBuffer: *const c_void,
        srcSize: size_t,
        preferencesPtr: *const LZ4FPreferences,
    ) -> size_t;

    // unsigned LZ4F_getVersion(void);
    pub fn LZ4F_getVersion() -> c_uint;

    // LZ4F_errorCode_t LZ4F_createCompressionContext(LZ4F_cctx** cctxPtr, unsigned version);
    pub fn LZ4F_createCompressionContext(
        cctxPtr: *mut *mut LZ4FCompressionContext,
        version: c_uint,
    ) -> LZ4FErrorCode;

    // LZ4F_errorCode_t LZ4F_freeCompressionContext(LZ4F_cctx* cctx);
    pub fn LZ4F_freeCompressionContext(cctx: *mut LZ4FCompressionContext) -> LZ4FErrorCode;

    // size_t LZ4F_compressBegin(LZ4F_cctx* cctx, void* dstBuffer, size_t dstCapacity, const LZ4F_preferences_t* prefsPtr);
    pub fn LZ4F_compressBegin(
        cctx: *mut LZ4FCompressionContext,
        dstBuffer: *mut c_void,
        dstCapacity: size_t,
        prefsPtr: *const LZ4FPreferences,
    ) -> size_t;

    // size_t LZ4F_compressBound(size_t srcSize, const LZ4F_preferences_t* prefsPtr);
    pub fn LZ4F_compressBound(srcSize: size_t, prefsPtr: *const LZ4FPreferences) -> size_t;

    // size_t LZ4F_compressUpdate(LZ4F_cctx* cctx, void* dstBuffer, size_t dstCapacity, const void* srcBuffer, size_t srcSize, const LZ4F_compressOptions_t* cOptPtr);
    pub fn LZ4F_compressUpdate(
        cctx: *mut LZ4FCompressionContext,
        dstBuffer: *mut c_void,
        dstCapacity: size_t,
        srcBuffer: *const c_void,
        srcSize: size_t,
        cOptPtr: *const LZ4FCompressOptions,
    ) -> size_t;

    // size_t LZ4F_flush(LZ4F_cctx* cctx, void* dstBuffer, size_t dstCapacity, const LZ4F_compressOptions_t* cOptPtr);
    pub fn LZ4F_flush(
        cctx: *mut LZ4FCompressionContext,
        dstBuffer: *mut c_void,
        dstCapacity: size_t,
        cOptPtr: *const LZ4FCompressOptions,
    ) -> size_t;

    // size_t LZ4F_compressEnd(LZ4F_cctx* cctx, void* dstBuffer, size_t dstCapacity, const LZ4F_compressOptions_t* cOptPtr);
    pub fn LZ4F_compressEnd(
        cctx: *mut LZ4FCompressionContext,
        dstBuffer: *mut c_void,
        dstCapacity: size_t,
        cOptPtr: *const LZ4FCompressOptions,
    ) -> size_t;

    // LZ4F_errorCode_t LZ4F_createDecompressionContext(LZ4F_dctx** dctxPtr, unsigned version);
    pub fn LZ4F_createDecompressionContext(
        dctxPtr: *mut *mut LZ4FDecompressionContext,
        version: c_uint,
    ) -> LZ4FErrorCode;

    // LZ4F_errorCode_t LZ4F_freeDecompressionContext(LZ4F_dctx* dctx);
    pub fn LZ4F_freeDecompressionContext(dctx: *mut LZ4FDecompressionContext) -> LZ4FErrorCode;

    // size_t LZ4F_headerSize(const void* src, size_t srcSize);
    pub fn LZ4F_headerSize(src: *const c_void, srcSize: size_t) -> size_t;

    // size_t LZ4F_getFrameInfo(LZ4F_dctx* dctx, LZ4F_frameInfo_t* frameInfoPtr, const void* srcBuffer, size_t* srcSizePtr);
    pub fn LZ4F_getFrameInfo(
        dctx: *mut LZ4FDecompressionContext,
        frameInfoPtr: *mut LZ4FFrameInfo,
        srcBuffer: *const c_void,
        srcSizePtr: *mut size_t,
    ) -> size_t;

    // size_t LZ4F_decompress(LZ4F_dctx* dctx, void* dstBuffer, size_t* dstSizePtr, const void* srcBuffer, size_t* srcSizePtr, const LZ4F_decompressOptions_t* dOptPtr);
    pub fn LZ4F_decompress(
        dctx: *mut LZ4FDecompressionContext,
        dstBuffer: *mut c_void,
        dstSizePtr: *mut size_t,
        srcBuffer: *const c_void,
        srcSizePtr: *mut size_t,
        dOptPtr: *const LZ4FDecompressOptions,
    ) -> size_t;

    // void LZ4F_resetDecompressionContext(LZ4F_dctx* dctx);
    pub fn LZ4F_resetDecompressionContext(dctx: *mut LZ4FDecompressionContext) -> c_void;

    // LZ4F_errorCodes LZ4F_getErrorCode(size_t functionResult);
    pub fn LZ4F_getErrorCode(functionResult: size_t) -> LZ4FErrorCodes;

    // size_t LZ4F_getBlockSize(unsigned);
    // See LZ4FBlockSize::get_size

    // LZ4F_CDict* LZ4F_createCDict(const void* dictBuffer, size_t dictSize);
    pub fn LZ4F_createCDict(dictBuffer: *const c_void, dictSize: size_t) -> *mut LZ4FCDict;

    // void LZ4F_freeCDict(LZ4F_CDict* CDict);
    pub fn LZ4F_freeCDict(cdict: *mut LZ4FCDict) -> c_void;

    // size_t LZ4F_compressFrame_usingCDict(LZ4F_cctx* cctx, void* dst, size_t dstCapacity, const void* src, size_t srcSize, const LZ4F_CDict* cdict, const LZ4F_preferences_t* preferencesPtr);
    pub fn LZ4F_compressFrame_usingCDict(
        cctx: *mut LZ4FCompressionContext,
        dst: *mut c_void,
        dstCapacity: size_t,
        src: *const c_void,
        srcSize: size_t,
        cdict: *const LZ4FCDict,
        preferencesPtr: *const LZ4FPreferences,
    ) -> size_t;

    // size_t LZ4F_compressBegin_usingCDict(LZ4F_cctx* cctx,void* dstBuffer, size_t dstCapacity,const LZ4F_CDict* cdict,const LZ4F_preferences_t* prefsPtr);
    pub fn LZ4F_compressBegin_usingCDict(
        cctx: *mut LZ4FCompressionContext,
        dstBuffer: *mut c_void,
        dstCapacity: size_t,
        cdict: *const LZ4FCDict,
        prefsPtr: LZ4FPreferences,
    ) -> size_t;

    // size_t LZ4F_decompress_usingDict(LZ4F_dctx* dctxPtr, void* dstBuffer, size_t* dstSizePtr, const void* srcBuffer, size_t* srcSizePtr, const void* dict, size_t dictSize, const LZ4F_decompressOptions_t* decompressOptionsPtr);
    pub fn LZ4F_decompress_usingDict(
        dctxPtr: *mut LZ4FDecompressionContext,
        dstBuffer: *mut c_void,
        dstSizePtr: *mut size_t,
        srcBuffer: *const c_void,
        srcSizePtr: *mut size_t,
        dict: *const c_void,
        dictSize: size_t,
        decompressOptionsPtr: LZ4FDecompressOptions,
    ) -> size_t;
}

#[cfg(test)]
mod tests {
    use super::LZ4_versionNumber;

    #[test]
    fn test_version_number() {
        unsafe {
            LZ4_versionNumber();
        }
    }
}
