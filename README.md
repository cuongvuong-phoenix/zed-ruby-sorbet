# Ruby Sorbet extension for Zed editor

An extension to enable *stable* Sorbet LSP for [Zed editor](https://zed.dev/).

> The [Ruby Sorbet VSCode extension](https://marketplace.visualstudio.com/items?itemName=sorbet.sorbet-vscode-extension) has 3 versions of running the LSP:
> - `stable`
> - `beta`: run via `--enable-all-beta-lsp-features`
> - `experimental`: run via `--enable-all-experimental-lsp-features`

## 🚀 Usage

**Disclaimer**: I do not think it's a good idea to publish the extension as a concrete Rust crate as this should be embedded directly into Zed's repo like [other official LSPs](https://github.com/zed-industries/zed/tree/main/extensions). The extension certainly needs to be more polished, too. Therefore, the usage instructions will recommend you to clone this repo and use it as an Dev Extension locally.

> Prerequisites:
> - [Rust](https://www.rust-lang.org/)
> - `wasm32-wasi` target (can be installed via `$ rustup target add wasm32-wasi`)

1. Clone the repo

    ```shell
    $ git clone https://github.com/cuongvuong-phoenix/zed-ruby-sorbet.git --depth=1
    ```

2. Open Zed editor -> `Extensions` -> Click `Install Dev Extension` -> Choose the cloned repo folder

3. Wait for Zed to compile the extension (can `open log` to see progress or errors if occur). When done, the extension will show in the list of `Installed` extensions.

4. Open Zed Settings and add the Sorbet LSP to the list of `language_servers` for `Ruby` language. For example:

    ```json
    {
      "languages": {
        "Ruby": {
          "language_servers": ["ruby-lsp", "ruby-sorbet", "rubocop", "!solargraph", "..."]
        },
      },
    }
    ```
