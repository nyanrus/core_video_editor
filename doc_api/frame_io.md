# フレーム I/O(フィルタ系含む)の API

register

```json5
{
  name: "test module",
  authors: ["NyanRus"],
  url: [],
  version: "0.0.1beta",
  tag: ["input", "output", "frame", "ui"],
}
```

config 設定で json5 を受け付ける

```json5
{
  a: {
    index: 0, //GUIに表示される順番
    default: 0, //デフォルトの値、型の検知もこれで
    value: 0, //変わる値
    scripts : [{ // 言語設定がlangに当てはまらない場合、一番最初のものを表示する。
      lang:["en-us","en"],//言語 RFC-4646準拠 複数指定可
      name:"a",//値の名前
      abbr-description: "value for test", //簡略な説明(GUI上に表示される)
      description:"""value for test
      roughly long sentence
      """, //説明(ツールチップ用？)
    },
    {
      lang:["ja-jp","ja"],//日本語版
      name:"あ",
      abbr-description: "テスト用の値",
      description:"""テスト用の値
      適当な長い文章
      """,
    }],
  },
}
```
