# iOS Build System (ibs)

A command-line tool for iOS project management, building, and deployment.

## Prerequisites

- macOS with Xcode installed
- iOS device or simulator for testing
- `ios-deploy` tool (install via `brew install ios-deploy`)

## Installation

```bash
cargo install --path .
```

## Usage

### Create a new iOS project

```bash
ibs setup PROJECT_NAME --team-id YOUR_TEAM_ID
```

### Build the project

```bash
ibs build --team-id YOUR_TEAM_ID
```

### Deploy to Device

```bash
ibs deploy device --team-id YOUR_TEAM_ID
```

### Deploy to Simulator

First, list available simulators:
```bash
xcrun simctl list devices
```

Then deploy to a specific simulator:
```bash
ibs deploy simulator --simulator-id SIMULATOR_ID
```

## TODO

### App Execution
- [ ] Add direct app launch functionality for iOS devices
- [ ] Support debugging on device
- [ ] Add logging and crash reporting
- [ ] Support background app launch
- [ ] Add app state monitoring

### Known Limitations
1. App execution on device:
   - Currently, the tool only installs the app on the device
   - You need to manually launch the app from your device
   - Future versions will support direct app launch and debugging

2. Device deployment:
   - Make sure your device is connected and trusted
   - You may need to open Xcode and let it install device support files
   - Trust the developer certificate on your device (Settings → General → Device Management)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 