# アイテム構造

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

'type は `{prefix}\_{filter_name};author`の感じになる
e.g.

```
  i_video;official
  f_cv;nyanrus
```

i は Item,
f は Frame(Interface)だ
