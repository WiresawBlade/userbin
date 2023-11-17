<div align="center">

# UserBin

### 一个快速将可执行文件链接到指定目录的工具。

| [⚡ 功能](#-功能) | [📦 安装](#-安装) | [🛠️ 用法](#-用法) |
| ---------------- | ---------------- | ---------------- |

</div>

---

## ⚡ 功能

`UserBin` 是一个非常简单的工具，它只做一件事：为任意位置的可执行文件，在固定目录，创建指向该文件的 `PowerShell` 脚本文件，省去了将各种路径添加到 `PATH` 的麻烦。创建的脚本依旧支持读取命令行参数，因此该工具十分适合为各种命令行工具创建链接。

- 将可执行文件连接到固定目录，你只需要将这一个目录添加到 `PATH` ，即可对大量程序进行统一配置，类似 Linux 平台的 `/usr/bin` 
- 支持为链接设置别名

## 📦 安装

下载已编译的二进制文件，可通过 [Release 页面](https://github.com/WiresawBlade/userbin/releases) 获取。

## 🛠️ 用法

### 初次使用

- `UserBin` 需要知晓统一管理目录的路径，因此需要提前设置环境变量 `USERBIN_PATH` 
- 将设置了 `USERBIN_PATH` 的路径添加到 `PATH` 环境变量
- 为方便后续使用，建议将本程序添加到 `PATH` 环境变量中。`UserBin` 支持创建自身的链接，因此你只需要在首次使用时运行下列命令，后续即可在任意环境中使用：

```pwsh
userbin .\userbin.exe
```

### 命令行参数

```
USAGE:
    userbin.exe [OPTIONS] <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alias <alias>

ARGS:
    <target>
```

#### -a / --alias

设置链接的别名（可省）。

默认状态下，链接的别名取决于它指向的可执行文件的文件名。如一个名为 `target.exe` 的程序，其被创建的链接为 `target.ps1` 。有时我们可能想手动指定链接的名称，即可设置别名参数。

例：

```pwsh
userbin -a tar .\target.exe
```

它将会创建名为 `tar.ps1` 的脚本，而不是默认的 `target.ps1` 。

### 临时环境变量

如果不想设置永久环境变量，可以使用以下方式设置临时环境变量，该变量仅在当前会话有效。此方式同样适用于需要临时切换管理目录的场景，因为它会暂时覆盖永久变量。

```pwsh
$env:USERBIN_PATH = "D:\Me\bin" ; userbin .\target.exe
```
