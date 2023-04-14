import os
import re

def process_file(filepath):
    with open(filepath, 'r', encoding='utf-8') as file:
        lines = file.readlines()

    correct_path_comment = f"// {filepath}\n"
    rs_comment_pattern = r'^// .+\.rs$'

    action = None
    if len(lines) > 0 and re.match(rs_comment_pattern, lines[0]):
        if lines[0] != correct_path_comment:
            lines[0] = correct_path_comment
            action = "replaced"
    else:
        lines.insert(0, correct_path_comment)
        action = "added"

    with open(filepath, 'w', encoding='utf-8') as file:
        file.writelines(lines)

    if action:
        print(f"{action.title()} path comment in {filepath}")
    else:
        print(f"Path comment unchanged in {filepath}")

def process_directory(root_dir):
    ignored_dirs = {'src/generated', 'target'}
    ignored_prefix = '.'

    for root, dirs, files in os.walk(root_dir):
        dirs[:] = [d for d in dirs if not d.startswith(ignored_prefix) and os.path.relpath(os.path.join(root, d), root_dir).replace('\\', '/') not in ignored_dirs]

        for file in files:
            if file.lower().endswith('.rs'):
                filepath = os.path.join(root, file)
                relative_path = os.path.relpath(filepath, root_dir).replace('\\', '/')
                process_file(relative_path)

def find_root_directory():
    cwd = os.getcwd()
    if os.path.isfile(os.path.join(cwd, 'cargo.toml')):
        return cwd

    while True:
        root_dir = input("Enter the project root directory: ")
        if os.path.isfile(os.path.join(root_dir, 'cargo.toml')):
            return root_dir
        else:
            print("No cargo.toml found in the specified directory. Please try again.")

if __name__ == "__main__":
    project_root = find_root_directory()
    process_directory(project_root)
