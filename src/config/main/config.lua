local ok_files = { ".gitattributes", ".gitignore" }

return {
  ---@type "auto"|"on"|"ansi"|"off"|nil
  color = "auto",
  ---@param filepath string Path to the file relative to the starting directory
  ---@param attributes FileAttributes
  ---@param default boolean
  ---@return boolean
  skip = function(filepath, attributes, default)
    local filename = fancytree.path.filename(filepath)
    for _, ok_filename in ipairs(ok_files) do
      if filename == ok_filename then
        return false
      end
    end
    -- The default is to hide dotfiles on Unix and files with the hidden attribute on
    -- Windows.
    return default
  end,
}
