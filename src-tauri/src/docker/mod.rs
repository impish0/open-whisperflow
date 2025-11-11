use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions,
    StartContainerOptions, StopContainerOptions,
};
use bollard::models::{ContainerStateStatusEnum, DeviceRequest, HostConfig, PortBinding};
use bollard::Docker;
use std::collections::HashMap;
use std::default::Default;

use crate::error::{AppError, Result};

const FASTER_WHISPER_IMAGE: &str = "fedirz/faster-whisper-server:latest-cuda";
const FASTER_WHISPER_IMAGE_CPU: &str = "fedirz/faster-whisper-server:latest-cpu";
const CONTAINER_NAME: &str = "open-whisperflow-whisper";
const WHISPER_PORT: u16 = 8000;

/// Docker client wrapper for managing faster-whisper containers
pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    /// Create a new Docker client
    pub fn new() -> Result<Self> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| AppError::Docker(format!("Failed to connect to Docker: {}", e)))?;

        Ok(Self { docker })
    }

    /// Check if Docker is available and running
    pub async fn is_available(&self) -> bool {
        self.docker.ping().await.is_ok()
    }

    /// Detect if NVIDIA GPU is available for CUDA support
    pub async fn has_nvidia_gpu(&self) -> bool {
        // Check if NVIDIA runtime is available
        if let Ok(info) = self.docker.info().await {
            if let Some(runtimes) = info.runtimes {
                return runtimes.contains_key("nvidia");
            }
        }

        // Fallback: check if nvidia-smi command exists
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("nvidia-smi").output() {
                return output.status.success();
            }
        }

        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            if let Ok(output) = Command::new("nvidia-smi.exe").output() {
                return output.status.success();
            }
        }

        false
    }

    /// Get the appropriate image based on GPU availability
    pub async fn get_image_name(&self) -> String {
        if self.has_nvidia_gpu().await {
            log::info!("NVIDIA GPU detected, using CUDA image");
            FASTER_WHISPER_IMAGE.to_string()
        } else {
            log::info!("No NVIDIA GPU detected, using CPU image");
            FASTER_WHISPER_IMAGE_CPU.to_string()
        }
    }

    /// Check if the faster-whisper container exists
    pub async fn container_exists(&self) -> Result<bool> {
        let filters = HashMap::from([("name".to_string(), vec![CONTAINER_NAME.to_string()])]);

        let options = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };

        let containers = self
            .docker
            .list_containers(Some(options))
            .await
            .map_err(|e| AppError::Docker(format!("Failed to list containers: {}", e)))?;

        Ok(!containers.is_empty())
    }

    /// Check if the container is running
    pub async fn is_container_running(&self) -> Result<bool> {
        if !self.container_exists().await? {
            return Ok(false);
        }

        let inspect = self
            .docker
            .inspect_container(CONTAINER_NAME, None)
            .await
            .map_err(|e| AppError::Docker(format!("Failed to inspect container: {}", e)))?;

        if let Some(state) = inspect.state {
            if let Some(status) = state.status {
                return Ok(status == ContainerStateStatusEnum::RUNNING);
            }
        }

        Ok(false)
    }

    /// Pull the faster-whisper Docker image
    pub async fn pull_image(&self) -> Result<()> {
        let image = self.get_image_name().await;
        log::info!("Pulling Docker image: {}", image);

        use bollard::image::CreateImageOptions;
        use futures_util::stream::StreamExt;

        let options = CreateImageOptions {
            from_image: image.as_str(),
            ..Default::default()
        };

        let mut stream = self.docker.create_image(Some(options), None, None);

        while let Some(result) = stream.next().await {
            match result {
                Ok(info) => {
                    if let Some(status) = info.status {
                        log::debug!("Pull status: {}", status);
                    }
                    if let Some(progress) = info.progress {
                        log::debug!("Progress: {}", progress);
                    }
                }
                Err(e) => {
                    return Err(AppError::Docker(format!("Failed to pull image: {}", e)));
                }
            }
        }

        log::info!("Successfully pulled Docker image: {}", image);
        Ok(())
    }

    /// Create and start the faster-whisper container
    pub async fn start_container(&self) -> Result<()> {
        // Check if container already exists
        if self.container_exists().await? {
            // If it exists but is not running, start it
            if !self.is_container_running().await? {
                log::info!("Starting existing container");
                self.docker
                    .start_container(CONTAINER_NAME, None::<StartContainerOptions<String>>)
                    .await
                    .map_err(|e| AppError::Docker(format!("Failed to start container: {}", e)))?;
                return Ok(());
            } else {
                log::info!("Container is already running");
                return Ok(());
            }
        }

        // Pull image if needed
        self.pull_image().await?;

        let image = self.get_image_name().await;
        let has_gpu = self.has_nvidia_gpu().await;

        log::info!("Creating faster-whisper container with GPU: {}", has_gpu);

        // Port binding for the API
        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            format!("{}/tcp", WHISPER_PORT),
            Some(vec![PortBinding {
                host_ip: Some("127.0.0.1".to_string()),
                host_port: Some(WHISPER_PORT.to_string()),
            }]),
        );

        // Host config with GPU support if available
        let mut host_config = HostConfig {
            port_bindings: Some(port_bindings),
            ..Default::default()
        };

        // Add GPU support if NVIDIA GPU is available
        if has_gpu {
            host_config.device_requests = Some(vec![DeviceRequest {
                driver: Some("nvidia".to_string()),
                count: Some(-1), // -1 means all GPUs
                device_ids: None,
                capabilities: Some(vec![vec!["gpu".to_string()]]),
                options: None,
            }]);
        }

        // Container configuration
        let config = Config {
            image: Some(image.clone()),
            host_config: Some(host_config),
            env: Some(vec![
                // Set default model (can be overridden per request)
                "WHISPER__MODEL=base".to_string(),
                // Enable CUDA if GPU is available
                format!("WHISPER__DEVICE={}", if has_gpu { "cuda" } else { "cpu" }),
            ]),
            ..Default::default()
        };

        let options = CreateContainerOptions {
            name: CONTAINER_NAME,
            platform: None,
        };

        // Create container
        self.docker
            .create_container(Some(options), config)
            .await
            .map_err(|e| AppError::Docker(format!("Failed to create container: {}", e)))?;

        log::info!("Container created, starting...");

        // Start container
        self.docker
            .start_container(CONTAINER_NAME, None::<StartContainerOptions<String>>)
            .await
            .map_err(|e| AppError::Docker(format!("Failed to start container: {}", e)))?;

        log::info!("faster-whisper container started successfully");
        Ok(())
    }

    /// Stop the faster-whisper container
    pub async fn stop_container(&self) -> Result<()> {
        if !self.container_exists().await? {
            log::info!("Container does not exist, nothing to stop");
            return Ok(());
        }

        if !self.is_container_running().await? {
            log::info!("Container is not running, nothing to stop");
            return Ok(());
        }

        log::info!("Stopping faster-whisper container");

        let options = StopContainerOptions { t: 10 }; // 10 second timeout

        self.docker
            .stop_container(CONTAINER_NAME, Some(options))
            .await
            .map_err(|e| AppError::Docker(format!("Failed to stop container: {}", e)))?;

        log::info!("Container stopped successfully");
        Ok(())
    }

    /// Remove the faster-whisper container
    pub async fn remove_container(&self) -> Result<()> {
        if !self.container_exists().await? {
            log::info!("Container does not exist, nothing to remove");
            return Ok(());
        }

        // Stop container if running
        if self.is_container_running().await? {
            self.stop_container().await?;
        }

        log::info!("Removing faster-whisper container");

        let options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        self.docker
            .remove_container(CONTAINER_NAME, Some(options))
            .await
            .map_err(|e| AppError::Docker(format!("Failed to remove container: {}", e)))?;

        log::info!("Container removed successfully");
        Ok(())
    }

    /// Get the container API URL
    pub fn get_api_url(&self) -> String {
        format!("http://127.0.0.1:{}", WHISPER_PORT)
    }

    /// Wait for container to be ready by checking health
    pub async fn wait_for_ready(&self, max_attempts: u32) -> Result<()> {
        log::info!("Waiting for container to be ready...");

        for attempt in 1..=max_attempts {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            // Check if container is still running
            if !self.is_container_running().await? {
                return Err(AppError::Docker(
                    "Container stopped unexpectedly".to_string(),
                ));
            }

            // Try to hit the health endpoint
            let url = format!("{}/health", self.get_api_url());
            if let Ok(response) = reqwest::get(&url).await {
                if response.status().is_success() {
                    log::info!("Container is ready!");
                    return Ok(());
                }
            }

            log::debug!("Container not ready yet (attempt {}/{})", attempt, max_attempts);
        }

        Err(AppError::Docker(format!(
            "Container did not become ready after {} attempts",
            max_attempts
        )))
    }
}
