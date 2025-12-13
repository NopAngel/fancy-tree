---@meta

---@param path string
---@return string|nil
---@nodiscard
local function filename(path) end

---@param glob string
---@param path string
---@return boolean
---@nodiscard
local function glob_matches(glob, path) end

---@class Path
local path = {
  filename = filename,
  glob_matches = glob_matches,
}

---@class FancyTree
---@field is_unix boolean
---@field os string
fancytree = {
  path = path,
}
