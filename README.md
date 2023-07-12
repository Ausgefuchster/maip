# maip

maip stands for "merge AWS IAM policies". It is a command line tool that merges multiple AWS IAM policies into one.
Since AWS has quotas that limit the number of managed policies that can be attached to a user, group or role,
this tool can be used to merge multiple policies into one.

## ⚙️ Installation

## 📖 Usage 

```
maip merge --out <output file> --file <input file 1> --file <input file 2> ...
```

You can provide as many input files as you want. 
The output file will be created if it does not exist and overwritten if it does.

If you have many policies to merge, you can use the `--all` flag to merge all policies in the current directory:

```
maip merge --all <directory> --out <output file>
```

## 🎯 Features

This is a list of features and planned features:

- [x] Merge multiple policies into one
- [ ] Merge all policies in a directory
- [ ] Merge all policies in a directory and all subdirectories recursively
- [ ] Merge managed policies from AWS by ARNs