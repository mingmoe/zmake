module;
#include <string>
export module Moe.ZMake;
export import :Engine;

namespace MoeOrganisation::ZMake{
    
    export class ZMakeInstance{
        std::string getVersion();
    };

}

