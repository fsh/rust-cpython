use libc::{c_char, c_int, c_void};

use crate::object::*;
use crate::pyport::Py_ssize_t;

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub static mut PyDict_Type: PyTypeObject;
    pub static mut PyDictIterKey_Type: PyTypeObject;
    pub static mut PyDictIterValue_Type: PyTypeObject;
    pub static mut PyDictIterItem_Type: PyTypeObject;
    pub static mut PyDictKeys_Type: PyTypeObject;
    pub static mut PyDictItems_Type: PyTypeObject;
    pub static mut PyDictValues_Type: PyTypeObject;
    #[cfg(Py_3_8)]
    pub static mut PyDictRevIterKey_Type: PyTypeObject;
    #[cfg(Py_3_8)]
    pub static mut PyDictRevIterValue_Type: PyTypeObject;
    #[cfg(Py_3_8)]
    pub static mut PyDictRevIterItem_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyDict_Check(op: *mut PyObject) -> c_int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_DICT_SUBCLASS)
}

#[inline(always)]
pub unsafe fn PyDict_CheckExact(op: *mut PyObject) -> c_int {
    (Py_TYPE(op) == &mut PyDict_Type) as c_int
}

#[inline(always)]
pub unsafe fn PyDictKeys_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyDictKeys_Type)
}

#[inline(always)]
pub unsafe fn PyDictItems_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyDictItems_Type)
}

#[inline(always)]
pub unsafe fn PyDictValues_Check(op: *mut PyObject) -> c_int {
    PyObject_TypeCheck(op, &mut PyDictValues_Type)
}

#[inline(always)]
pub unsafe fn PyDictViewSet_Check(op: *mut PyObject) -> c_int {
    (PyDictKeys_Check(op) != 0 || PyDictItems_Check(op) != 0) as c_int
}

#[cfg_attr(windows, link(name = "pythonXY"))]
extern "C" {
    pub fn PyDict_New() -> *mut PyObject;
    pub fn PyDict_GetItem(mp: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_GetItemWithError(mp: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_SetItem(mp: *mut PyObject, key: *mut PyObject, item: *mut PyObject) -> c_int;
    pub fn PyDict_DelItem(mp: *mut PyObject, key: *mut PyObject) -> c_int;
    pub fn PyDict_Clear(mp: *mut PyObject) -> ();
    pub fn PyDict_Next(
        mp: *mut PyObject,
        pos: *mut Py_ssize_t,
        key: *mut *mut PyObject,
        value: *mut *mut PyObject,
    ) -> c_int;
    pub fn PyDict_Keys(mp: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_Values(mp: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_Items(mp: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_Size(mp: *mut PyObject) -> Py_ssize_t;
    pub fn PyDict_Copy(mp: *mut PyObject) -> *mut PyObject;
    pub fn PyDict_Contains(mp: *mut PyObject, key: *mut PyObject) -> c_int;
    pub fn PyDict_Update(mp: *mut PyObject, other: *mut PyObject) -> c_int;
    pub fn PyDict_Merge(mp: *mut PyObject, other: *mut PyObject, _override: c_int) -> c_int;
    pub fn PyDict_MergeFromSeq2(d: *mut PyObject, seq2: *mut PyObject, _override: c_int) -> c_int;
    pub fn PyDict_GetItemString(dp: *mut PyObject, key: *const c_char) -> *mut PyObject;
    pub fn PyDict_SetItemString(
        dp: *mut PyObject,
        key: *const c_char,
        item: *mut PyObject,
    ) -> c_int;
    pub fn PyDict_DelItemString(dp: *mut PyObject, key: *const c_char) -> c_int;
    #[cfg(Py_3_10)]
    pub fn PyObject_GenericGetDict(obj: *mut PyObject, context: *mut c_void) -> *mut PyObject;
}
