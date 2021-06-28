#if defined(__arm64__)

// Newer version of Hypervisor.Framework (the ones with ARM support) includes more convenient `Hypervisor.h`
#include "Hypervisor/Hypervisor.h"

#elif defined(__x86_64__)

#include "Hypervisor/hv.h"
#include "Hypervisor/hv_vmx.h"

#endif
