settings.set("imgtool.overwrite",true)
settings.set("mksea.overwrite",true)

fs.delete("output/*")

local log = fs.open("log.txt",'wb')

function log_factory(target)
    return function(text) log.write("["..target.."] "..text.."\n") end
end

function deepcopy(orig)
    local orig_type = type(orig)
    local copy
    if orig_type == 'table' then
        copy = {}
        for orig_key, orig_value in next, orig, nil do
            copy[deepcopy(orig_key)] = deepcopy(orig_value)
        end
        setmetatable(copy, deepcopy(getmetatable(orig)))
    else -- number, string, boolean, etc
        copy = orig
    end
    return copy
end

local print = log_factory("BUILD")

function runfile(text,v)
    local split = {}
    local spat, epat, buf, quoted = [=[^(['"])]=], [=[(['"])$]=]
    for str in text:gmatch("%S+") do
        local squoted = str:match(spat)
        local equoted = str:match(epat)
        local escaped = str:match([=[(\*)['"]$]=])
        if squoted and not quoted and not equoted then
            buf, quoted = str, squoted
        elseif buf and equoted == quoted and #escaped % 2 == 0 then
            str, buf, quoted = buf .. ' ' .. str, nil, nil
        elseif buf then
            buf = buf .. ' ' .. str
        end
        if not buf then table.insert(split,(str:gsub(spat,""):gsub(epat,""))) end
    end
    if buf then print("Missing matching quote for "..buf) end
    local prg = table.remove(split,1)
    print("os.run() "..prg.." "..textutils.serialise(split,{compact=true}))
    local env = {shell = shell, multishell = multishell, require = require,package = package}
    env.print = log_factory(v)
    os.run(env,prg,table.unpack(split))
end

for _,v in ipairs(fs.list("input")) do
    fs.makeDir("output/"..v)
    runfile("programs/imgtool.lua build compress output/"..v.."/orangebox.vgz input/"..v.."/OS input/"..v.."/Apps input/"..v.."/startup.lua",v)
    runfile("programs/imgtool.lua build nocompress output/"..v.."/yellowbox.vfs input/"..v.."/OS input/"..v.."/Apps input/"..v.."/startup.lua",v)
    runfile("programs/mksea.lua output/"..v.."/orangebox.vgz output/"..v..'/installer.lua true ""',v)
end

log.close()