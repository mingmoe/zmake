
add_rules("mode.debug", "mode.release","mode.check")
set_languages("cxx23")

option("quickjs")
    set_default(false)
    set_showmenu(true)
    set_description("Enable QuickJs Engine")

option("v8")
    set_default(true)
    set_showmenu(true)
    set_description("Enable Google V8 Engine")


target("zmake-core")
    set_kind("static")
    set_toolchains("clang")
    add_files("*.cppm")
    add_options("quickjs")
    add_options("v8")

    add_includedirs(
        "../library/spdlog/include",
        "../library/expected/include",
        "../library/fmtlib/include",
        {public=true})

    set_configdir("$(buildir)/config-include")
    add_includedirs("$(buildir)/config-include")

    if has_config("quickjs") then
        set_configvar("MOE_ORG_ZMAKE_ENABLE_QUICKJS", "true")
        add_headerfiles("library/quickjspp/quickjspp.hpp")
        add_files("quickjs/*.cppm")
        add_deps("quickjs")
    end

    if has_config("v8") then
        set_configvar("MOE_ORG_ZMAKE_ENABLE_V8","true")
        add_files("v8/*.cppm")
        add_linkdirs(get_config("v8_obj_path"))
        add_includedirs(get_config("v8_include_path"))
        add_links("v8_monolith","v8_libbase","v8_libplatform")
        add_defines("V8_ENABLE_SANDBOX=ON","V8_COMPRESS_POINTERS=ON","_ITERATOR_DEBUG_LEVEL=0")
    end

    add_configfiles("config.hpp.in",{filename = "zmake-config.hpp"})

