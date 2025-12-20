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

---@param path string
---@return boolean
local function is_ignored(path) end

---@class Git
local git = {
  is_ignored = is_ignored,
}

---@class FancyTree
---@field is_unix boolean
---@field os string
---@field git Git|nil
fancytree = {
  path = path,
}
