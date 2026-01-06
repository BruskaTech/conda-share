# conda-share

Have you ever want to make conda environments sharable? Well this is the package for you!

After years of waiting for someone to build a tool that exports conda environments that can easily transfer across computers (and operating systems) while still keeping the package versions, I decided to build it myself.

## Why use conda-share?

### TL;DR

There is (at time of writing) no conda command in existence that exports only the user installed packages, with their version numbers, and also includes the installed pip packages. This is what you need to share an environment consistently and effectively.

conda-share makes this easy.

There is even a single line you can add to your python files and jupyer notebooks to save an updated environment every time it is run (so you don't have to keep track of it).

### General Goals

The primary goal is to generate a **shareable environment YAML** that is closer to “what you explicitly installed” (via `--from-history`) while also including version
numbers and pip packages.

The secondary goal is to provide a **programmatic API** for Python and Rust applications to read and evaluate conda environments.

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

There are other small benefits to using this software:

- If you happen to make a typo when writing the environment name, it will tell you if the environment doesn't exist. I'm surprised that the default conda command doesn't error in this situation.
- Convenient functions to get info from conda environments in Python and Rust.
- You help me get my name out there. :)

## How to get the executable

Go to the releases page in this GitHub repo and download the right executable for you.

There are two different executables for each OS:

- conda-share, which is the CLI version
- conda-share-gui, which is the GUI version

## How to use

### CLI version

Download it, unzip it, and use it.

The exception is of course MacOS with it's extra security settings. Since the app built in github actions is not a signed release, you will need to let MacOS know that the app is safe to run by running the following command:

```bash
# Make sure to replace <path_to_file> with the path to your downloaded and unzipped conda-share-gui file.
xattr -d com.apple.quarantine <path_to_file>/conda-share
```

Here are some examples on how to use it.

```bash
# Export environment to a file with the same name as the environment
./conda-share <env_name>
# ex: conda-share test_env
# creates test_env.yml in the current folder

# Export environment to a file with a different location/name
./conda-share <env_name> -p <new_file_path>
# ex: conda-share test_env -p ~/my_envs/env.yml
# creates my_test_env.yml inside the ~/my_envs folder

# Export environment and just display it to the screen
./conda-share <env_name> -d
# ex: conda-share test_env -d
# outputs the test_env yaml to screen
```

### GUI version

Download it, unzip it, and use it.

The exception is of course MacOS (again) with it's extra security settings. Since the app built in github actions is not a signed release, you will need to let MacOS know that the app is safe to run by running the following command:

```bash
# Make sure to replace <path_to_file> with the path to your downloaded and unzipped conda-share-gui file.
xattr -d com.apple.quarantine <path_to_file>/conda-share-gui
```

### Python version

This makes it super easy to automatically keep your environment up to date for a python file or a jupyter notebook.

First, make sure conda-share is installed in your conda environment.

```bash
# Make sure to replace <env_name> with the environment name
conda activate <env_name>
pip install conda-share
```

Then, just add this one line to the top of your code.

```python
import conda_share; conda_share.save_current_env()
# You can also save it to a different directory with a different name.
# import conda_share; conda_share.save_current_env("/path/to/my_envs/env.yml")
```

Voilà! That's it.

There are also other functions available in the python package for viewing and sharing conda environments.

### Rust version

Just do ```cargo add conda_share``` and start coding.

## So I have the yaml file, how do I setup the environment on a new computer?

Good question. Make sure conda is installed on the computer and then just run the following command.

```bash
# Make sure to replace <env_file> with the yaml file path
conda env create -f <env_file>
```

If you want it to also show up in jupyter-lab, you will need to run the following commands as well. You *may* need to restart your jupyter-lab server after it is added.

```bash
# Make sure to replace <env_name> with the environment name
conda activate <env_name>
conda install ipykernel
python -m ipykernel install --user --name <env_name>
```

## How to Build

### Rust parts

1. Install rust [at this website](https://rust-lang.org/tools/install/).
    1. You may need to update rust first if it was already installed with `rustup default stable; rustup update`
1. Run `cargo build -r` in the repo.
1. Copy the `conda-share` executable from `target/release/conda-share` to your normal executable location.
1. Run it.

### Python parts

1. Make sure the rust parts (or at least conda_share) build correctly first.
1. Install [maturin](https://www.maturin.rs/installation.html) using pip, pipx, or uv.
1. Run `maturin build --release --sdist --manifest-path conda_share_py/Cargo.toml` in the repo.
1. Look in the `target/wheels` folder to see the newly build .whl files.
1. Install the new whl file with `pip install target/wheels/<my_whl_file>.whl`

### Other tips

1. If you are building for an actual github release, please make sure to run these three commands locally BEFORE making the release!!!

    ```bash
    cargo build
    cargo publish --dry-run -p conda_share
    maturin build --release --sdist --manifest-path conda_share_py/Cargo.toml
    ```
