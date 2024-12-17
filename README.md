# keycap
任意のMisskeyサーバーのアカウントに接続し、タイムラインの閲覧、通知の確認、ノートの送信が可能な非常に軽量なwebクライアントを配信するサーバープログラム

## 背景
Misskeyサーバーがデフォルトで提供するwebクライアントをモバイル端末から利用する場合、電力消費が大きいため電源が確保できない場所での使用が躊躇される。軽量な代替クライアントが欲しいが、iOS用アプリの開発環境を用意するのは嫌だ。webクライアントを配信するサーバーを作ることでこれを達成したい。

## 動かし方
- `git clone これ`
- `cd front`
- `yarn build`
- `cd ../`
- `cargo run`
- `http://localhost:3030` をブラウザで開く
- 必要な権限を含んだトークンと、 Misskey サーバーのホスト名を入力
- ノートを送ったり、ユーザー名を取得したりできます