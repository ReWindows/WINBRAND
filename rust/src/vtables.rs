//! Evidence-labelled vftable metadata and raw slot access.
//! Slot calls remain the consumer's responsibility until a signature is unique.

use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct VTableInfo { pub name: &'static str, pub rva: u32, pub first_slot: usize, pub slot_count: usize, pub confidence: &'static str }

#[derive(Clone, Copy, Debug)]
pub struct VTableSlot { pub table_rva: u32, pub slot: u32, pub byte_offset: u32, pub target_rva: u32, pub target_id: Option<&'static str>, pub target_name: Option<&'static str>, pub ambiguous: bool, pub this_adjustment: Option<i32> }

pub static VTABLES: &[VTableInfo] = &[
    VTableInfo { name: "const BaseBrdResourceIdMap::`vftable'", rva: 0x29000, first_slot: 0, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const ResourceIdMap::`vftable'", rva: 0x29010, first_slot: 2, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const type_info::`vftable'", rva: 0x29020, first_slot: 4, slot_count: 1, confidence: "likely" },
    VTableInfo { name: "const wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::`vftable'", rva: 0x290D0, first_slot: 5, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const wistd::__function::__base<bool __cdecl (void *, uint64_t, void *, uint64_t, unsigned int)>::`vftable'", rva: 0x29100, first_slot: 10, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const wil::ResultException::`vftable'", rva: 0x29130, first_slot: 15, slot_count: 2, confidence: "likely" },
    VTableInfo { name: "const ShellBrdResourceIdMap::`vftable'", rva: 0x29140, first_slot: 17, slot_count: 2, confidence: "likely" },
];

pub static VTABLE_SLOTS: &[VTableSlot] = &[
    VTableSlot { table_rva: 0x29000, slot: 0, byte_offset: 0, target_rva: 0x23910, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29000, slot: 1, byte_offset: 8, target_rva: 0xF8E0, target_id: Some("?UpdateResourceId@BaseBrdResourceIdMap@@UEAAXKIPEAI@Z"), target_name: Some("public: virtual void __cdecl BaseBrdResourceIdMap::UpdateResourceId(unsigned long, unsigned int, unsigned int *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x29010, slot: 0, byte_offset: 0, target_rva: 0x23910, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29010, slot: 1, byte_offset: 8, target_rva: 0x1E230, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29020, slot: 0, byte_offset: 0, target_rva: 0x1E250, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x290D0, slot: 0, byte_offset: 0, target_rva: 0x1F730, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x290D0, slot: 1, byte_offset: 8, target_rva: 0x222E0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x290D0, slot: 2, byte_offset: 16, target_rva: 0x222E0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x290D0, slot: 3, byte_offset: 24, target_rva: 0x223C0, target_id: Some("?destroy@?$__func@V<lambda_8db0ce862824541f40dfb767113f1e28>@@$$A6A_NPEAX_K01I@Z@__function@wistd@@UEAAXXZ"), target_name: Some("public: virtual void __cdecl wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::destroy(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x290D0, slot: 4, byte_offset: 32, target_rva: 0x1F660, target_id: Some("??R?$__func@V<lambda_8db0ce862824541f40dfb767113f1e28>@@$$A6A_NPEAX_K01I@Z@__function@wistd@@UEAA_N$$QEAPEAX$$QEA_K01$$QEAI@Z"), target_name: Some("public: virtual bool __cdecl wistd::__function::__func<class <lambda_8db0ce862824541f40dfb767113f1e28>, bool __cdecl (void *, uint64_t, class <lambda_8db0ce862824541f40dfb767113f1e28>, void *, unsigned int)>::operator()(void * &&, uint64_t &&, void * &&, uint64_t &&, unsigned int &&)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x29100, slot: 0, byte_offset: 0, target_rva: 0x1F730, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29100, slot: 1, byte_offset: 8, target_rva: 0x1E230, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29100, slot: 2, byte_offset: 16, target_rva: 0x1E230, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29100, slot: 3, byte_offset: 24, target_rva: 0x1E230, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29100, slot: 4, byte_offset: 32, target_rva: 0x1E230, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29130, slot: 0, byte_offset: 0, target_rva: 0x1F760, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29130, slot: 1, byte_offset: 8, target_rva: 0x22550, target_id: Some("?what@ResultException@wil@@UEBAPEBDXZ"), target_name: Some("public: virtual char const * __cdecl wil::ResultException::what(void) const"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x29140, slot: 0, byte_offset: 0, target_rva: 0x23910, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x29140, slot: 1, byte_offset: 8, target_rva: 0x23950, target_id: Some("?UpdateResourceId@ShellBrdResourceIdMap@@UEAAXKIPEAI@Z"), target_name: Some("public: virtual void __cdecl ShellBrdResourceIdMap::UpdateResourceId(unsigned long, unsigned int, unsigned int *)"), ambiguous: false, this_adjustment: None },
];

/// Reads a raw function pointer from an object's primary vftable.
///
/// # Safety
/// `object` must point to a live object with a readable primary vftable,
/// and `slot` must be valid for that concrete object. This function does
/// not invent or transmute a callable signature.
pub unsafe fn raw_object_slot(object: *const c_void, slot: usize) -> Option<*const ()> {
if object.is_null() { return None; }
let table = unsafe { *(object.cast::<*const *const ()>()) };
if table.is_null() { return None; }
let target = unsafe { *table.add(slot) };
(!target.is_null()).then_some(target)
}
