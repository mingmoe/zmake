
add_rules("mode.debug", "mode.release")
set_languages("c++23")

target("zmake")
    set_kind("static")
    add_files("*.cppm")

