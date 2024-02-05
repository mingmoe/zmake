
target("quickjs")
    set_languages("c11")
    set_kind("static")
    set_toolchains("clang")
    add_links("pthread", "m", "dl")
    add_files("quickjs/quickjs.c","quickjs/libbf.c","quickjs/libregexp.c","quickjs/libunicode.c","quickjs/cutils.c",
    "quickjs/quickjs-libc.c")
    add_defines(
        "CONFIG_VERSION=\"" .. "2024-1-13" .. "\"",
        "CONFIG_BIGNUM",
        "_GNU_SOURCE")
    add_cflags("-funsigned-char","-rdynamic","-MMD","-MF","-fwrapv")
    add_includedirs("quickjs/", {public = true})

