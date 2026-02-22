#!/usr/bin/env python3
import re

with open('src/commands.rs', 'r') as f:
    content = f.read()

# Replace each occurrence of the old pattern
old_pattern = r'''    let app_data_dir = app\.path\(\)\.app_data_dir\(\)
        \.map_err\(\|e\| \{
            logger\.error\(&format!\("Failed to get app data directory: \{\}", e\)\);
            e\.to_string\(\)
        \}\)?;
    let db_path = app_data_dir\.join\("novel_studio\.db"\);'''

new_code = '''    let db_path = get_db_path(&app)?;'''

content = re.sub(old_pattern, new_code, content)

with open('src/commands.rs', 'w') as f:
    f.write(content)

print("Fixed all database path references successfully")
