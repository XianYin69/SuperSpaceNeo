## 1. 核心开发语言 (Core Language)
- C++ 17/20: 利用现代 C++ 的内存管理（智能指针）和高效率，确保作为系统级工具的响应速度（毫秒级唤起）。

- QML (Qt Quick): 用于构建“绝对理性”的扁平化前端界面。通过 GPU 渲染实现丝滑的列表滚动和 UI 响应，且开发效率远高于纯原生 Win32/X11。

## 2. 构建与项目管理 (Build System)
- CMake: 作为构建生成器，确保 Windows (MSVC) 和 Linux (GCC/Clang) 的跨平台编译逻辑一致。

- Ninja: 作为后端构建工具，替代传统的 make，实现极致的增量编译速度。

## 3. 数据持久化与通信 (Data & Network)
- JSON (nlohmann/json): 替代 Redis。作为配置文件和插件索引的存储格式，具有良好的可读性且跨平台通用。

- Qt Network: 用于从 GitHub 异步下载插件列表 (index.json) 和插件包，不阻塞 UI 线程。

## 4. 关键第三方集成 (Libraries)
- Everything SDK (Windows): 通过 IPC 机制接入，实现全盘毫秒级文件搜索，无需自建索引数据库。

- System Hooks (Native API):

- Windows: user32.dll (RegisterHotKey)。

- Linux: X11 或 Wayland 原生全局热键监听。

## 5. 插件系统架构 (Plugin Architecture)
- 载体: 逻辑部分采用 Lua (via sol2) 或 二进制可执行文件（通过 Process IPC）。

- 分发: GitHub Raw / Releases。将 GitHub 作为一个去中心化的“插件商店”。

## 6. 视觉与交互规范 (Design Logic)
- 风格: Rational Flat Design (绝对理性扁平化)。

### 核心特性:

- 无边框 (Frameless)。

- 无阴影与圆角。

- 高对比度网格对齐。

- 字体驱动（使用 Inter 或 Fira Code 等工程感字体）。