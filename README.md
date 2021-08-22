## What's this
CloudWatch Logs のログイベント内容を見るためのツール

## Usage
```bash
$ caw -h
caw 0.0.1

d2verb

A tool to inspect cloudwatch logs

USAGE:
    caw <SUBCOMMAND>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    list    List log groups
    show    Show log events
```
### List log groups
```bash
$ caw list
/aws/codebuild/codebuild-app-master
/aws/codebuild/codebuild-petclinic-master
/aws/codebuild/sample
...
```

ロググループ名のパターンを指定することもできる
```bash
$ caw list -p lambda
/aws/lambda/alertNotifier
/aws/lambda/lambda-config-demo
...
```

### Show log events message
```bash
$ caw show /aws/lambda/lambda-s3
+53602-12-29 23:10:31: START RequestId: cd4bf2fc-3dc5-46da-9935-08948038b437 Version: $LATEST
...
```

メッセージのパターンを指定することもできる
```bash
$ caw show /aws/lambda/lambda-s3 -p ERROR
+53605-11-23 07:01:05: [ERROR] Exception: boom!
Traceback (most recent call last):
  File "/var/task/lambda_function.py", line 5, in lambda_handler
    raise Exception("boom!")
...
```