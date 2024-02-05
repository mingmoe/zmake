local project = {}

project.capabilities = nil

function project.update()
    -- generate compile_commands.json
     vim.fn.system({
        "xmake",
        "project",
        "-k",
        "compile_commands",
        "."
    })
end

function project.setup(arg)
    -- set up for clang
    project.capabilities = arg.capabilities
    project.update()

    vim.fn.system({
        "xmake",
        "f",
        "--toolchain=clang",
        "-c"
    })
    -- configure
    require("lspconfig").clangd.setup({
        capabilities = project.capabilities,
        filetypes =  { "cpp","c","h","hpp","cppm"},
    })
end

return project

