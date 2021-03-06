use std::mem;

pub use self::H5O_mcdt_search_ret_t::*;
pub use self::H5O_type_t::*;

use crate::internal_prelude::*;

pub const H5O_COPY_SHALLOW_HIERARCHY_FLAG: c_uint = 0x0001;
pub const H5O_COPY_EXPAND_SOFT_LINK_FLAG: c_uint = 0x0002;
pub const H5O_COPY_EXPAND_EXT_LINK_FLAG: c_uint = 0x0004;
pub const H5O_COPY_EXPAND_REFERENCE_FLAG: c_uint = 0x0008;
pub const H5O_COPY_WITHOUT_ATTR_FLAG: c_uint = 0x0010;
pub const H5O_COPY_PRESERVE_NULL_FLAG: c_uint = 0x0020;
#[cfg(not(hdf5_1_8_9))]
pub const H5O_COPY_ALL: c_uint = 0x003F;
#[cfg(hdf5_1_8_9)]
pub const H5O_COPY_MERGE_COMMITTED_DTYPE_FLAG: c_uint = 0x0040;
#[cfg(hdf5_1_8_9)]
pub const H5O_COPY_ALL: c_uint = 0x007F;

pub const H5O_SHMESG_NONE_FLAG: c_uint = 0x0000;
pub const H5O_SHMESG_SDSPACE_FLAG: c_uint = 1 << 0x0001;
pub const H5O_SHMESG_DTYPE_FLAG: c_uint = 1 << 0x0003;
pub const H5O_SHMESG_FILL_FLAG: c_uint = 1 << 0x0005;
pub const H5O_SHMESG_PLINE_FLAG: c_uint = 1 << 0x000b;
pub const H5O_SHMESG_ATTR_FLAG: c_uint = 1 << 0x000c;
pub const H5O_SHMESG_ALL_FLAG: c_uint = H5O_SHMESG_SDSPACE_FLAG
    | H5O_SHMESG_DTYPE_FLAG
    | H5O_SHMESG_FILL_FLAG
    | H5O_SHMESG_PLINE_FLAG
    | H5O_SHMESG_ATTR_FLAG;

pub const H5O_HDR_CHUNK0_SIZE: c_uint = 0x03;
pub const H5O_HDR_ATTR_CRT_ORDER_TRACKED: c_uint = 0x04;
pub const H5O_HDR_ATTR_CRT_ORDER_INDEXED: c_uint = 0x08;
pub const H5O_HDR_ATTR_STORE_PHASE_CHANGE: c_uint = 0x10;
pub const H5O_HDR_STORE_TIMES: c_uint = 0x20;
pub const H5O_HDR_ALL_FLAGS: c_uint = H5O_HDR_CHUNK0_SIZE
    | H5O_HDR_ATTR_CRT_ORDER_TRACKED
    | H5O_HDR_ATTR_CRT_ORDER_INDEXED
    | H5O_HDR_ATTR_STORE_PHASE_CHANGE
    | H5O_HDR_STORE_TIMES;

pub const H5O_SHMESG_MAX_NINDEXES: c_uint = 8;
pub const H5O_SHMESG_MAX_LIST_SIZE: c_uint = 5000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum H5O_type_t {
    H5O_TYPE_UNKNOWN = -1,
    H5O_TYPE_GROUP = 0,
    H5O_TYPE_DATASET = 1,
    H5O_TYPE_NAMED_DATATYPE = 2,
    H5O_TYPE_NTYPES = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H5O_hdr_info_t {
    pub version: c_uint,
    pub nmesgs: c_uint,
    pub nchunks: c_uint,
    pub flags: c_uint,
    pub space: H5O_hdr_info_t__space,
    pub mesg: H5O_hdr_info_t__mesg,
}

impl Default for H5O_hdr_info_t {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H5O_hdr_info_t__space {
    pub total: hsize_t,
    pub meta: hsize_t,
    pub mesg: hsize_t,
    pub free: hsize_t,
}

impl Default for H5O_hdr_info_t__space {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H5O_hdr_info_t__mesg {
    pub present: uint64_t,
    pub shared: uint64_t,
}

impl Default for H5O_hdr_info_t__mesg {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H5O_info_t {
    pub fileno: c_ulong,
    pub addr: haddr_t,
    pub type_: H5O_type_t,
    pub rc: c_uint,
    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,
    pub btime: time_t,
    pub num_attrs: hsize_t,
    pub hdr: H5O_hdr_info_t,
    pub meta_size: H5O_info_t__meta_size,
}

impl Default for H5O_info_t {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct H5O_info_t__meta_size {
    pub obj: H5_ih_info_t,
    pub attr: H5_ih_info_t,
}

impl Default for H5O_info_t__meta_size {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

pub type H5O_msg_crt_idx_t = uint32_t;

pub type H5O_iterate_t = Option<
    extern "C" fn(
        obj: hid_t,
        name: *const c_char,
        info: *const H5O_info_t,
        op_data: *mut c_void,
    ) -> herr_t,
>;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum H5O_mcdt_search_ret_t {
    H5O_MCDT_SEARCH_ERROR = -1,
    H5O_MCDT_SEARCH_CONT = 0,
    H5O_MCDT_SEARCH_STOP = 1,
}

#[cfg(hdf5_1_8_9)]
pub type H5O_mcdt_search_cb_t =
    Option<extern "C" fn(op_data: *mut c_void) -> H5O_mcdt_search_ret_t>;

#[cfg(not(hdf5_1_10_3))]
extern "C" {
    pub fn H5Oget_info(loc_id: hid_t, oinfo: *mut H5O_info_t) -> herr_t;
    pub fn H5Oget_info_by_name(
        loc_id: hid_t, name: *const c_char, oinfo: *mut H5O_info_t, lapl_id: hid_t,
    ) -> herr_t;
    pub fn H5Oget_info_by_idx(
        loc_id: hid_t, group_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
        n: hsize_t, oinfo: *mut H5O_info_t, lapl_id: hid_t,
    ) -> herr_t;
    pub fn H5Ovisit(
        obj_id: hid_t, idx_type: H5_index_t, order: H5_iter_order_t, op: H5O_iterate_t,
        op_data: *mut c_void,
    ) -> herr_t;
    pub fn H5Ovisit_by_name(
        loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
        op: H5O_iterate_t, op_data: *mut c_void, lapl_id: hid_t,
    ) -> herr_t;
}

extern "C" {
    pub fn H5Oopen(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> hid_t;
    pub fn H5Oopen_by_addr(loc_id: hid_t, addr: haddr_t) -> hid_t;
    pub fn H5Oopen_by_idx(
        loc_id: hid_t, group_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
        n: hsize_t, lapl_id: hid_t,
    ) -> hid_t;
    pub fn H5Olink(
        obj_id: hid_t, new_loc_id: hid_t, new_name: *const c_char, lcpl_id: hid_t, lapl_id: hid_t,
    ) -> herr_t;
    pub fn H5Oincr_refcount(object_id: hid_t) -> herr_t;
    pub fn H5Odecr_refcount(object_id: hid_t) -> herr_t;
    pub fn H5Ocopy(
        src_loc_id: hid_t, src_name: *const c_char, dst_loc_id: hid_t, dst_name: *const c_char,
        ocpypl_id: hid_t, lcpl_id: hid_t,
    ) -> herr_t;
    pub fn H5Oset_comment(obj_id: hid_t, comment: *const c_char) -> herr_t;
    pub fn H5Oset_comment_by_name(
        loc_id: hid_t, name: *const c_char, comment: *const c_char, lapl_id: hid_t,
    ) -> herr_t;
    pub fn H5Oget_comment(obj_id: hid_t, comment: *mut c_char, bufsize: size_t) -> ssize_t;
    pub fn H5Oget_comment_by_name(
        loc_id: hid_t, name: *const c_char, comment: *mut c_char, bufsize: size_t, lapl_id: hid_t,
    ) -> ssize_t;
    pub fn H5Oclose(object_id: hid_t) -> herr_t;
}

#[cfg(hdf5_1_8_5)]
use crate::h5::htri_t;

#[cfg(hdf5_1_8_5)]
extern "C" {
    pub fn H5Oexists_by_name(loc_id: hid_t, name: *const c_char, lapl_id: hid_t) -> htri_t;
}

#[cfg(hdf5_1_10_0)]
extern "C" {
    pub fn H5Odisable_mdc_flushes(object_id: hid_t) -> herr_t;
    pub fn H5Oenable_mdc_flushes(object_id: hid_t) -> herr_t;
    pub fn H5Oare_mdc_flushes_disabled(object_id: hid_t, are_disabled: *mut hbool_t) -> herr_t;
    pub fn H5Oflush(obj_id: hid_t) -> herr_t;
    pub fn H5Orefresh(oid: hid_t) -> herr_t;
}

#[cfg(hdf5_1_10_3)]
mod hdf5_1_10_3 {
    use super::*;

    extern "C" {
        pub fn H5Oget_info2(loc_id: hid_t, oinfo: *mut H5O_info_t, fields: c_uint) -> herr_t;
        pub fn H5Oget_info_by_name2(
            loc_id: hid_t, name: *const c_char, oinfo: *mut H5O_info_t, fields: c_uint,
            lapl_id: hid_t,
        ) -> herr_t;
        pub fn H5Oget_info_by_idx2(
            loc_id: hid_t, group_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
            n: hsize_t, oinfo: *mut H5O_info_t, fields: c_uint, lapl_id: hid_t,
        ) -> herr_t;
        pub fn H5Ovisit2(
            obj_id: hid_t, idx_type: H5_index_t, order: H5_iter_order_t, op: H5O_iterate_t,
            op_data: *mut c_void, fields: c_uint,
        ) -> herr_t;
        pub fn H5Ovisit_by_name2(
            loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
            op: H5O_iterate_t, op_data: *mut c_void, fields: c_uint, lapl_id: hid_t,
        ) -> herr_t;
        #[deprecated(note = "deprecated in HDF5 1.10.3, use H5Oget_info2()")]
        pub fn H5Oget_info1(loc_id: hid_t, oinfo: *mut H5O_info_t) -> herr_t;
        #[deprecated(note = "deprecated in HDF5 1.10.3, use H5Oget_info_by_name2()")]
        pub fn H5Oget_info_by_name1(
            loc_id: hid_t, name: *const c_char, oinfo: *mut H5O_info_t, lapl_id: hid_t,
        ) -> herr_t;
        #[deprecated(note = "deprecated in HDF5 1.10.3, use H5Oget_info_by_idx2()")]
        pub fn H5Oget_info_by_idx1(
            loc_id: hid_t, group_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
            n: hsize_t, oinfo: *mut H5O_info_t, lapl_id: hid_t,
        ) -> herr_t;
        #[deprecated(note = "deprecated in HDF5 1.10.3, use H5Ovisit2()")]
        pub fn H5Ovisit1(
            obj_id: hid_t, idx_type: H5_index_t, order: H5_iter_order_t, op: H5O_iterate_t,
            op_data: *mut c_void,
        ) -> herr_t;
        #[deprecated(note = "deprecated in HDF5 1.10.3, use H5Ovisit_by_name2()")]
        pub fn H5Ovisit_by_name1(
            loc_id: hid_t, obj_name: *const c_char, idx_type: H5_index_t, order: H5_iter_order_t,
            op: H5O_iterate_t, op_data: *mut c_void, lapl_id: hid_t,
        ) -> herr_t;
    }

    pub use self::{
        H5Oget_info1 as H5Oget_info, H5Oget_info_by_idx1 as H5Oget_info_by_idx,
        H5Oget_info_by_name1 as H5Oget_info_by_name, H5Ovisit1 as H5Ovisit,
        H5Ovisit_by_name1 as H5Ovisit_by_name,
    };
}

#[cfg(hdf5_1_10_3)]
pub use self::hdf5_1_10_3::*;
