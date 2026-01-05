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

The secondary goal is to provide a **programmatic API** for Python applications to read and evaluate conda environments.

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
- Convenient functions to get info from conda environments in Python.
- You help me get my name out there. :)

## How to use

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
