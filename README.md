# conda-share

Have you ever want to make conda environments sharable? Well this is the package for you!

After years of waiting for someone to build a tool that exports conda environments that can easily transfer across computers (and operating systems) while still keeping the package versions, I decided to build it myself.

## Why use conda-share?

### TL;DR

There is (at time of writing) no conda command in existence that exports only the user installed packages, with their version numbers, and also includes the installed pip packages. This is what you need to share an environment consistently and effectively.

conda-share makes this easy.

### So why not just use "conda env export"?

Well, it outputs all the operating system specific packages too!

So if you export on your lab server that runs Linux and try to install it on your MacBook, it won't be able to build. That is not because there is any difference in practice if you manually install the packages, but because of those extra operating system specific packages.

Guess how I know this...

Conda-share solves this problem by only including the packages you specifically installed, just like `conda env export --from-history`

### So why not just use "conda env export --from-history"?

Because that command doesn't return any version numbers that you didn't ask for at time of install. It also doesn't include any pip packages.

This means that if you were to `conda install python=3.13 numpy` at the beginning, then only the `python` package will have a version number in your export. If numpy has upgraded 5 major versions since then, then when the next person goes to recreate this environment, they will install the wrong version of numpy.

To solve this, conda-share only includes the packages in `--from-history`, but it includes the version numbers as well. Additionally, conda-share includes the pip packages.

### Other small benefits

There are other small benefits to using this software

- If you happen to make a typo when writing the environment name, it will tell you if the environment doesn't exist. I'm surprised that the default conda command doesn't error in this situation.
- You help me get my name out there. :)

## How to get the executable

Go to the releases page in this GitHub repo and download the right executable for you.

There are two different executables for each OS:

- conda-share, which is the CLI version
- conda-share-gui, which is the GUI version

## How to use the CLI version

```bash
# Export environment to a file with the same name as the environment
conda-share <env_name>
# ex: conda-share test_env
# creates test_env.yml in the current folder

# Export environment to a file with a different location/name
conda-share <env_name> -p <new_file_path>
# ex: conda-share test_env -p ~/my_envs/my_test_env.yml
# creates my_test_env.yml inside the ~/my_envs folder

# Export environment and just display it to the screen
conda-share <env_name> -d
# ex: conda-share test_env -d
# outputs the test_env yaml to screen
```

## How to use the GUI version

Install it and use it.

I personally think it is intuitive, but if you don't think so then please let me know. I'll try to make it more clear.

## How to Build

1. Install rust [at this website](https://rust-lang.org/tools/install/).
1. Run `cargo build -r` in this repo.
1. Copy the `conda-share` executable from `target/release/conda-share` to your normal executable location.
1. Run it.
