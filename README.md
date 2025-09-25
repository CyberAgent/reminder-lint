# reminder-lint
`reminder-lint` is a code reminder tool that supports all languages and configuration files.

[Use with GitHub Actions](https://github.com/CyberAgent/reminder-lint#GitHub-Actions)
[Install in local environment](https://github.com/CyberAgent/reminder-lint#Install)

## Concept
`reminder-lint` is designed to eliminate TODO comments in the code
When writing code, we usually leave TODO comments for processes we want to postpone, but with `reminder-lint`, you can notice forgotten TODO comments over time by leaving comments according to the rules.

## Comment Syntax
By default, `reminder-lint` recursively search file lines that match the regular expression `remind:\W?` under the current directory.
Among the lines that match the regular expression, it searches for dates that match the `%Y/%m/%d` notation, and if the described date exceeds the current time, the process ends with exit code 1.
Specifically, instead of TODO comments, write comments as follows.
```rust
// ...
// remind: 2024/06/27 remove after enabling this feature flag.
if perfect_feature_enabled {
  // perfect program
}
```

After the described date, comments will be displayed in the execution result of `reminder-lint`, and the process will end with exit code 1.
```shell
$ reminder-lint run
./src/main.rs:2 // remind: 2024/06/27 remove after enabling this feature flag.
```

## Install

### Homebrew
```shell
$ brew install CyberAgent/tap/reminder-lint
```

### Docker
```shell
$ docker run --rm -v "$(pwd):/workspace" --workdir /workspace ghcr.io/cyberagent/reminder-lint:latest run
```

### Binary
Pre-built binaries are available on the [releases page](https://github.com/CyberAgent/reminder-lint/releases/latest).


## GitHub Actions
You can use `reminder-lint` on GitHub Actions.

```yml
name: reminder-lint

on:
  schedule:
    - cron: '0 1 * * 1,2,3,4,5'

jobs:
  run:
    runs-on: ubuntu-latest
    name: reminder-lint
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Run
        uses: CyberAgent/reminder-lint@latest # Recommended to specify with a full-length commit SHA
        with:
          args: run
```

If you want to notify the result of `reminder-lint` to Slack, you can use the following action.
```yml
name: reminder-lint

on:
  schedule:
    - cron: '0 1 * * 1,2,3,4,5'

jobs:
  run:
    runs-on: ubuntu-latest
    name: reminder-lint
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Run
        id: run
        continue-on-error: true
        uses: CyberAgent/reminder-lint@latest # Recommended to specify with a full-length commit SHA
        with:
          args: run

      - name: Notify
        if: ${{ steps.run.outputs.stdout != '' }}
        uses: slackapi/slack-github-action@v2.0.0
        with:
          webhook: ${{ secrets.SLACK_WEBHOOK_URL }}
          webhook-type: incoming-webhook
          payload: |
            {
              "blocks": [
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": "GitHub Actions: ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}"
                  }
                },
                {
                  "type": "section",
                  "text": {
                    "type": "mrkdwn",
                    "text": ${{ toJSON(format('```{0}```', steps.run.outputs.stdout)) }}
                  }
                }
              ]
            }
```

## Customize
Some teams may want to change the notation of reminder comments or specify the reminder timing in more detail.

`reminder-lint` supports configuration files, and you can customize these notations and use them.
When you execute `init`, `remind.yml` file and `.remindignore` file are created.

By default, `.remindignore` contains `remind.yml`.

```shell
$ reminder-lint init
```

Or, you can use Docker to generate configuration files.
```shell
$ docker run --rm -it -v "$(pwd):/workspace" --workdir /workspace ghcr.io/cyberagent/reminder-lint:latest init
```

### Syntax Customization
For example, you can make the comments more colloquial.
```yml
comment_regex: "remind @.+ at"
datetime_format: "%Y/%m/%d HH:MM:SS"
search_directory: .
```

Or, you can match existing TODOs with `reminder-lint`.
```yml
comment_regex: "(?i)TODO"
datetime_format: "%Y/%m/%d HH:MM:SS"
search_directory: .
```

In this case, you can write reminder comments as follows.
```rust
// ...
// remind @arabian9ts at 2024/06/27 10:00:00 remove after enabling this feature flag.
if perfect_feature_enabled {
  // Do perfect feature
}
```

However, the more complex the format, the more likely it is that non-conforming comments will be missed during inspection. Therefore, it is recommended to use the simplest format possible.

## Validation of Reminder Comments
`reminder-lint` can validate reminder comments.

For example, by configuring `remind.yml` (or `remind.yaml`) as follows, you can validate the date format and assigned user name format as mandatory.

```yml
comment_regex: remind:.*
search_directory: .
trigger:
  datetime: "%Y/%m/%d"
validate:
  datetime:
    format: "%Y/%m/%d"
  assignee:
    format: "@(kqito|arabian9ts|dora1998)"
```

When you run `reminder-lint validate` in this state, reminder comments that do not follow the expected format will be detected, and the process will terminate with exit code 1.

```shell
./main.go:11 // remind: hoge: missing date and assignee
Missing `assignee` format: @(kqito|arabian9ts|dora1998)
Missing `datetime` format: %Y/%m/%d

./main.go:14 // remind: 2024/05/02: missing assginee
Missing `assignee` format: @(kqito|arabian9ts|dora1998)

./main.go:17 // remind: @arabian9ts missing date
Missing `datetime` format: %Y/%m/%d
```

## Migration from pure TODOs
You can use `reminder-lint` to gradually remove TODOs from your codebase that already exist.
1. Initially, set `comment_regex: (?i)TODO` to make existing TODO comments a reminder target.
2. Decide whether to remind TODO comments without a date, or change the behavior in the configuration file depending on the project (`remind_if_no_date`).
3. Decide the reminder date for the TODO you want to delete and modify the TODO comment to include a date.
4. Work on the TODOs and delete comments.
