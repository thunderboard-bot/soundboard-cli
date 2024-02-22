use crate::api::{get_client, post_client};
use crate::config::CustomError;

#[derive(Debug, serde::Deserialize)]
pub struct SoundDto {
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "submittedBy"))]
    pub submitted_by: String
}

pub async fn add_sound(sound_name: String, sound_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let sound_file_binary = std::fs::read(sound_file_path)?;
    
    use base64::{Engine as _, engine::general_purpose};
    let b64_sound_file = general_purpose::STANDARD.encode(sound_file_binary);
    
    let form = format!(
        r#"{{
            "name": "{}",
            "file": "{}"
        }}"#,
        sound_name,
        b64_sound_file
    );
    let client = post_client("sound".to_string(), form)?;
    let response = client.send().await?;
    if response.status() != 200
    {
        return Err(
            Box::new(
                CustomError(
                    format!("Failed to upload sound: {}", response.text().await?)
                )
            )
        )
    }
    
    let sound_dto = response.json::<SoundDto>().await?;
    println!("Successfully uploaded! Sound is playable via 'soundboard-cli play {}'", sound_dto.name);
    
    Ok(())
}