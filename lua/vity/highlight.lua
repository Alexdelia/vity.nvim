local M = {}

function M.setup()
	---@param group string
	---@param opt table
	local function hl(group, opt)
		vim.api.nvim_set_hl(0, group, opt)
	end

	hl("Comment", {fg = "#ff0000"})
end

return M
