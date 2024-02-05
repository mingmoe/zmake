
set_languages("cxx23")
set_policy("check.auto_ignore_flags", false)

target("zmake")
    set_kind("binary")
    set_toolchains("clang")

    -- set_toolset("cc","clang++")
    -- set_toolset("cxx","clang++")

    add_files("*.cpp")
    add_cxxflags("-stdlib=libc++",{force=true})

    add_deps("zmake-core")


