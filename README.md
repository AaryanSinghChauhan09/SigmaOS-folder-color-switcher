    # SigmaOS-folder-color-switcher

    Colour-code folders in the file manager

    Part of the **SigmaOS** sovereign operating-system ecosystem — a multi-repository
    distribution modelled on Linux Mint and Omarchy Linux. See the parent
    [`SigmaOS`](https://github.com/AaryanSinghChauhan09/SigmaOS) repository for the
    full component index.

    ## Upstream inspirations
    - https://github.com/linuxmint/folder-color-switcher

    ## Design principles applied
    - **Separation of Mechanism & Policy**
- **Defensive Programming**
- **Fail-Fast**
- **KISS**
- **Least Astonishment**

    ## Usage
    Verbs: `set, get, clear, list, palette`

    ```
    $ sigma-foldercolor palette
$ sigma-foldercolor set ~/Documents red
$ sigma-foldercolor get  ~/Documents
red
$ sigma-foldercolor clear ~/Documents

    ```

    ## Build & test
    ```
    make build
    make test
    ```

    ## Licence
    GNU General Public License v3.0 (FOSS).
