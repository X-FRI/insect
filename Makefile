install:
	xmake f -m release && xmake build
	sudo cp build/linux/x86_64/release/sinbuger /usr/local/bin

uninstall:
	sudo rm /usr/local/bin/sinbuger

build.release:
	xmake f -m release && xmake build

build.debug:
	xmake f -m release && xmake build

debug:
	xmake f -m debug && xmake build && xmake run -d

fmt:
	find src -iname '*.h' -o -iname '*.cpp' -o -iname '*.hpp' | xargs clang-format -i
