# dotrs

A try on building a dotfiles manager which makes managing and synchronizing dotfiles across multiple systems as
seamless and easy as possible.

> [!Warning]
> This tool is currently under construction and thus neither feature-complete nor has it been field-tested enough
> to ensure a confident level of consistency and reliability.

## Feature Roadmap

- [x] Import dotfiles from a remote repository
- [x] Apply dotfiles to home directory
- [x] Update dotfiles to remote directory
- [x] Service to automatically sync dotfiles with remote repository
- [x] Profiles and templating support
- [ ] Value encryption in profiles

## Installation

You can install dotrs either by downloading the precompiled binaries for your system configuration from the
[**releases page**](https://github.com/shellshape/dotrs), or by installing it from source by using cargo install.

```bash
cargo install dotrs
```

To be able to properly use the `dotrs cd` command, you can use the [`bin/wrapper-bash.sh`](bin/wrapper-bash.sh) script.
Simply download it to your system and source it in your `.profile`.

```bash
curl -Lo /usr/local/bin/dotrs-wrapper.sh \
    "https://raw.githubusercontent.com/shellshape/dotrs/refs/heads/main/bin/wrapper-bash.sh"
echo "source /usr/local/bin/dotrs-wrapper.sh" >> "$HOME/.profile"
```

### Service Setup

_WIP_

## Concept

### CLI Tool

dotrs uses a central Git repository as reference to share and synchronize dotfiles across your systems. You can import
_(or simply point dotrs via the config to an already cloned repository)_ a repository of dotfiles. These are stored in
a _stage_ directory. There, you can edit and modify your dotfiles and templates. From there, with the `apply` command,
the templates are processed and the dotfiles are copied to your home directory. With the `pull` command, you can update
your stage from the upstream repository. With the `update` command, you can commit your changes in the stage and push
them to the upstream repository.

### Service Module

But because we are all lazy and forget to actively pull, commit and sync our stuff, dotrs tries to do all that for you
automatically with the provided service module, which is directly integrated into the tool and can be started with the
`start-service` command. _Example service configuration is also available in the [service/](service/) directory._ The
service watches the stage directory for changes and applies them automatically to your home directory. Also, after a
debounce period, changes are automatically committed and pushed from the stage to the upstream repository. Also,
periodically, the service pulls changes from the upstream repository into the stage directory and applies them, so that
your dotfiles are always up-to-date across devices.
