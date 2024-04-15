{
	init = function ()
		cmd [[
			call vcvarsx86_amd64.bat
			cl /TP /MT /EHsc /GL /O2 /W3 /Za /std:c++latest %name.cpp
		]]
	end,
	run = function()
		run "%name.exe"
	end,
}