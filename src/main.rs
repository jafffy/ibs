use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use std::{fs, process::Command};

#[derive(Parser)]
#[command(name = "ibs")]
#[command(about = "iOS Build System - Command line tool for iOS project management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup a new iOS project
    Setup {
        /// Name of the project
        project_name: String,
        /// Apple Developer Team ID
        team_id: String,
    },
    /// Build the iOS project
    Build {
        /// Build configuration (Debug/Release)
        #[arg(default_value = "Debug")]
        configuration: String,
        /// Development Team ID
        #[arg(short, long)]
        team_id: Option<String>,
        /// Project scheme name (defaults to project name)
        #[arg(short, long)]
        scheme: Option<String>,
    },
}

fn run_command(command: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(command)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute command: {} {:?}", command, args))?;

    if !status.success() {
        anyhow::bail!("Command failed: {} {:?}", command, args);
    }
    Ok(())
}

fn setup_project(project_name: &str, team_id: &str) -> Result<()> {
    println!("{} iOS project: {}", "Setting up".green(), project_name);
    println!("{} {}", "Team ID:".yellow(), team_id);

    // Create project directory
    fs::create_dir_all(project_name)?;
    std::env::set_current_dir(project_name)?;

    // Create project.yml for xcodegen
    println!("\n{} Creating project configuration...", "→".blue());
    let project_yml = format!(
        r#"name: {}
options:
  bundleIdPrefix: com.example
  deploymentTarget:
    iOS: 15.0
  developmentTeam: {}
  xcodeVersion: "15.0"
  createIntermediateGroups: true
targets:
  {}:
    type: application
    platform: iOS
    sources:
      - path: Sources
    settings:
      base:
        INFOPLIST_FILE: Sources/Info.plist
        PRODUCT_BUNDLE_IDENTIFIER: com.example.{}
        DEVELOPMENT_TEAM: {}
        CODE_SIGN_STYLE: Automatic
schemes:
  {}:
    build:
      targets:
        {}: all
    run:
      config: Debug
    test:
      config: Debug
    profile:
      config: Release
    analyze:
      config: Debug
    archive:
      config: Release
"#,
        project_name, team_id, project_name, project_name, team_id, project_name, project_name
    );
    fs::write("project.yml", project_yml)?;

    // Create basic project structure
    println!("\n{} Creating project structure...", "→".blue());
    fs::create_dir_all("Sources")?;

    // Create Info.plist
    let info_plist = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>$(DEVELOPMENT_LANGUAGE)</string>
    <key>CFBundleExecutable</key>
    <string>$(EXECUTABLE_NAME)</string>
    <key>CFBundleIdentifier</key>
    <string>$(PRODUCT_BUNDLE_IDENTIFIER)</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>$(PRODUCT_NAME)</string>
    <key>CFBundlePackageType</key>
    <string>$(PRODUCT_BUNDLE_PACKAGE_TYPE)</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0</string>
    <key>CFBundleVersion</key>
    <string>1</string>
    <key>LSRequiresIPhoneOS</key>
    <true/>
    <key>UILaunchStoryboardName</key>
    <string>LaunchScreen</string>
    <key>UIRequiredDeviceCapabilities</key>
    <array>
        <string>armv7</string>
    </array>
    <key>UISupportedInterfaceOrientations</key>
    <array>
        <string>UIInterfaceOrientationPortrait</string>
        <string>UIInterfaceOrientationLandscapeLeft</string>
        <string>UIInterfaceOrientationLandscapeRight</string>
    </array>
</dict>
</plist>"#;
    fs::write("Sources/Info.plist", info_plist)?;

    // Create LaunchScreen.storyboard
    let launch_screen = r#"<?xml version="1.0" encoding="UTF-8"?>
<document type="com.apple.InterfaceBuilder3.CocoaTouch.Storyboard.XIB" version="3.0" toolsVersion="21701" targetRuntime="iOS.CocoaTouch" propertyAccessControl="none" useAutolayout="YES" launchScreen="YES" useTraitCollections="YES" useSafeAreas="YES" colorMatched="YES" initialViewController="01J-lp-oVM">
    <device id="retina6_12" orientation="portrait" appearance="light"/>
    <dependencies>
        <plugIn identifier="com.apple.InterfaceBuilder.IBCocoaTouchPlugin" version="21701"/>
        <capability name="Safe area layout guides" minToolsVersion="9.0"/>
        <capability name="System colors in document resources" minToolsVersion="11.0"/>
        <capability name="documents saved in the Xcode 8 format" minToolsVersion="8.0"/>
    </dependencies>
    <scenes>
        <!--View Controller-->
        <scene sceneID="EHf-IW-A2E">
            <objects>
                <viewController id="01J-lp-oVM" sceneMemberID="viewController">
                    <view key="view" contentMode="scaleToFill" id="Ze5-6b-2t3">
                        <rect key="frame" x="0.0" y="0.0" width="393" height="852"/>
                        <autoresizingMask key="autoresizingMask" widthSizable="YES" heightSizable="YES"/>
                        <viewLayoutGuide key="safeArea" id="6Tk-OE-BBY"/>
                        <color key="backgroundColor" systemColor="systemBackgroundColor"/>
                    </view>
                </viewController>
                <placeholder placeholderIdentifier="IBFirstResponder" id="iYj-Kq-Ea1" userLabel="First Responder" sceneMemberID="firstResponder"/>
            </objects>
            <point key="canvasLocation" x="53" y="375"/>
        </scene>
    </scenes>
    <resources>
        <systemColor name="systemBackgroundColor">
            <color white="1" alpha="1" colorSpace="custom" customColorSpace="genericGamma22GrayColorSpace"/>
        </systemColor>
    </resources>
</document>"#;
    fs::write("Sources/LaunchScreen.storyboard", launch_screen)?;

    // Create AppDelegate.swift
    let app_delegate = r#"import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        window = UIWindow(frame: UIScreen.main.bounds)
        window?.rootViewController = UIViewController()
        window?.makeKeyAndVisible()
        return true
    }

    func application(
        _ application: UIApplication,
        configurationForConnecting connectingSceneSession: UISceneSession,
        options: UIScene.ConnectionOptions
    ) -> UISceneConfiguration {
        return UISceneConfiguration(name: "Default Configuration", sessionRole: connectingSceneSession.role)
    }
}
"#;
    fs::write("Sources/AppDelegate.swift", app_delegate)?;

    // Generate Xcode project
    println!("\n{} Generating Xcode project...", "→".blue());
    run_command("xcodegen", &["generate"])?;

    // Initialize git repository
    println!("\n{} Initializing git repository...", "→".blue());
    run_command("git", &["init"])?;

    // Create .gitignore
    let gitignore = r#".DS_Store
xcuserdata/
*.xcodeproj/*
!*.xcodeproj/project.pbxproj
!*.xcodeproj/xcshareddata/
!*.xcodeproj/project.xcworkspace/
!*.xcworkspace/contents.xcworkspacedata
**/xcshareddata/WorkspaceSettings.xcsettings
DerivedData/
.swiftpm/
"#;
    fs::write(".gitignore", gitignore)?;

    // Add files to git
    run_command("git", &["add", "."])?;
    run_command("git", &["commit", "-m", "Initial commit"])?;

    println!("\n{} Project setup completed successfully!", "Success:".green());
    println!(
        "\nTo get started:\n{} cd {}\n{} open {}.xcodeproj",
        ">".blue(),
        project_name,
        ">".blue(),
        project_name
    );

    Ok(())
}

fn find_xcode_project() -> Result<(String, std::path::PathBuf)> {
    let current_dir = std::env::current_dir()?;
    
    // First, check the current directory for .xcodeproj
    let entries = fs::read_dir(&current_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.extension().and_then(|s| s.to_str()) == Some("xcodeproj") {
            let scheme_name = path.file_stem()
                .and_then(|n| n.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid project name"))?
                .to_string();
            return Ok((scheme_name, current_dir));
        }
    }
    
    // If not found, check parent directory
    if let Some(parent_dir) = current_dir.parent() {
        let entries = fs::read_dir(parent_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() && path.extension().and_then(|s| s.to_str()) == Some("xcodeproj") {
                let scheme_name = path.file_stem()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| anyhow::anyhow!("Invalid project name"))?
                    .to_string();
                return Ok((scheme_name, parent_dir.to_path_buf()));
            }
        }
    }
    
    anyhow::bail!("No Xcode project found in current or parent directory")
}

fn build_project(configuration: &str, team_id: Option<&str>, scheme: Option<&str>) -> Result<()> {
    // Find Xcode project and change to its directory
    let (default_scheme, project_dir) = find_xcode_project()?;
    std::env::set_current_dir(&project_dir)?;

    // Create and prepare intermediate directories
    println!("{} Creating build directories...", "→".blue());
    fs::create_dir_all("intermediate/logs")?;
    fs::create_dir_all("intermediate/build")?;
    
    // Mark the build directory as deletable by Xcode's build system
    run_command("xattr", &["-w", "com.apple.xcode.CreatedByBuildSystem", "true", "intermediate/build"])?;

    // Use provided scheme or default to found project name
    let scheme_name = scheme.unwrap_or(&default_scheme);

    println!("{} Building for iOS device...", "→".blue());
    println!("Configuration: {}", configuration);
    println!("Scheme: {}", scheme_name);
    if let Some(team) = team_id {
        println!("Team ID: {}", team);
    }

    // Clean build
    println!("\n{} Cleaning previous build...", "→".blue());
    let clean_args = vec![
        "clean",
        "-scheme",
        scheme_name,
        "-configuration",
        configuration,
        "-sdk",
        "iphoneos",
        "CONFIGURATION_BUILD_DIR=intermediate/build",
        "ONLY_ACTIVE_ARCH=NO",
    ];

    run_command("xcodebuild", &clean_args)?;

    // Build
    println!("\n{} Building project...", "→".blue());
    let mut build_args = vec![
        "build",
        "-scheme",
        scheme_name,
        "-configuration",
        configuration,
        "-sdk",
        "iphoneos",
        "-allowProvisioningUpdates",
        "CONFIGURATION_BUILD_DIR=intermediate/build",
        "ONLY_ACTIVE_ARCH=NO",
    ];

    if let Some(team) = team_id {
        build_args.push("DEVELOPMENT_TEAM");
        build_args.push(team);
        build_args.push("CODE_SIGN_STYLE=Automatic");
    }

    run_command("xcodebuild", &build_args)?;

    println!("\n{} Build completed successfully!", "Success:".green());
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Setup { project_name, team_id } => {
            setup_project(&project_name, &team_id)?;
        }
        Commands::Build { configuration, team_id, scheme } => {
            build_project(&configuration, team_id.as_deref(), scheme.as_deref())?;
        }
    }

    Ok(())
}
