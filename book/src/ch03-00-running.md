# Running

After compiling your source with `aidoku package` (or `aidoku pkg`), which generates the
`package.aix` file, you have a few options for running.

## Xcode

If you are running macOS and have Xcode installed, a convenient method may be using the iOS
simulator for testing sources, but it requires a more complex setup. Since Aidoku is open source,
you can clone the [GitHub repo]() and install it to a simulator. Then, simply drag and drop the
"package.aix" file onto the simulator. If Aidoku is attached to the debugger in Xcode, logs will be
displayed in the Xcode log viewer.

## Physical Device

Otherwise, you can choose to either transfer the aix file to the device manually (e.g. via AirDrop),
or serve a source list with `aidoku serve package.aix`. The latter method will give you a
`localhost` URL that you can add as a source list in Aidoku on any device connected to the same
network. Your source can then be installed from the "add source" view inside Aidoku, and will show
as an update if you increment the source version number and recompile. Note that you may need to
pull down to refresh the source lists on the browse tab.
