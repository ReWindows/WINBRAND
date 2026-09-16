// Flat C exports observed in WINBRAND.dll. Unknown ABIs are intentionally not declared.
// Validated dialects: ISO C17/C23/C26 draft and Windissect C20 (C ABI under C++20).
#pragma once
#ifndef WINBRAND_C_H
#define WINBRAND_C_H
#if defined(__cplusplus)
#  if __cplusplus < 202002L
#    define WINDISSECT_C20_PROFILE 0
#  else
#    define WINDISSECT_C20_PROFILE 1
#  endif
extern "C" {
#else
#  if defined(__STDC_VERSION__) && __STDC_VERSION__ < 201710L
#    error "Windissect C output requires C17 or newer"
#  endif
#  define WINDISSECT_C20_PROFILE 0
#  define WINDISSECT_C_STANDARD __STDC_VERSION__
#endif

// Export: BrandingLoadStringForEdition (ABI unverified)
// Export: BrandingLoadNeutralStringForEdition (ABI unverified)
// Export: GetHinstanceByNameSpace (ABI unverified)
// Export: BrandingFormatStringForEdition (ABI unverified)
// Export: BrandingLoadString (ABI unverified)
// Export: BrandingFormatString (ABI unverified)
// Export: BrandingLoadImage (ABI unverified)
// Export: BrandingLoadBitmap (ABI unverified)
// Export: BrandingLoadCursor (ABI unverified)
// Export: BrandingLoadIcon (ABI unverified)
// Export: EulaFreeBuffer (ABI unverified)
// Export: GetEULAFile (ABI unverified)
// Export: GetEULAFileEx (ABI unverified)
// Export: GetEULAInCurrentUILanguage (ABI unverified)
// Export: GetInstalledEULAPath (ABI unverified)
// Export: InstallEULA (ABI unverified)

#ifdef __cplusplus
} // extern "C"
#endif
#endif // WINBRAND_C_H
