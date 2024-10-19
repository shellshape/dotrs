# dotrs

A try on building a dotfiles manager which makes managing and synchronizing dotfiles across multiple systems as
seamless and easy as possible.

> [!Warning]
> This tool is currently under construction and thus neither feature-complete nor has it been field-tested enough
> to ensure a confident level of consistency and reliability.

## Concept

### CLI Tool

dotrs uses a central Git repository as reference to share and synchronize dotfiles across your systems. You can import
*(or simply point dotrs via the config to an already cloned repository)* a repository of dotfiles. These are stored in
a *stage* directory. There, you can edit and modify your dotfiles and templates. From there, with the `apply` command,
the templates are processed and the dotfiles are copied to your home directory. With the `pull` command, you can update
your stage from the upstream repository. With the `update` command, you can commit your changes in the stage and push
them to the upstream repository.

### Service Module

But because we are all lazy and forget to actively pull, commit and sync our stuff, dotrs tries to do all that for you
automatically with the provided service module, which is directly integrated into the tool and can be started with the
`start-service` command. *Example service configuration is also available in the [service/](service/) directory.* The
service watches the stage directory for changes and applies them automatically to your home directory. Also, after a
debounce period, changes are automatically committed and pushed from the stage to the upstream repository. Also,
periodically, the service pulls changes from the upstream repository into the stage directory and applies them, so that
your dotfiles are always up-to-date across devices.