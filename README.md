# conda-share

Have you ever want to make conda environments sharable? Well this is the package for you!

## So why not just use "conda env export"?

Well, it outputs all the operating system specific packages too!

So if you export on your lab server that runs Linux and try to install it on your MacBook, it won't be able to build. That is not because there is any difference in practice if you manually install the packages, but because of those extra operating spsecific packages.

Guess how I know this...

## So why not just use "conda env export --from-history"?

Because that doesn't return any version numbers that you didn't ask for at time of install. It also doesn't include any pip packages.

This adds the version numbers to the top level packages provided in the --from-history command and also adds the pip packages back in.

## Other small benefits

There are other small benefits to using this software

- If you happen to make a typo when writing the environment name, it will tell you if the environment doesn't exist. I'm surprised that the default conda command doesn't error in this situation.
- You help me get my name out there. :)
