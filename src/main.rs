use async_stream::stream;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_transcribestreaming::model::{
    AudioEvent, AudioStream, LanguageCode, MediaEncoding, TranscriptResultStream,
};
use aws_sdk_transcribestreaming::types::Blob;
use aws_sdk_transcribestreaming::{Client, Error, Region};
use bytes::BufMut;
use std::time::Duration;
use cpal::StreamConfig;
use cpal::traits::{DeviceTrait, HostTrait};


const CHUNK_SIZE: usize = 8192;


#[tokio::main]
async fn main() -> Result<(), Error> {

    let audio_file = "testing-123.wav";

    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env()
        .region(region_provider).load().await;

    let client = Client::new(&shared_config);

    /*let input_stream = stream! {
        let pcm = pcm_data(&*audio_file);

        for chunk in pcm.chunks(CHUNK_SIZE) {
            // Sleeping isn't necessary, but emphasizes the streaming aspect of this
            tokio::time::sleep(Duration::from_millis(100)).await;
            yield Ok(AudioStream::AudioEvent(AudioEvent::builder().audio_chunk(Blob::new(chunk)).build()));
        }
    };*/

    let host = cpal::default_host();
    let device = host.default_input_device().expect("failed to find a default input device");

    println!("Using device: {}", device.name()?);


    // TODO create input_stream from device
    let input_stream = stream! {
        /*let config = StreamConfig::new()
            .sample_rate(device.default_sample_rate())
            .channel_count(1)
            .buffer_size(CHUNK_SIZE)
            .build();*/
        StreamConfig {

        }

        let stream = device.build_input_stream(config).await.expect("failed to create input stream");

        while let Ok(data) = stream.read_frame().await {
            yield Ok(AudioStream::AudioEvent(AudioEvent::builder().audio_chunk(Blob::new(data.data)).build()));
        }
    };

    let mut full_message = String::new();
    while let Some(event) = output.transcript_result_stream.recv().await? {
        match event {
            TranscriptResultStream::TranscriptEvent(transcript_event) => {
                let transcript = transcript_event.transcript.unwrap();
                for result in transcript.results.unwrap_or_else(|| Vec::new()) {
                    if result.is_partial {
                        // if verbose {
                            println!("Partial: {:?}", result);
                        // }
                    } else {
                        let first_alternative = &result.alternatives.as_ref().unwrap()[0];
                        full_message += first_alternative.transcript.as_ref().unwrap();
                        full_message.push('\n');
                    }
                }
            }
            otherwise => panic!("received unexpected event type: {:?}", otherwise),
        }
    }
    println!("\nFully transcribed message:\n\n{}", full_message);

    Ok(())
}


fn pcm_data(audio_file: &str) -> Vec<u8> {

    let reader = hound::WavReader::open(audio_file).unwrap();
    let samples_result: hound::Result<Vec<i16>> = reader.into_samples::<i16>().collect();

    let mut pcm: Vec<u8> = Vec::new();
    for sample in samples_result.unwrap() {
        pcm.put_i16_le(sample);
    }
    pcm
}
