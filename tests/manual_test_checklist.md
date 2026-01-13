# Manual Test Checklist

## Pre-Testing Setup
- [ ] Install Rust toolchain (latest stable)
- [ ] Install Node.js and npm (v18+)
- [ ] Clone repository
- [ ] Navigate to project directory

## Backend Tests (Automated)

### Run Rust Tests
```bash
cd src-tauri
cargo test --verbose
```

**Expected Results:**
- [ ] `test_system_monitor_creation` - PASS
- [ ] `test_get_metrics_returns_valid_data` - PASS
- [ ] `test_cpu_temporal_smoothing` - PASS
- [ ] `test_metrics_refresh` - PASS
- [ ] `test_cleanup_temp_does_not_panic` - PASS
- [ ] `test_pet_creation` - PASS
- [ ] `test_pet_happy_state` - PASS
- [ ] `test_pet_okay_state` - PASS
- [ ] `test_pet_stressed_state` - PASS
- [ ] `test_pet_critical_state_ram` - PASS
- [ ] `test_pet_critical_state_cpu` - PASS
- [ ] `test_pet_critical_state_disk` - PASS
- [ ] `test_pet_state_transitions` - PASS
- [ ] `test_updated_thresholds` - PASS

### Verify Compilation
```bash
cd src-tauri
cargo check
cargo clippy
```

**Expected Results:**
- [ ] No compilation errors
- [ ] No clippy warnings (or only minor ones)

## Frontend Tests (Manual UI Testing)

### Install Dependencies
```bash
cd ui
npm install
```

**Expected Results:**
- [ ] All npm dependencies installed successfully
- [ ] No critical vulnerability warnings

### Development Mode
```bash
cd ui
npm run dev
```

**In parallel terminal:**
```bash
cd src-tauri
cargo tauri dev
```

**Expected Results:**
- [ ] Vite dev server starts on port 5173
- [ ] Tauri window opens
- [ ] Window is 300x300 pixels
- [ ] Window has no frame/decorations
- [ ] Window is transparent (desktop visible through empty areas)
- [ ] Window is always-on-top
- [ ] Window does not appear in taskbar

### UI Component Testing

#### Pet Display
- [ ] Pet emoji is visible and large (120px)
- [ ] Pet emoji shows correct state:
  - Happy: 😊
  - Okay: 😐
  - Stressed: 😰
  - Critical: 🔥

#### Animations
- [ ] **Happy state**: Pet bounces up/down smoothly (2s cycle)
- [ ] **Okay state**: Pet sways left/right (3s cycle)
- [ ] **Stressed state**: Pet breathes/pulses (1.5s cycle)
- [ ] **Critical state**: Pet shakes rapidly (0.5s cycle)
- [ ] **Critical state**: Particle effects appear (fire particles)

#### Hover Interaction
- [ ] Hovering over pet shows stats tooltip
- [ ] Tooltip displays:
  - RAM percentage
  - CPU percentage
  - Disk junk (MB and %)
- [ ] Tooltip appears above pet
- [ ] Tooltip has dark background with good contrast
- [ ] Tooltip disappears when hover ends

#### Click Interaction
- [ ] Clicking pet opens action menu
- [ ] Menu has "Clean Temp Files" button
- [ ] Menu has "Close" button
- [ ] Menu appears in center of window
- [ ] Menu has semi-transparent white background
- [ ] Clicking outside menu closes it

### Action Menu Testing

#### Cleanup Action
- [ ] Click "Clean Temp Files" button
- [ ] Alert shows cleanup result message
- [ ] Alert shows MB deleted (may be 0 due to permissions)
- [ ] Metrics refresh after cleanup
- [ ] Pet state updates if metrics changed

### Background Polling

#### Metric Updates
- [ ] Open console (F12 in dev mode)
- [ ] Observe "metrics-update" events every 5 seconds
- [ ] Pet state updates automatically
- [ ] Pet emoji changes when system load changes
- [ ] No errors in console

### Window Properties

#### Transparency
- [ ] Window background is fully transparent
- [ ] Desktop/other windows visible through transparent areas
- [ ] Only pet emoji and UI elements are opaque

#### Always-on-Top
- [ ] Open another window (browser, notepad, etc.)
- [ ] Pet window stays on top

#### Draggable
- [ ] Click and drag pet to move window
- [ ] Window follows cursor smoothly

### Stress Testing

#### High RAM Scenario
1. Open multiple large applications
2. Wait for metrics to update (5 seconds)
3. [ ] Pet transitions to Okay/Stressed/Critical based on RAM
4. [ ] Tooltip shows increased RAM percentage

#### High CPU Scenario
1. Run CPU-intensive task (video encoding, compilation, etc.)
2. Wait for metrics to update
3. [ ] Pet transitions based on CPU usage
4. [ ] CPU percentage in tooltip reflects load
5. [ ] Temporal smoothing prevents rapid state flicker

#### High Disk Junk Scenario
1. Create many files in TEMP directory
2. Wait for metrics to update
3. [ ] Disk junk MB increases
4. [ ] Pet state reflects disk usage
5. [ ] Cleanup action reduces junk

## Production Build Testing

### Build Application
```bash
cd src-tauri
cargo tauri build
```

**Expected Results:**
- [ ] Build completes without errors
- [ ] `.exe` installer created in `src-tauri/target/release/bundle/nsis/`
- [ ] Standalone `.exe` created in `src-tauri/target/release/`

### Install and Run
- [ ] Run installer
- [ ] Application installs successfully
- [ ] Launch application from Start Menu
- [ ] All UI tests pass in production build
- [ ] Performance is smooth (no lag)
- [ ] Memory usage is reasonable (< 100MB)

## Cross-Version Testing

### Windows 10
- [ ] All tests pass on Windows 10

### Windows 11
- [ ] All tests pass on Windows 11
- [ ] DPI scaling works correctly on high-DPI displays

## Performance Metrics

Record the following:
- [ ] Startup time: _______ seconds
- [ ] Memory usage (idle): _______ MB
- [ ] Memory usage (after 1 hour): _______ MB
- [ ] CPU usage (idle): _______ %
- [ ] CPU usage (during polling): _______ %

## Known Issues / Notes

Document any issues found:

```
Issue 1:
Description:
Steps to reproduce:
Expected:
Actual:

Issue 2:
...
```

## Sign-Off

- [ ] All automated tests pass
- [ ] All manual UI tests pass
- [ ] Production build tested
- [ ] Performance is acceptable
- [ ] No critical bugs found

**Tester Name:** ___________________
**Date:** ___________________
**Build Version:** ___________________
**Windows Version:** ___________________
