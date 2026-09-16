#pragma once
#include "../windissect_forwards.h"

// Reconstructed from WINBRAND.dll by Windissect. 4 member(s).
class BaseBrdResourceIdMap {
public:
    // Category: Method | Source: PDB Internal
    // Symbol: ?UpdateResourceId@BaseBrdResourceIdMap@@UEAAXKIPEAI@Z
    virtual void UpdateResourceId(unsigned long, unsigned int, unsigned int *);
private:
    // Category: Method | Source: PDB Internal
    // Symbol: ?UpdateResourceIdOptional@BaseBrdResourceIdMap@@AEAAXKIPEAI@Z
    void UpdateResourceIdOptional(unsigned long, unsigned int, unsigned int *);
};
