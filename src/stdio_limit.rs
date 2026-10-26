//! Bounds newline-delimited stdio JSON-RPC frames before rmcp's AsyncRwTransport
//! can accumulate an unbounded read_until buffer. This is a local process
//! defense: Streamable HTTP has independent request limits.

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt, DuplexStream};

pub const MAX_STDIO_FRAME_BYTES: usize = 1_048_576;
const CHUNK_SIZE: usize = 8192;
const PIPE_CAPACITY: usize = 64 * 1024;

/// Forward stdin through a small backpressured pipe, failing closed as soon as
/// any individual frame exceeds its limit (even with no terminating newline).
/// The flag allows the outer CLI to exit nonzero without logging input data.
pub fn bounded_reader<R>(reader: R) -> (DuplexStream, Arc<AtomicBool>)
where
    R: AsyncRead + Unpin + Send + 'static,
{
    let (mut writer, bounded) = tokio::io::duplex(PIPE_CAPACITY);
    let too_large = Arc::new(AtomicBool::new(false));
    let rejected = Arc::clone(&too_large);
    tokio::spawn(async move {
        let mut reader = reader;
        let mut buf = [0_u8; CHUNK_SIZE];
        let mut current_line = 0_usize;
        loop {
            let size = match reader.read(&mut buf).await {
                Ok(0) | Err(_) => break,
                Ok(count) => count,
            };
            for byte in &buf[..size] {
                if *byte == b'\n' {
                    current_line = 0;
                } else {
                    current_line += 1;
                    if current_line > MAX_STDIO_FRAME_BYTES {
                        rejected.store(true, Ordering::Release);
                        // Never reflect untrusted request bytes in the logs.
                        eprintln!("Rejected oversized MCP stdio frame (1 MiB maximum).");
                        return;
                    }
                }
            }
            if writer.write_all(&buf[..size]).await.is_err() {
                break;
            }
        }
    });
    (bounded, too_large)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn forwards_multiple_valid_frames_without_modifying_bytes() {
        let (mut writer, input) = tokio::io::duplex(4096);
        let sender = tokio::spawn(async move {
            writer.write_all(b"{\"id\":1}\n{\"id\":2}\n").await.unwrap();
        });
        let (mut bounded, overflow) = bounded_reader(input);
        let mut bytes = Vec::new();
        bounded.read_to_end(&mut bytes).await.unwrap();
        sender.await.unwrap();
        assert_eq!(bytes, b"{\"id\":1}\n{\"id\":2}\n");
        assert!(!overflow.load(Ordering::Acquire));
    }

    #[tokio::test]
    async fn terminates_oversized_unterminated_frame_without_buffering_it() {
        let (mut writer, input) = tokio::io::duplex(8192);
        let sender = tokio::spawn(async move {
            writer.write_all(b"{}\n").await.unwrap();
            let _ = writer.write_all(&vec![b'x'; MAX_STDIO_FRAME_BYTES + 8192]).await;
        });
        let (mut bounded, overflow) = bounded_reader(input);
        let mut bytes = Vec::new();
        bounded.read_to_end(&mut bytes).await.unwrap();
        sender.await.unwrap();
        assert!(overflow.load(Ordering::Acquire));
        assert!(bytes.starts_with(b"{}\n"));
        // One input frame cannot grow the decoder buffer beyond its cap,
        // and the relay itself never buffers more than a fixed 8-KiB chunk.
        assert!(bytes.len() <= MAX_STDIO_FRAME_BYTES + 1);
    }
}
