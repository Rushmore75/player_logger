local function getSensors()
    local proxy = {}
    local component = require("component")
    local scan_index = 0
    -- loop thru all components and find entity detectors 
    for id, name in component.list() do
		if (name == "os_entdetector") then
			proxy[scan_index] = component.proxy(id)
			scan_index = scan_index + 1
		end
    end

    return proxy, scan_index
end

local function scan()
	local proxy, count = getSensors()
	local internet = require("internet")

	print("Scanning from " .. count .. " locations")
	while (true) do
		for index = 0, count-1 do
			local found = proxy[index].scanPlayers(64)
			for _, player in ipairs(found) do
		    	internet.request("https://friendlyfire.oliveratkinson.net/api/logplayer", tostring(player.name), {}, "POST")
				print(index .. " found: " .. player.name)
			end
		end
		os.sleep(15) -- 10 crashes intermitantaly
	end
end

-- start
scan()
