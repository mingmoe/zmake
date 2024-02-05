module;
#include <string>
#include "zmake-config.hpp"
export module Moe.ZMake;
export import :Engine;

#ifdef MOE_ORG_ZMAKE_ENABLE_QUICKJS
export import :QuickJS;
#endif

#ifdef MOE_ORG_ZMAKE_ENABLE_V8
export import :V8;
#endif

namespace MoeOrg::ZMake{
    
    export class ZMakeInstance{
        std::string getVersion();
    };

}

