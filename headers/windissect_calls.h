// Windissect exact-build RVA call helpers — DO NOT EDIT.
// Configure set_resolver() with a loader that validates the expected build.
#pragma once
#include <cstddef>
#include <cstdint>
#include <cstdlib>

#if defined(_M_IX86) || defined(__i386__)
#error "Windissect typed direct calls currently require the Windows x64 ABI"
#endif

namespace windissect_calls {
using ResolveRva = const void* (*)(std::uint32_t);
#if defined(_WIN32)
extern "C" __declspec(dllimport) void* __stdcall GetModuleHandleW(const wchar_t*);
[[nodiscard]] inline const void* default_resolver(std::uint32_t rva) {
static constexpr wchar_t module_name[13] = {0x0057, 0x0049, 0x004E, 0x0042, 0x0052, 0x0041, 0x004E, 0x0044, 0x002E, 0x0064, 0x006C, 0x006C, 0x0000};
const auto base = static_cast<const std::uint8_t*>(GetModuleHandleW(module_name));
if (base == nullptr) return nullptr;
const auto pe = *reinterpret_cast<const std::uint32_t*>(base + 0x3c);
const auto live_timestamp = *reinterpret_cast<const std::uint32_t*>(base + pe + 8);
const auto image_size = *reinterpret_cast<const std::uint32_t*>(base + pe + 80);
if (live_timestamp != 4122134267U || rva >= image_size) return nullptr;
return base + rva;
}
inline ResolveRva resolver = default_resolver;
#else
inline ResolveRva resolver = nullptr;
#endif
inline void set_resolver(ResolveRva value) noexcept { resolver = value; }
[[nodiscard]] inline const void* require(std::uint32_t rva) {
if (resolver == nullptr) std::abort();
const void* address = resolver(rva);
if (address == nullptr) std::abort();
return address;
}

} // namespace windissect_calls
