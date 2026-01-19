# Arkinput

跨平台（Windows/macOS）输入记录工具，用于记录用户在电脑上的所有键盘输入，包括时间、应用程序和输入内容。

## 技术栈

- **后端**: Rust + Tauri 2.0
- **前端**: Vue 3 + TypeScript + Tailwind CSS
- **数据库**: SQLite (rusqlite)
- **键盘监听**: rdev (跨平台)

## 功能特性

- 全局键盘输入记录
- 自动检测当前活动窗口和应用
- 智能合并连续输入（500ms 超时）
- 按应用统计输入数据
- 支持搜索和筛选记录
- 排除指定应用的输入记录
- 数据导出为 JSON 格式
- 清理旧数据

## 安装和运行

项目位于 `/home/tison/projects/arkinput` 目录。

```bash
# 进入项目目录
cd /home/tison/projects/arkinput

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建生产版本
npm run tauri build
```

## 项目结构

```
arkinput/
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── main.rs               # 主入口
│   │   ├── lib.rs                # 库入口，Tauri 命令
│   │   ├── keyboard.rs           # 键盘事件监听
│   │   ├── window.rs             # 活动窗口检测
│   │   ├── database.rs           # SQLite 操作
│   │   └── models.rs             # 数据模型
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                          # Vue 前端
│   ├── App.vue
│   ├── main.ts
│   ├── views/
│   │   ├── Dashboard.vue         # 统计仪表板
│   │   ├── Records.vue           # 记录列表
│   │   └── Settings.vue          # 设置
│   └── components/
│       ├── Sidebar.vue
│       └── RecordItem.vue
├── package.json
├── vite.config.ts
└── tailwind.config.js
```

## 注意事项

- 在 macOS 上需要授予「辅助功能」权限才能监听键盘事件
- 在 Windows 上需要以管理员权限运行才能监听全局键盘事件
- 数据存储在应用数据目录下的 `arkinput.db` 文件中
