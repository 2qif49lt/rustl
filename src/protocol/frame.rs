use crate::utils::field_len;
use crate::utils::field_offset;

use super::{header_len, Header};

use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::{Decoder, Encoder, Framed, LengthDelimitedCodec};

use bytes::{Buf, BufMut, BytesMut};

/*
                INPUT

+-- cmd ---+-- size --+-- opt --+----- body ----+
|   u32    |    u32   |   u32   |  Hello world  |
+----------+----------+---------+---------------+

                FULL DECODED
+-- cmd ---+-- size --+-- opt --+----- body ----+
|   u32    |    u32   |   u32   |  Hello world  |
+----------+----------+---------+---------------+
*/
#[allow(dead_code)]
fn full_codec() -> LengthDelimitedCodec {
    LengthDelimitedCodec::builder()
        .length_field_offset(field_offset!(Header::size))
        .length_field_length(field_len!(Header::size))
        .length_adjustment(0 - (field_len!(Header::cmd) + field_len!(Header::size)) as isize)
        .num_skip(0)
        .native_endian()
        .new_codec()
}

pub struct SelfFrameCodec {}
const MAX_FRAME_LENGTH: usize = 1024 * 1024 * 1024;

impl Decoder for SelfFrameCodec {
    type Item = Vec<u8>;
    type Error = std::io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < header_len() {
            return Ok(None);
        }

        let header = Header::from_bytes(&src[..header_len()])?;
        if header.size > MAX_FRAME_LENGTH as u32 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("frame length {} is too large.", header.size),
            ));
        }
        if header.size > src.len() as u32 {
            return Ok(None);
        }

        let mut ret = vec![0u8; header.size as usize];
        src.copy_to_slice(&mut ret);

        Ok(Some(ret))
    }
}

/*
impl Encoder<Bytes> for SelfFrameCodec {
    type Error = std::io::Error;
    fn encode(&mut self, item: Bytes, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if item.len() > MAX_FRAME_LENGTH {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("frame length {} is too large.", item.len()),
            ));
        }
        dst.reserve(item.len());
        dst.put(item);
        Ok(())
    }
}

impl Encoder<BytesMut> for SelfFrameCodec {
    type Error = std::io::Error;
    fn encode(&mut self, item: BytesMut, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if item.len() > MAX_FRAME_LENGTH {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("frame length {} is too large.", item.len()),
            ));
        }
        dst.reserve(item.len());
        dst.put(item);
        Ok(())
    }
}

*/
impl<T> Encoder<T> for SelfFrameCodec
where
    T: Buf,
{
    type Error = std::io::Error;
    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if item.remaining() > MAX_FRAME_LENGTH {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("frame length {} is too large.", item.remaining()),
            ));
        }
        dst.reserve(item.remaining());
        dst.put(item);
        Ok(())
    }
}

pub fn bind_transport<T: AsyncRead + AsyncWrite>(io: T) -> Framed<T, SelfFrameCodec> {
    Framed::new(io, SelfFrameCodec{})
}


#[cfg(test)]
mod tests {
    #[test]
    fn bufreader() {
        use bytes::BytesMut;

        let b = BytesMut::from(&b"hello world"[..]);

        assert_eq!(&b[..5], b"hello");
        assert_eq!(&b[..], b"hello world");
        assert_eq!(b.len(), 11);
    }
}
