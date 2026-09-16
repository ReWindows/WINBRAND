#pragma once
#include "../windissect_forwards.h"

// Reconstructed from WINBRAND.dll by Windissect. 3 member(s).
class SErrorConverter {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?C_LR2HR@SErrorConverter@@SAJK@Z
    static long C_LR2HR(unsigned long);
    // Category: Method | Source: PDB Internal
    // Symbol: ?C_NtStatus2HR@SErrorConverter@@SAHJPEAJ@Z
    static int C_NtStatus2HR(long, long *);
    // Category: Method | Source: PDB Internal
    // Symbol: ?C_Win32Error2HR@SErrorConverter@@SAHKPEAJ@Z
    static int C_Win32Error2HR(unsigned long, long *);
};
