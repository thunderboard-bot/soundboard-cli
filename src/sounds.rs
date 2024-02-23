use crate::api::{get_client, post_client,  post_without_body_client};
use crate::custom_error::CustomError;

#[derive(Debug, serde::Deserialize)]
pub struct Sound {
    pub id: String,
    pub name: String,
    #[serde(rename(deserialize = "submittedBy"))]
    pub submitted_by: String
}
pub async fn add_sound(sound_name: String, sound_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let sound_file_binary = std::fs::read(sound_file_path.clone())?;

    use base64::{Engine as _, engine::general_purpose};
    let b64_sound_file = general_purpose::STANDARD.encode(sound_file_binary);
    let filename = sound_file_path.split("/").last().unwrap();

    let form = format!(
        r#"{{
            "name": "{}",
            "fileName": "{}",
            "data": "{}"
        }}"#,
        sound_name,
        filename,
        b64_sound_file
    );
    let client = post_client("api/sound".to_string(), form)?;
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

    let sound = response.json::<Sound>().await?;
    println!("Successfully uploaded! Sound is playable via 'soundboard-cli play {}'", sound.id);

    Ok(())
}

pub async fn play_sound(sound_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = post_without_body_client(format!("api/sound/{}/play", sound_id.clone()))?;
    let response = client.send().await?;
    if response.status() != 200
    {
        return Err(
            Box::new(
                CustomError(
                    format!("Failed to play sound: {}", response.text().await?)
                )
            )
        )
    }

    println!("Playing sound {}", sound_id);

    Ok(())
}

pub async fn list_sounds() -> Result<(), Box<dyn std::error::Error>> {
    let client = get_client("api/sound".to_string())?;
    let response = client.send().await?;
    if response.status() != 200
    {
        return Err(
            Box::new(
                CustomError(
                    format!("Failed to list sounds: {}", response.text().await?)
                )
            )
        )
    }

    let sound_dto = response.json::<Vec<Sound>>().await?;
    for sound in sound_dto
    {
        println!("{} | added by {}", sound.id, sound.submitted_by);
    }

    Ok(())
}