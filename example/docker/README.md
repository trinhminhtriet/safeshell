# Docker Example

If you would like to see how safeshell works as a command line and shell plugin, you can run this docker image by running the following command

```sh
docker build -t safeshell .
docker run -it safeshell bash
```

After exec to the container you can:
1. See safeshell findings when open the shell
2. [See sensitive data](../../README.md#eyes-find-sensitive-commands)
3. [Delete sensitive data](../../README.md#broom-clear-findings-)
4. See more features in [README file](../../README.md)