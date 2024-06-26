# Space Cowboy RPG

## inspiration

The primary inspirations to this project are Skyrim, DND, Doug Doug's [babadaboosh](https://github.com/DougDougGithub/Babagaboosh.git), Joon Sung et. al.'s [Generative Agents: Interactive Simulacra of Human Behavior](https://github.com/joonspk-research/generative_agents), and my friends TTRPG setting.

## build

Download [this](https://drive.google.com/file/d/0B7XkCwpI5KDYNlNUTTlSS21pQmM), extract it, rename it to `word2vec.bin`, and place it in the `resources` folder

Next you need to setup your `config.toml` file using any text editor.
When done setting up, it should look something like this

As this makes use of cargo, the project can be built with the `cargo b` command.

After this, when built, if you choose to deploy the binary, make sure that it is in the same directory as your resource folder or a copy of it

```toml
openapi_key = "API_KEY"
elevenlabs_key = "API_KEY"
```

## contribution

Currently, this is being run by just me and nobody else, so contribution rules are subject to change. If you do wish to contribute, please reach out to me on discord at sofialo
