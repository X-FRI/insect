set_project("bonding")
set_version("0.1.3")
set_xmakever("2.8.1")

add_rules("mode.debug", "mode.release")
add_rules("plugin.compile_commands.autoupdate", {outputdir = "."})

add_requires("linenoise")

target("sinbuger")
    set_kind("binary")
    set_languages("c++17")

    add_files("src/*.cpp")
    add_packages("linenoise")