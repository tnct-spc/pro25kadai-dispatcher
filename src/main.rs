use std::process::Stdio;
use tokio::process::{Command, Child};

/// 実行するコマンドの設定
struct CommandConfig {
    name: &'static str,
    directory: &'static str,
    setup_script: Option<&'static str>,
    command: &'static str,
    args: Vec<&'static str>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = vec![
        CommandConfig {
            name: "RealSense",
            directory: "~/realsense_ws",
            setup_script: Some("install/local_setup.bash"),
            command: "ros2",
            args: vec!["launch", "realsense_camera", "rs_launch.py", "initial_reset:=true"],
        },
        CommandConfig {
            name: "ROS Bridge",
            directory: ".",
            setup_script: None,
            command: "ros2",
            args: vec!["launch", "rosbridge_server", "rosbridge_websocket_launch.xml"],
        },
        CommandConfig {
            name: "LiDAR (MID360)",
            directory: "~/slam_ws",
            setup_script: Some("install/setup.bash"),
            command: "ros2",
            args: vec!["launch", "livox_ros_driver2", "msg_MID360_launch.py"],
        },
        CommandConfig {
            name: "SLAM (FAST_LIO)",
            directory: "~/slam_ws",
            setup_script: Some("install/setup.bash"),
            command: "ros2",
            args: vec!["launch", "fast_lio", "mapping.launch.py"],
        },
        CommandConfig {
            name: "GPS (ZED-F9P)",
            directory: ".",
            setup_script: None,
            command: "ros2",
            args: vec!["launch", "ublox_gps", "ublox_gps_node-launch.py"],
        },
    ];

    let mut children: Vec<(String, Child)> = Vec::new();
    for config in commands {
        // setup.bashスクリプトを実行してからコマンドを実行
        let child = if let Some(setup_script) = config.setup_script {
            // bashでsetup.bashを実行してから本来のコマンドを実行
            let combined_command = format!(
                "source {} && {} {}",
                setup_script,
                config.command,
                config.args.join(" ")
            );

            Command::new("bash")
                .args(["-c", &combined_command])
                .current_dir(&config.directory)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null())
                .spawn()?
        } else {
            // setup.bashが指定されていない場合は従来通り
            Command::new(&config.command)
                .args(&config.args)
                .current_dir(&config.directory)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .stdin(Stdio::null())
                .spawn()?
        };

        println!("Running {} [{:>5}]: {} {}", config.name, child.id().unwrap_or(0), config.command, config.args.join(" "));
        children.push((config.name.to_string(), child));
    }

    tokio::signal::ctrl_c().await?;
    println!("Received SIGINT. Stopping...");

    for (name, mut child) in children {
        match child.kill().await {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to stop {}: {}", name, e);
            }
        }
    }

    Ok(())
}
