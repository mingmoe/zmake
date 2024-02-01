local project = {}

function project.setup(arg)
    -- make command.json
    vim.fn.system({
        "ln",
        "-f",
        "-s",
        "build/compile_commands.json",
        "compile_commands.json"
    })

    -- configure
    require("lspconfig").clangd.setup({
        capabilities = arg.capabilities,
        filetypes =  { "cpp","c","h","hpp","cppm"},
    })
end

return project

