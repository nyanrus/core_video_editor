# モジュール(プラグイン)の設計

Core 側が Module の init を呼び出し、その時 Core の機能＆関数のリファレンスを渡す。\
優先度の概念は検討中 \
Core 側で Module を登録するか Module 側で登録するかどっちか \
依存関係を求めて、それを基盤に依存するものが全部読み込まれた後 \
init を再度呼び出す \
Core はいま読み込んでいる Module の Array のポインタを ReadOnly で渡す \
Core 側でチェック後 Module 側で再チェック必要性あるのかわからない。

Rust は固定的な abi を持たないので`extern "C"`をするか `https://crates.io/crates/abi_stable`に頼ることになる。
`extern "C"`のほうが C/C++の API がある言語たちが参戦できていいのだが、`Vec<T>`などで揉めることになりそう。
