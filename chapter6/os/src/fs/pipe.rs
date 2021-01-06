use super::*;
use alloc::collections::VecDeque;


/// 控制台键盘输入，实现 [`INode`] 接口
pub struct Pipe {
    /// 从后插入，前段弹出
    status: Status,
    buffer: Arc<Mutex<VecDeque<u8>>>,
}

#[derive(Copy, Clone, PartialEq)]
enum Status {
    Write,
    Read,
}

impl INode for Pipe {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        if offset != 0 || self.status == Status::Write {
            // 不支持 offset
            Err(FsError::NotSupported)
        } else if self.buffer.lock().len() == 0 {
            // 缓冲区没有数据，将当前线程休眠
            Ok(0)
        } else {
            let mut stdin_buffer = self.buffer.lock();
            for (i, byte) in buf.iter_mut().enumerate() {
                if let Some(b) = stdin_buffer.pop_front() {
                    *byte = b;
                } else {
                    return Ok(i);
                }
            }
            Ok(buf.len())
        }
    }

    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        if offset != 0 || self.status == Status::Read {
            Err(FsError::NotSupported)
        } else {
            let mut data = self.buffer.lock();
            for c in buf {
                data.push_back(*c);
            }
            Ok(buf.len())
        }
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

impl Pipe {
    pub fn make_pipe() -> (Pipe, Pipe) {
        let buffer: VecDeque<u8> = VecDeque::new();
        let data = Arc::new(Mutex::new(buffer));

        (
            Pipe {
                buffer: data.clone(),
                status: Status::Read,
            },
            Pipe {
                buffer: data.clone(),
                status: Status::Write,
            },
        )
    }
}