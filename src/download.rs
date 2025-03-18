use {
    crate::Args,
    reqwest,
    std::{
        path::Path,
        sync::Arc,
        time::Duration,
    },
    tokio::{
        fs::File,
        io::AsyncWriteExt,
        time::sleep,
    },
};

async fn download(
    url: &str,
    flags: Arc<Args>,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await?;
    let total_size = response
        .content_length()
        .unwrap_or(0);

    let filename = flags
        .output
        .clone()
        .unwrap_or_else(|| {
            Path::new(url)
                .file_name()
                .map(|f| {
                    f.to_string_lossy()
                        .into_owned()
                })
                .unwrap_or_else(|| "downloaded_file".to_string())
        });

    let save_path = Path::new(&flags.path).join(&filename);
    let mut file = File::create(save_path).await?;

    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream
        .next()
        .await
    {
        let chunk = chunk?;

        // Rate limiting logic
        if let Some(rate_limit) = flags.rate_limit {
            let sleep_duration = Duration::from_millis(
                (chunk.len() as f64 / rate_limit as f64 * 1000.0) as u64,
            );
            sleep(sleep_duration).await;
        }

        file.write_all(&chunk)
            .await?;
        downloaded += chunk.len() as u64;

        // Progress tracking could be added here
    }

    Ok(())
}
