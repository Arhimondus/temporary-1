run "dotnet publish -c Release"
run "bin/Release/net5.0/win-x64/publish/%name.exe"

--[[
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net5.0</TargetFramework>
	<PublishSingleFile>true</PublishSingleFile>
	<SelfContained>false</SelfContained>
	<IncludeAllContentForSelfExtract>true</IncludeAllContentForSelfExtract>
	<RuntimeIdentifier>win-x64</RuntimeIdentifier>
  </PropertyGroup>
</Project>
--]]