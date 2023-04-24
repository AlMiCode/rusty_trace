use std::path::PathBuf;

use anyhow::Context;

pub struct OpenImageDenoise(pub(crate) OIDNDevice);
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_DEFAULT: OIDNDeviceType = 0;
pub const OIDNDeviceType_OIDN_DEVICE_TYPE_CPU: OIDNDeviceType = 1;
pub type OIDNDeviceType = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct OIDNDeviceImpl {
    _unused: [u8; 0],
}
impl Clone for OIDNDeviceImpl {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OIDNDevice = *mut OIDNDeviceImpl;

#[repr(C)]
#[derive(Debug, Copy)]
pub struct OIDNFilterImpl {
    _unused: [u8; 0],
}
impl Clone for OIDNFilterImpl {
    fn clone(&self) -> Self {
        *self
    }
}
pub type OIDNFilter = *mut OIDNFilterImpl;

pub const OIDNFormat_OIDN_FORMAT_UNDEFINED: OIDNFormat = 0;
pub const OIDNFormat_OIDN_FORMAT_FLOAT: OIDNFormat = 1;
pub const OIDNFormat_OIDN_FORMAT_FLOAT2: OIDNFormat = 2;
pub const OIDNFormat_OIDN_FORMAT_FLOAT3: OIDNFormat = 3;
pub const OIDNFormat_OIDN_FORMAT_FLOAT4: OIDNFormat = 4;
pub const OIDNFormat_OIDN_FORMAT_HALF: OIDNFormat = 257;
pub const OIDNFormat_OIDN_FORMAT_HALF2: OIDNFormat = 258;
pub const OIDNFormat_OIDN_FORMAT_HALF3: OIDNFormat = 259;
pub const OIDNFormat_OIDN_FORMAT_HALF4: OIDNFormat = 260;
pub type OIDNFormat = ::std::os::raw::c_uint;

impl OpenImageDenoise {
    pub fn denoise(
        input: &[f32],
        dimensions: (usize, usize),
        output: &mut [f32],
    ) -> anyhow::Result<()> {
        let mut lib_dir = PathBuf::from(std::env::var("OIDN_DIR")?);
        lib_dir.push("lib");
        lib_dir.push(libloading::library_filename("OpenImageDenoise"));
        unsafe {
            let lib = libloading::Library::new(lib_dir)?;
            let oidn_new_device: libloading::Symbol<
                unsafe extern "C" fn(type_: OIDNDeviceType) -> OIDNDevice,
            > = lib.get(b"oidnNewDevice")?;
            let oidn_commit_device: libloading::Symbol<unsafe extern "C" fn(device: OIDNDevice)> =
                lib.get(b"oidnCommitDevice")?;
            let oidn_retain_device: libloading::Symbol<unsafe extern "C" fn(device: OIDNDevice)> =
                lib.get(b"oidnRetainDevice")?;
            let oidn_release_device: libloading::Symbol<unsafe extern "C" fn(device: OIDNDevice)> =
                lib.get(b"oidnReleaseDevice")?;

            let oidn_new_filter: libloading::Symbol<
                unsafe extern "C" fn(
                    device: OIDNDevice,
                    type_: *const ::std::os::raw::c_char,
                ) -> OIDNFilter,
            > = lib.get(b"oidnNewFilter")?;
            //let oidn_retain_filter: libloading::Symbol<unsafe extern fn (filter: OIDNFilter)> = lib.get(b"oidnRetainFilter")?;
            let oidn_set_shared_filter_image: libloading::Symbol<
                unsafe extern "C" fn(
                    filter: OIDNFilter,
                    name: *const ::std::os::raw::c_char,
                    ptr: *mut ::std::os::raw::c_void,
                    format: OIDNFormat,
                    width: usize,
                    height: usize,
                    byteOffset: usize,
                    bytePixelStride: usize,
                    byteRowStride: usize,
                ),
            > = lib.get(b"oidnSetSharedFilterImage")?;
            let oidn_set_filter1b: libloading::Symbol<
                unsafe extern "C" fn(
                    filter: OIDNFilter,
                    name: *const ::std::os::raw::c_char,
                    value: bool,
                ),
            > = lib.get(b"oidnSetFilter1b")?;
            let oidn_set_filter1f: libloading::Symbol<
                unsafe extern "C" fn(
                    filter: OIDNFilter,
                    name: *const ::std::os::raw::c_char,
                    value: f32,
                ),
            > = lib.get(b"oidnSetFilter1f")?;
            let oidn_commit_filter: libloading::Symbol<unsafe extern "C" fn(filter: OIDNFilter)> =
                lib.get(b"oidnCommitFilter")?;
            let oidn_execute_filter: libloading::Symbol<unsafe extern "C" fn(filter: OIDNFilter)> =
                lib.get(b"oidnExecuteFilter")?;
            let oidn_release_filter: libloading::Symbol<unsafe extern "C" fn(filter: OIDNFilter)> =
                lib.get(b"oidnReleaseFilter")?;

            let device = oidn_new_device(OIDNDeviceType_OIDN_DEVICE_TYPE_DEFAULT);
            oidn_commit_device(device);
            oidn_retain_device(device);

            let filter = oidn_new_filter(device, b"RT\0" as *const _ as _);

            let buffer_dims = 3 * dimensions.0 * dimensions.1;
            if input.len() != buffer_dims {
                return None.context("Dimensions do not match");
            }
            let input_ptr = input.as_ptr();
            oidn_set_shared_filter_image(
                filter,
                b"color\0" as *const _ as _,
                input_ptr as *mut _,
                OIDNFormat_OIDN_FORMAT_FLOAT3,
                dimensions.0 as _,
                dimensions.1 as _,
                0,
                0,
                0,
            );

            if output.len() != buffer_dims {
                return None.context("Dimensions do not match 2");
            }
            oidn_set_shared_filter_image(
                filter,
                b"output\0" as *const _ as _,
                output.as_mut_ptr() as *mut _,
                OIDNFormat_OIDN_FORMAT_FLOAT3,
                dimensions.0 as _,
                dimensions.1 as _,
                0,
                0,
                0,
            );

            oidn_set_filter1b(filter, b"hdr\0" as *const _ as _, false);
            oidn_set_filter1f(filter, b"inputScale\0" as *const _ as _, f32::NAN);
            oidn_set_filter1b(filter, b"srgb\0" as *const _ as _, true);
            oidn_set_filter1b(filter, b"clean_aux\0" as *const _ as _, false);

            oidn_commit_filter(filter);
            oidn_execute_filter(filter);

            oidn_release_filter(filter);
            oidn_release_device(device);
        };
        Ok(())
    }
}
