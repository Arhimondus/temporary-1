run "deno run --no-remote %name.js"

--[[
	import { readLines } from "./std/io/bufio.ts";
	const { value: input } = await readLines(Deno.stdin).next();
	console.log(input);
--]]