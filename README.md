This is my chess engine written in rust.

Master branch has my first version using depth first search.

BFS branch has an updated version using breadth first search which is currently the best.

Threads branch is a version of master which runs on many threads.

# Setup

`git clone https://github.com/RuairidhWilliamson/chess.git`

`git checkout bfs`

Copy config_example.json to config.json and fill in your lichess bot details.

Then run `cargo run`

# Config

You can change the time the engine spends in `engine/engine_config.rs` by default it will use default_debug.

You can specify the time the engine should aim to spend what fraction of the time should be spent on searching deep moves such as checks and captures and the maximum deep depth.
