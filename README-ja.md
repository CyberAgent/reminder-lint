# reminder-lint
`reminder-lint`はあらゆる言語や設定ファイルに対応した、コードリマインドツールです。  

[GitHub Actionsで使用する](https://github.com/CyberAgent/reminder-lint#GitHub-Actions)  
[ローカル環境にインストールする](https://github.com/CyberAgent/reminder-lint#Install)

## コンセプト
`reminder-lint`は、コード上のTODOコメントを消し去ることを目標に開発されています。  
コードを書きながら、後回しにしたい処理には通常TODOコメントを残しますが、`reminder-lint`では規約に沿ったコメントを残すことで、時間が経過してもTODOコメントの消し忘れに気づくことができます。  

## コメントのSyntax
デフォルトでは、`reminder-lint`はカレントディレクトリ配下のファイルを再帰的に操作し、正規表現`remind:\W?`にマッチする行を探索します。  
正規表現にマッチした行のうち`%Y/%m/%d`表記にマッチする日付を探し、記述された日付が現在時刻を超過している場合に，終了コード1とともにプロセスを終了します。
具体的には、TODOコメントの代わりに、以下のようにコメントを記述します。
```rust
// ...
// remind: 2024/06/27 remove after enabling this feature flag.
if perfect_feature_enabled {
  // perfect program
}
```

記述した日程以降、Feature Flags削除のコメントが`reminder-lint`の実行結果に表示されるようになり、終了コード1でプロセスを終了します。
```shell
$ reminder-lint run
./src/main.rs:2 // remind: 2024/06/27 remove after enabling this feature flag.
```

## インストール

### Homebrew
```shell
$ brew install CyberAgent/tap/reminder-lint
```

### Docker
```shell
$ docker run --rm -v "$(pwd):/workspace" --workdir /workspace ghcr.io/cyberagent/reminder-lint:latest run
```

### Binary
ビルド済みのバイナリを[リリース](https://github.com/CyberAgent/reminder-lint/releases/latest)からインストールできます。


## GitHub Actions
GitHub Actions上で`reminder-lint`を利用することができます。
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
        uses: CyberAgent/reminder-lint@0.1.2
        with:
          args: run
```

もし、reminder-lintの実行結果をSlackに通知したい場合、以下のようにワークフローを設定できます。
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
        uses: CyberAgent/reminder-lint@0.1.2
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

## カスタマイズ
チームによっては、リマインドコメントの記法を変更したり、時刻単位でのリマインドタイミングを指定したいケースが想定されます。  

`reminder-lint`は設定ファイルをサポートしており、これらの記法をカスタマイズして利用することが可能です。
`init`を実行すると、`remind.yml`ファイルや`.remindignore`ファイルが作成されます。

このときデフォルトで、 `.remindignore` に `remind.yml` が記述されています。

```shell
$ reminder-lint init
```

または、Dockerを利用して設定ファイルを生成することができます。
```shell
$ docker run --rm -it -v "$(pwd):/workspace" --workdir /workspace ghcr.io/cyberagent/reminder-lint:latest init
```

### Syntaxのカスタマイズ
例えば、より口語的なコメントにすることも可能です。
```yml
comment_regex: "remind @.+ at"
datetime_format: "%Y/%m/%d HH:MM:SS"
search_directory: .
```

または、既存のTODOを`reminder-lint`でマッチさせることも可能です。
```yml
comment_regex: "(?i)TODO"
datetime_format: "%Y/%m/%d HH:MM:SS"
search_directory: .
```

この場合、以下のようにリマインドコメントを記述できます。
```rust
// ...
// remind @arabian9ts at 2024/06/27 10:00:00 remove after enabling this feature flag.
if perfect_feature_enabled {
  // Do perfect feature
}
```

ただし、フォーマットを複雑にするほど、フォーマットに則っていないコメントが検査が漏れる可能性があるため、できるだけシンプルなフォーマットで運用することを推奨します。


## TODOからのマイグレーション
既にあるTODOコメントを処理していくために、`reminder-lint`を利用して段階的にコードベースのTODOの削除を進めることができます。  
1. 最初は、`comment_regex: (?i)TODO`を設定し、既存のTODOコメントをリマンド対象にします。
2. 日付が含まれていないTODOコメントをリマインドするか、プロジェクトによって設定ファイルで挙動を変更します(`remind_if_no_date`)。
3. 削除したいTODOのリマインド日を決めて、TODOコメントに日付を含めるように修正します。
4. TODOコメントを対応して、コメントを削除します。
