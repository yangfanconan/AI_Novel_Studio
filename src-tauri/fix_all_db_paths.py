#!/usr/bin/env python3
import re

with open('src/commands.rs', 'r') as f:
    content = f.read()

# Pattern to match the old database path getting code
pattern = r'''let app_data_dir = app\.path\(\)\.app_data_dir\(\)\s*\.map_err\(\|e\| \{\s*logger\.error\(&format!\("Failed to get app data directory: \{\}", e\)\);\s*e\.to_string\(\)\s*\}\)?;\s*let db_path = app_data_dir\.join\("novel_studio\.db"\);'''

replacement = '''let db_path = get_db_path(&app)?;'''

content = re.sub(pattern, replacement, content)

with open('src/commands.rs', 'w') as f:
    f.write(content)

print("Fixed all database path references")
