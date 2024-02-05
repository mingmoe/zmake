module;
#include <string_view>
#include <string>
#include <tl/expected.hpp>
#include <optional>
export module Moe.ZMake:Engine;

namespace MoeOrg::ZMake{
    
    export class EngineInterface{
        public:
        EngineInterface operator=(const EngineInterface&) = delete;
        EngineInterface operator=(EngineInterface&&) = delete;
        EngineInterface(const EngineInterface&) = delete;
        EngineInterface(EngineInterface&&) = delete;
        EngineInterface() = default;
        virtual ~EngineInterface() = default;

        /**
         * simply descript for the engine.
         * e.g. QuickJS with ts support.
         **/
        virtual std::string getEngineDescription();

        /**
         * Eval a part code
         **/
        virtual tl::expected<std::string,std::string> eval(std::string_view code,std::string_view source);
    };

}

