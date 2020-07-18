#include "rpll_instance_mgr.h"

void RPLLInstanceMgrHooks::DeleteInstanceFromDB(uint32_t instanceId)
{
    RPLLHooks::InstanceDelete(instanceId);
}