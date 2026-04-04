## 1. 版本控制与协作流：Git Gitflow
- 不要只是简单地 git push。引入分支管理概念：

- Main 分支：永远保持可运行的稳定版本。

- Develop 分支：日常开发的主战场。

- Feature 分支：每个新功能（如 feat/plugin-system）单独开分支，完成后通过 Merge Request (MR) 或 Pull Request (PR) 合并。

- 规范化 Commit：使用 Conventional Commits 规范。例如：feat: 增加本地 JSON 配置读取 或 fix: 修复 Linux 下热键冲突。

## 2. 自动化构建与持续集成 (CI/CD)
- 利用 GitHub Actions。这是现代开发流的灵魂：

- 自动编译：每当你 push 代码，GitHub 自动在云端调用 Ninja + CMake 尝试在 Windows 和 Ubuntu 上编译你的代码。

- 产物发布：一旦你打上版本标签（Tag），Actions 自动生成对应的 .exe 或二进制压缩包并挂在 Release 页面。

- 代码静态分析：引入 clang-format 和 cppcheck。每次提交代码时，自动检查代码风格是否符合规范（比如缩进是否对齐、是否有内存泄漏隐患）。

## 3. 依赖管理方案
- 手动下载 .h 和 .lib 文件是上个世纪的做法。

- CMake FetchContent：在 CMakeLists.txt 中直接声明 nlohmann/json 或 sol2 的 GitHub 地址。编译时，CMake 会自动下载并关联，你的工程目录永远保持干净。

- Vcpkg / Conan：如果后续引入更大型的库（如 OpenCV 或 Protocol Buffers），使用这些 C++ 包管理器，一键处理编译依赖。

## 4. 文档驱动开发 (Documentation as Code)
- Markdown：所有的方案设计、插件协议全部写在项目根目录的 docs/ 下。

- Doxygen：在 C++ 头文件中写规范注释，自动化生成网页版的 API 文档，这在多人参与或后续维护插件系统时至关重要。

## 5. 开发环境一致性 (Dev Containers)
- 如果你在 Windows 和 Linux 之间切换，或者以后有同学想加入你的项目：

- 使用 VS Code Dev Containers。你可以通过一个 .devcontainer 文件夹定义完整的开发环境（包含特定的 GCC 版本、CMake 版本和 Qt 环境）。

- 无论谁拉取代码，只要点击“在容器中打开”，环境瞬间配好，彻底告别“我电脑上能跑，你那里不行”的尴尬。