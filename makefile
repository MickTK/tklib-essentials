build: release move

release:
	cargo build --target x86_64-pc-windows-gnu -p tklib --release

move:
	cp target/x86_64-pc-windows-gnu/release/tklib.dll Essentials/Plugins/TKLib/bin/

target:
	rustup target add x86_64-pc-windows-gnu
