# Task Tracker CLI

A simple, fast command-line tool for managing tasks—perfect for keeping track of to-dos, tracking progress, and staying organised in your terminal.

## 🚀 Features

- Create new tasks with a description.

- Update existing tasks by ID.

- Delete tasks that are no longer needed.

- Mark tasks as In Progress or Done.

- List tasks with optional status filtering.

- Stores tasks locally in a JSON file (no external database required).

- Minimal dependencies — built with native file system handling and straightforward error handling.

## ⚙️ Configuration

Task Tracker CLI uses a Config.toml file located in the project root to determine where tasks are stored.

### Example Config.toml

```toml
[storage]
directory = "/home/my-user/.config/task-tracker"
```

### Important
The directory you specify must already exist.
If it doesn’t, create it before running the CLI:
```bash
mkdir -p /home/my-user/.config/task-tracker
```

### Defaults
If no Config.toml is found, Task Tracker CLI stores tasks in:
```bash
./tasks.json
```

## 📦 Installation

```bash
# Clone the repository
git clone https://github.com/Sarang681/task-tracker-cli.git
cd task-tracker-cli

# Build the binary (requires Rust & Cargo)
cargo build --release

# (Optional) Install it globally
cargo install --path .
```

## ✅ Usage

After installation you’ll have a task-tracker executable. Here are some common commands:

### Add a task
```bash
task-tracker add "Write project README"
```

### Update a task
```bash
task-tracker update 3 "Review and refine documentation"
```

### Delete a task
```bash
task-tracker delete 5
```

### Mark a task’s status
```bash
task-tracker mark 2 in-progress
task-tracker mark 2 done
```

### List tasks
```bash
task-tracker list
task-tracker list in-progress
task-tracker list done
```

## 🧩 Command Reference

- **Add**: Adds a new task.

- **Update**: Updates an existing task’s description by its ID.

- **Delete**: Deletes a task by ID.

- **Mark**: Changes the status of a task (to In Progress or Done).

- **List**: Lists all tasks, optionally filtered by status.

## 🗄️ How It Works

The tool saves tasks locally in a JSON file (as configured). Each task record includes a unique ID, description, and status (Todo, In Progress, Done). Because everything’s stored locally with minimal overhead, you can easily move the file, back it up, or integrate the CLI into shell scripts.

## 🌐 Project Reference
This CLI is inspired by and aligned with the Task Tracker project from Roadmap.sh:
https://roadmap.sh/projects/task-tracker

## 🧪 Contributing

1. Contributions are welcome! If you’d like to contribute:

2. Fork the repository.

3. Create a new branch for your feature or fix.

4. Submit a pull request with clear description & tests (if applicable).

## 📜 License

This project is released under the MIT License.
See the [LICENSE](LICENSE) file for details.

## 🤝 Acknowledgments

Thanks to everyone who uses, stars, or contributes to this project. Feedback is appreciated!
