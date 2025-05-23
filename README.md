# 🚀 Velox – 3D Character Controller for Bevy

**Velox** is a Bevy crate offering a **smooth and customizable 3D Character Controller**, powered by **Avian Physics**. Perfect for **FPS, TPS, platformers, and more**, it provides robust handling of movement, jumping, gravity, and collisions.

## 🌟 Features
- ✅ **Compatible with Avian Physics** – Uses a lightweight and efficient physics engine.
- ✅ **Natural movement** – Supports walking, jumping, gravity, slope handling, and collisions.
- ✅ **Customizable** – Adjust speed, acceleration, forces, and other parameters.
- ✅ **Easy to integrate** – Bevy plugin ready to use, designed for 3D games.
- ✅ **Extensible** – Easily add your own behaviors.

---

## 📦 Installation
Add the dependency to your **Cargo.toml** file:
```toml
[dependencies]
velox = { git = "https://github.com/suleymanlaarabi/velox", branch = "main"}
```

---

## ⚡ Quick Start
### 1️⃣ **Adding the plugin to Bevy**
```rust
use bevy::prelude::*;
use velox::VeloxPlugin;
use avian3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(VeloxPlugin)
        .run();
}
```

### 2️⃣ **Creating a character with Velox**
```rust
use bevy::prelude::*;
use velox::prelude::*;

fn setup(mut commands: Commands) {
    commands
        .spawn((
            CharacterController,
            CharacterInputConfig::default(),
            CharacterMovement::default()
        ))
        .with_children(|parent| {
            parent.spawn(CharacterControllerCamera);
        });
}
```

---

## 📜 License
Velox is licensed under **MIT**, meaning you can freely use it in your projects.

---

## 🛠️ Contributions
Contributions are welcome! If you want to add features or fix bugs, feel free to open an issue or pull request.

---

🔥 **Ready to bring your characters to life?** Integrate **Velox** into your Bevy game now! 🚀
