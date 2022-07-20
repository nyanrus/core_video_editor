# フレーム I/O(フィルタ系含む)の API

config 設定で json5 を受け付ける

```json5
{
  a: {
    index: 0, //GUIに表示される順番
    default: 0, //デフォルトの値、型の検知もこれで
    value: 0, //変わる値
    scripts : [{
      lang:["en-us","en"]//言語 RFC-4646準拠
      abbr-description: "value for test" //簡略な説明(GUI上に表示される)
      description:"""value for test
      roughly long sentence
      """, //説明(ツールチップ用？)
    },
    {
      lang:["ja-jp","ja"]//日本語版
      abbr-description: "テスト用の値"
      description:"""テスト用の値
      適当な長い文章
      """,
    }],
  },
}
```
