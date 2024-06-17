vim.cmd("set expandtab")
vim.cmd("set tabstop=2")
vim.cmd("set softtabstop=2")
vim.cmd("set shiftwidth=2")
vim.g.mapleader = " "

-- TODO: Adjust values after automatic detection of on-monitor, or, on-laptop
-- Configuration for Layout
-- vim.cmd("set lines=60 columns=180")

-- Opens a split window below the current one, and, opens a terminal
vim.cmd("autocmd VimEnter * belowright split | :terminal")

-- Reduce the height of the terminal window
vim.cmd("autocmd VimEnter * horizontal resize -15")

-- Reveals the files-tree in the left
vim.cmd("autocmd VimEnter * Neotree filesystem reveal right")

-- Reduce the width of the file-tree window
vim.cmd("autocmd VimEnter * vertical resize -5")

-- Set line numbers
vim.cmd("set number")
