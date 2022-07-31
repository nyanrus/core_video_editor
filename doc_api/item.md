# アイテム

## アイテム構造

```json5
[
  'type:{
    ulid:"",
    settings:[{}], //frame_io.md 参照
    child:[
      'type:{
        ulid:"",
        settings:[],
        child:[],
      },
      'type:{
        ulid:"",
        settings:[],
        child:[],
      }
    ],
  },
]
```

'type は `{prefix}_{filter_name};author`の感じになる \
e.g.

```
  i_video;official
  f_cv;nyanrus
```

i は Item,
f は Frame(Interface)だ

## アイテムプロパティ

AviUtl を参考に、 \
バッファは固定（解像度）で \
フィルタ処理、XYZ、拡大率、透明度、回転の順に処理
