# Assets Directory

This directory will contain application assets like tray icons.

## TODO: Create tray icon

For the MVP, you need to create a simple `icon.ico` file. You can:

1. **Quick method**: Use an online ICO converter
   - Create a simple 16x16 or 32x32 PNG image
   - Convert it to ICO format at https://convertio.co/png-ico/
   - Save as `icon.ico` in this directory

2. **Using existing icons**:
   - Use any existing .ico file as a placeholder
   - Windows system icons can be extracted from `C:\Windows\System32\shell32.dll`

3. **Future enhancement**:
   - Create different icons for each PetState (Happy, Okay, Stressed, Critical)
   - Use animated icons for more engaging UI

## File structure (future):
```
assets/
├── README.md (this file)
├── icon.ico (placeholder - single icon)
├── happy.ico (future)
├── okay.ico (future)
├── stressed.ico (future)
└── critical.ico (future)
```

For now, the app will run without an icon (tray will show a default system icon).
