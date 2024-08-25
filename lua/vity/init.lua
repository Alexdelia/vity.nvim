local M = {}

function M.colorscheme()
	vim.cmd("hi clear")
    vim.g.colors_name = "vity"

	require("vity.highlight").setup()
end

function M.load()
	vim.api.nvim_command("colorscheme vity")
end

return M
