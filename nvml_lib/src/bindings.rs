
#[repr(C)]
#[derive(Debug)]
pub enum NvmlReturn {
    NVML_SUCCESS = 0,
    NVML_ERROR_UNINITIALIZED = 1,
    NVML_ERROR_INVALID_ARGUMENT = 2,
    NVML_ERROR_NOT_SUPPORTED = 3,
    NVML_ERROR_NO_PERMISSION = 4,
    NVML_ERROR_ALREADY_INITIALIZED = 5,
    NVML_ERROR_NOT_FOUND = 6,
    NVML_ERROR_INSUFFICIENT_SIZE = 7,
    NVML_ERROR_INSUFFICIENT_POWER = 8,
    NVML_ERROR_DRIVER_NOT_LOADED = 9,
    NVML_ERROR_TIMEOUT = 10,
    NVML_ERROR_IRQ_ISSUE = 11,    
    NVML_ERROR_LIBRARY_NOT_FOUND = 12,
    NVML_ERROR_FUNCTION_NOT_FOUND = 13,
    NVML_ERROR_CORRUPTED_INFOROM = 14,
    NVML_ERROR_GPU_IS_LOST = 15,
    NVML_ERROR_RESET_REQUIRED = 16,
    NVML_ERROR_OPERATING_SYSTEM = 17,
    NVML_ERROR_LIB_RM_VERSION_MISMATCH = 18,
    NVML_ERROR_UNKNOWN = 999,  
}

#[repr(C)]
pub struct NvmlDevice;
pub type NvmlDeviceHandle = *mut NvmlDevice;

extern "C" {
    pub fn nvmlInit() -> NvmlReturn;
    pub fn nvmlShutdown() -> NvmlReturn;
    pub fn nvmlDeviceGetCount_v2(count: *mut u32) -> NvmlReturn;
    pub fn nvmlDeviceGetHandleByIndex_v2(index: u32, device: *mut u64) -> NvmlReturn;
    pub fn nvmlDeviceGetName(device: u64, name: *mut u8, length: u32) -> NvmlReturn;
}