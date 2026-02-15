# Creating a Source

With Rust and the `aidoku` cli tool installed, you are ready to create an Aidoku source.

To create a new project, you can use `aidoku` in a similar fashion as `cargo` to initialize the
required files and directory structure:

```sh
aidoku init source_dir
```

You will then be prompted to enter the following parameters:
- Source name: the name displayed for the installed source.
- Source URL: a link to whatever website you are creating a source for. If the website has multiple
  or alternative domains, you can configure more by editing the `source.json` file later.
- Languages: if the website language(s) are shown in the list, you can select them using the arrow
  keys and space bar, then press enter to confirm. Otherwise, select the "other" option and you will
  be prompted to enter any additional (ISO 639) language codes, separated by spaces.
- Content rating: if the website content is largely 18+, select "Primarily NSFW content" (or, if you
  open the website and can reasonably expect to see NSFW content). If the website contains no NSFW
  content, select safe. This helps users filter sources they to install or show.

These parameters are used to populate an initial `source.json` file that you can edit to provide
additional functionality for your source. We will explain this in the next section.

## Packaging the Source

To compile the Rust program and package the resource files together in an `aix` file, you can run
the following command:

```sh
aidoku package
```

If the build succeeds, you will have a resulting `package.aix` file in your current directory that
can be installed in Aidoku. We will explain in a later chapter the best methods for installing and
iterating through development builds.
