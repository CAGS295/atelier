return { 
  'nvim-telescope/telescope.nvim', 
  tag = '0.1.5',
  dependencies = { 'nvim-lua/plenary.nvim' },
  config = function()
    local builtin = require("telescope.builtin")
    vim.keymap.set('n', '<leader>tf', builtin.find_files, {})
    vim.keymap.set('n', '<leader>tg', builtin.live_grep, {})
    vim.keymap.set('n', '<leader>ts', builtin.grep_string,  {})
  end
}

