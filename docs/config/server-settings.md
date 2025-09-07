# Server Settings

Representing `server.toml`

## Server Address

The address to bind the server to.

:::code-group
```toml [server.toml]
bind = "0.0.0.0:25565"
```
:::

## Welcome Message

Welcome message displayed to players after joining.
You can use color codes (like `§a` for green).

:::code-group
```toml [server.toml]
welcome_message = "Welcome to PicoLimbo!"
```
:::

Welcome message can be disabled by setting an empty string.

:::code-group
```toml [server.toml]
welcome_message = ""
```
:::

## Default Gamemode

The default game mode for players.

:::code-group
```toml [server.toml]
default_game_mode = "spectator"
```
:::

Possible values:
```
survival
creative
adventure
spectator
```

> [!NOTE]
> For Minecraft versions 1.7.x, the spectator game mode does not exist. If you set `default_game_mode = "spectator"`, it will spawn players in "creative" mode instead.

## Hardcore

Spawns the player in hardcore mode.

:::code-group
```toml [server.toml]
hardcore = true
```
:::
