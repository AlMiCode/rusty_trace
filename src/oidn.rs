use std::path::PathBuf;

use anyhow::anyhow;

const OIDNDEVICE_TYPE_OIDN_DEVICE_TYPE_DEFAULT: OIDNDeviceType = 0;
type OIDNDeviceType = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy)]
struct OIDNDeviceImpl {
    _unused: [u8; 0],
}
impl Clone for OIDNDeviceImpl {
    fn clone(&self) -> Self {
        *self
    }
}
type OIDNDevice = *mut OIDNDeviceImpl;

#[repr(C)]
#[derive(Debug, Copy)]
struct OIDNFilterImpl {
    _unused: [u8; 0],
}
impl Clone for OIDNFilterImpl {
    fn clone(&self) -> Self {
        *self
    }
}
type OIDNFilter = *mut OIDNFilterImpl;

const OIDNFORMAT_OIDN_FORMAT_FLOAT3: OIDNFormat = 3;
type OIDNFormat = ::std::os::raw::c_uint;

lazy_static::lazy_static! {
    pub static ref OIND: OpenImageDenoiseAPI = OpenImageDenoiseAPI::new();
}

pub struct OpenImageDenoiseAPI {
    lib: Option<libloading::Library>,
}

impl OpenImageDenoiseAPI {
    fn new() -> Self {
        let lib_dir = std::env::var("OIDN_DIR");
        if let Err(e) = lib_dir {
            eprintln!("Denoiser not found: {}", e);
            return Self { lib: None };
        }
        let mut path = PathBuf::from(lib_dir.unwrap());
        path.push("lib");
        path.push(libloading::library_filename("OpenImageDenoise"));
        let lib = unsafe { libloading::Library::new(path).ok() };
        Self { lib }
    }

    pub fn availible(&self) -> bool {
        self.lib.is_some()
    }

    pub fn denoise(&self, image: &mut image::Rgb32FImage) {
        let dims = image.dimensions();
        let _result = unsafe {
            self.denoise_internal(
                (dims.0 as usize, dims.1 as usize),
                image.as_mut(),
                None,
                None,
            )
        };
    }

    pub fn denoise_extended(
        &self,
        image: &mut image::Rgb32FImage,
        albedo: &image::Rgb32FImage,
        normal: &image::Rgb32FImage,
    ) {
        let dims = image.dimensions();

        let _result = unsafe {
            self.denoise_internal(
                (dims.0 as usize, dims.1 as usize),
                image.as_mut(),
                Some(albedo.as_raw()),
                Some(normal.as_raw()),
            )
        };
    }

    unsafe fn denoise_internal(
        &self,
        dimensions: (usize, usize),
        image: &mut [f32],
        albedo: Option<&[f32]>,
        normal: Option<&[f32]>,
    ) -> anyhow::Result<()> {
        if self.lib.is_none() {
            return Err(anyhow!("Library is not loaded"));
        }
        let lib = self.lib.as_ref().unwrap();

        use libloading::Symbol;
        let oidn_new_device: Symbol<OIDNNewDeviceFn> = lib.get(b"oidnNewDevice")?;
        let oidn_commit_device: Symbol<OIDNManageDeviceFn> = lib.get(b"oidnCommitDevice")?;
        let oidn_retain_device: Symbol<OIDNManageDeviceFn> = lib.get(b"oidnRetainDevice")?;
        let oidn_release_device: Symbol<OIDNManageDeviceFn> = lib.get(b"oidnReleaseDevice")?;

        let oidn_new_filter: Symbol<OIDNNewFilterFn> = lib.get(b"oidnNewFilter")?;
        //let oidn_retain_filter: Symbol<OIDNManageFilterFn> = lib.get(b"oidnRetainFilter")?;
        let oidn_set_shared_filter_image: Symbol<OIDNSetSharedFilterImageFn> =
            lib.get(b"oidnSetSharedFilterImage")?;
        let oidn_set_filter1b: Symbol<OIDNSetFilter1b> = lib.get(b"oidnSetFilter1b")?;
        let oidn_set_filter1f: Symbol<OIDNSetFilter1f> = lib.get(b"oidnSetFilter1f")?;
        let oidn_commit_filter: Symbol<OIDNManageFilterFn> = lib.get(b"oidnCommitFilter")?;
        let oidn_execute_filter: Symbol<OIDNManageFilterFn> = lib.get(b"oidnExecuteFilter")?;
        let oidn_release_filter: Symbol<OIDNManageFilterFn> = lib.get(b"oidnReleaseFilter")?;

        let device = oidn_new_device(OIDNDEVICE_TYPE_OIDN_DEVICE_TYPE_DEFAULT);
        oidn_commit_device(device);
        oidn_retain_device(device);

        let filter = oidn_new_filter(device, b"RT\0" as *const _ as _);

        if let Some(alb) = albedo {
            oidn_set_shared_filter_image(
                filter,
                b"albedo\0" as *const _ as _,
                alb.as_ptr() as *mut _,
                OIDNFORMAT_OIDN_FORMAT_FLOAT3,
                dimensions.0 as _,
                dimensions.1 as _,
                0,
                0,
                0,
            );

            // No use supplying normal if albedo was
            // not also given.
            if let Some(norm) = normal {
                oidn_set_shared_filter_image(
                    filter,
                    b"normal\0" as *const _ as _,
                    norm.as_ptr() as *mut _,
                    OIDNFORMAT_OIDN_FORMAT_FLOAT3,
                    dimensions.0 as _,
                    dimensions.1 as _,
                    0,
                    0,
                    0,
                );
            }
        }

        let input_ptr = image.as_ptr();
        oidn_set_shared_filter_image(
            filter,
            b"color\0" as *const _ as _,
            input_ptr as *mut _,
            OIDNFORMAT_OIDN_FORMAT_FLOAT3,
            dimensions.0 as _,
            dimensions.1 as _,
            0,
            0,
            0,
        );

        oidn_set_shared_filter_image(
            filter,
            b"output\0" as *const _ as _,
            image.as_mut_ptr() as *mut _,
            OIDNFORMAT_OIDN_FORMAT_FLOAT3,
            dimensions.0 as _,
            dimensions.1 as _,
            0,
            0,
            0,
        );

        oidn_set_filter1b(filter, b"hdr\0" as *const _ as _, false);
        oidn_set_filter1f(filter, b"inputScale\0" as *const _ as _, f32::NAN);
        oidn_set_filter1b(filter, b"srgb\0" as *const _ as _, true);
        oidn_set_filter1b(filter, b"clean_aux\0" as *const _ as _, true);

        oidn_commit_filter(filter);
        oidn_execute_filter(filter);

        oidn_release_filter(filter);
        oidn_release_device(device);
        Ok(())
    }
}

type OIDNNewDeviceFn = unsafe extern "C" fn(type_: OIDNDeviceType) -> OIDNDevice;
type OIDNManageDeviceFn = unsafe extern "C" fn(device: OIDNDevice);
type OIDNNewFilterFn =
    unsafe extern "C" fn(device: OIDNDevice, type_: *const ::std::os::raw::c_char) -> OIDNFilter;
type OIDNManageFilterFn = unsafe extern "C" fn(filter: OIDNFilter);
type OIDNSetSharedFilterImageFn = unsafe extern "C" fn(
    filter: OIDNFilter,
    name: *const ::std::os::raw::c_char,
    ptr: *mut ::std::os::raw::c_void,
    format: OIDNFormat,
    width: usize,
    height: usize,
    byteOffset: usize,
    bytePixelStride: usize,
    byteRowStride: usize,
);
type OIDNSetFilter1b =
    unsafe extern "C" fn(filter: OIDNFilter, name: *const ::std::os::raw::c_char, value: bool);

type OIDNSetFilter1f =
    unsafe extern "C" fn(filter: OIDNFilter, name: *const ::std::os::raw::c_char, value: f32);
