# maip

maip is short for "merge aws iam policies," 
a command-line utility designed to consolidate multiple AWS IAM policies into a single entity.
Given that AWS imposes restrictions on the number of managed policies that can be linked to a user, group, or role,
this tool can be used to combine multiple policies into one.

## ⚙️ Installation

## 📖 Usage

```sh
maip merge --out <output file> --file <input file 1> --file <input file 2> ...
```

You can provide as many input files as you want.
The output file will be created if it does not exist and overwritten if it does.

If you have many policies to merge, you can use the `--all` flag to merge all policies in the current directory:

```sh
maip merge --all <directory> --out <output file>
```

### Using Docker

To use docker you can simply mount the directory containing the policies to merge into the container:

```sh
docker run -v <directory>:/policies ghcr.io/ausgefuchster/maip:<version> merge --all /policies
```

For running it with AWS managed policies you need to
provide the environment variables to the container:

```sh
docker run -v <directory>:/policies -e AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY -e AWS_DEFAULT_REGION ghcr.io/ausgefuchster/maip:<version> merge --all /policies
```

## 🎯 Features

This is a list of features and planned features:

- [x] Merge multiple policies into one
- [x] Merge all policies in a directory
- [ ] Merge all policies in a directory and all subdirectories recursively
- [x] Merge managed policies from AWS by ARNs