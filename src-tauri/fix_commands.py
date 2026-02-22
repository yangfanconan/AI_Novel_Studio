#!/usr/bin/env python3
import re

with open('src/commands.rs', 'r') as f:
    content = f.read()

# 替换函数签名中的 db_path: State<'_, DbPath> 为 app: AppHandle
content = re.sub(
    r'pub async fn (\w+)\([^)]*db_path: State<'"'"'_, DbPath>([^)]*)',
    r'pub async fn \1(\n    app: AppHandle,\2',
    content
)

# 替换函数体中的 db_path.as_path() 为 get_db_path(&app)?
content = re.sub(
    r'get_connection\(db_path\.as_path\(\)\)',
    r'get_connection(&get_db_path(&app)?)',
    content
)

with open('src/commands.rs', 'w') as f:
    f.write(content)

print("Fixed commands.rs")
