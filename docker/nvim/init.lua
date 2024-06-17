
-- specify the path for lazyvim package manager
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"

-- if not localized locally, then clone it
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable", -- latest stable release
    lazypath,
  })
end

-- pre-pend the path as a vim option
vim.opt.rtp:prepend(lazypath)

-- load other directives from the options
require("vim-options")

-- load plugins from a file
require("lazy").setup("plugins")

-- Load this color scheme
vim.cmd.colorscheme 'nordic'

