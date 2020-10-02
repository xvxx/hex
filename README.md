# decimal -> hex

Probably easier to just use one of these:

https://stackoverflow.com/questions/378829/convert-decimal-to-hexadecimal-in-unix-shell-script

    usage: hex NUMBER

    examples:

        $ hex 20,123  # 4E9B
        $ hex 4096    # 1000
        $ hex 1000    # 3E8

Install with [cargo](https://rustup.rs/):

    cargo install --git https://github.com/xvxx/hex
